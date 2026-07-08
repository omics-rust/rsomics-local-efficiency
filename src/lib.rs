/// Compute local efficiency of an undirected graph.
///
/// Replicates `networkx.local_efficiency` exactly:
/// - `local_efficiency(G) = (Σ_v global_efficiency(G[N(v)])) / |V|`
/// - `global_efficiency(H) = (Σ_{i≠j} 1/d(i,j)) / (|V|(|V|-1))`
///   where d is BFS hop-distance within the subgraph
/// - Nodes with fewer than 2 neighbours contribute 0.
///
/// Ref: Latora & Marchiori, PRL 87(19):198701 (2001). doi:10.1103/PhysRevLett.87.198701
use std::collections::{HashMap, HashSet, VecDeque};

/// Parse an undirected edge list from text.
///
/// Text from the first `#` onward is stripped as a comment; blank lines are skipped
/// (nx.read_edgelist convention).
/// Parallel edges are deduplicated (nx.Graph semantics). Self-loops are kept:
/// networkx stores `v` in its own adjacency `G[v]`, so `adj[v]` includes `v`.
/// This matters because `local_efficiency` induces a subgraph on `G[v]`, and a
/// self-loop makes `v` a member of that neighbour set.
/// Node IDs are assigned in first-seen order (matching networkx insertion order).
///
/// Returns `(n, adj)` where `adj[i]` is the neighbour list of node `i` in
/// insertion order, including `i` itself when `i` carries a self-loop.
pub fn parse_edge_list_ordered(input: &str) -> (usize, Vec<Vec<usize>>) {
    let mut name_to_id: HashMap<String, usize> = HashMap::new();
    let mut next_id = 0usize;
    let mut parsed: Vec<(usize, usize)> = Vec::new();

    for line in input.lines() {
        // nx.parse_edgelist strips a '#' comment anywhere in the line before tokenising.
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split_ascii_whitespace();
        let Some(u_str) = parts.next() else { continue };
        let Some(v_str) = parts.next() else { continue };

        let u = *name_to_id.entry(u_str.to_owned()).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            id
        });
        let v = *name_to_id.entry(v_str.to_owned()).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            id
        });
        parsed.push((u, v));
    }

    let n = next_id;
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut seen: HashSet<(usize, usize)> = HashSet::with_capacity(parsed.len());

    for (u, v) in parsed {
        let key = if u <= v { (u, v) } else { (v, u) };
        if !seen.insert(key) {
            continue;
        }
        if u == v {
            adj[u].push(u);
        } else {
            adj[u].push(v);
            adj[v].push(u);
        }
    }

    (n, adj)
}

/// BFS within a subgraph defined by `local_nodes` (global IDs in local index order).
/// `local_index[global_id]` = local index, or `u32::MAX` if not in the subgraph.
/// Returns hop-distances from `src_local` to every node in the subgraph.
fn bfs_distances(
    global_adj: &[Vec<usize>],
    local_nodes: &[usize],
    local_index: &[u32],
    src_local: usize,
) -> Vec<u32> {
    let k = local_nodes.len();
    let mut dist = vec![u32::MAX; k];
    dist[src_local] = 0;
    let mut queue = VecDeque::with_capacity(k);
    queue.push_back(src_local);

    while let Some(u_loc) = queue.pop_front() {
        let d = dist[u_loc];
        let u_glob = local_nodes[u_loc];
        for &v_glob in &global_adj[u_glob] {
            let li = local_index[v_glob];
            if li == u32::MAX {
                continue;
            }
            let v_loc = li as usize;
            if dist[v_loc] == u32::MAX {
                dist[v_loc] = d + 1;
                queue.push_back(v_loc);
            }
        }
    }
    dist
}

/// Global efficiency of the subgraph induced by `local_nodes` (global IDs).
/// Matches `networkx.global_efficiency` exactly, iterating in node insertion order.
fn global_efficiency_subgraph(global_adj: &[Vec<usize>], local_nodes: &[usize]) -> f64 {
    let k = local_nodes.len();
    if k < 2 {
        return 0.0;
    }

    let global_n = global_adj.len();
    let mut local_index = vec![u32::MAX; global_n];
    for (i, &g) in local_nodes.iter().enumerate() {
        local_index[g] = i as u32;
    }

    let denom = (k * (k - 1)) as f64;
    let mut g_eff = 0.0f64;

    for src in 0..k {
        let dist = bfs_distances(global_adj, local_nodes, &local_index, src);
        for &d in &dist {
            if d > 0 && d != u32::MAX {
                g_eff += 1.0 / d as f64;
            }
        }
    }

    g_eff / denom
}

/// Compute local efficiency, matching `networkx.local_efficiency`.
pub fn local_efficiency(n: usize, adj: &[Vec<usize>]) -> f64 {
    if n == 0 {
        return 0.0;
    }

    let mut total = 0.0f64;
    for v in 0..n {
        total += global_efficiency_subgraph(adj, &adj[v]);
    }
    total / n as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inline_hash_comment_matches_clean_graph() {
        let with_comments = "0 1\n1 2#c\n2 3\n0 #x\n";
        let clean = "0 1\n1 2\n2 3\n";

        let (n_c, adj_c) = parse_edge_list_ordered(with_comments);
        let (n_r, adj_r) = parse_edge_list_ordered(clean);

        assert_eq!(n_c, n_r);
        assert_eq!(adj_c, adj_r);
        assert_eq!(local_efficiency(n_c, &adj_c), local_efficiency(n_r, &adj_r));
    }
}
