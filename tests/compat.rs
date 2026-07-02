/// Golden values from networkx 3.6.1 (BSD-3-Clause).
/// Computed via `nx.local_efficiency(G)` on each test graph.
/// These are oracle values — never derived from our own output.
use rsomics_local_efficiency::{local_efficiency, parse_edge_list_ordered};

const EPS: f64 = 1e-12;

fn run(input: &str) -> f64 {
    let (n, adj) = parse_edge_list_ordered(input);
    local_efficiency(n, &adj)
}

fn kn_edges(n: usize) -> String {
    let mut s = String::new();
    for i in 0..n {
        for j in (i + 1)..n {
            s.push_str(&format!("{i} {j}\n"));
        }
    }
    s
}

fn cycle_edges(n: usize) -> String {
    let mut s = String::new();
    for i in 0..n {
        s.push_str(&format!("{} {}\n", i, (i + 1) % n));
    }
    s
}

fn path_edges(n: usize) -> String {
    let mut s = String::new();
    for i in 0..(n - 1) {
        s.push_str(&format!("{i} {}\n", i + 1));
    }
    s
}

#[test]
fn k4_is_one() {
    // nx: 1.0
    let v = run(&kn_edges(4));
    assert!((v - 1.0_f64).abs() < EPS, "K4 expected 1.0 got {v:.17e}");
}

#[test]
fn k5_is_one() {
    // nx: 1.0
    let v = run(&kn_edges(5));
    assert!((v - 1.0_f64).abs() < EPS, "K5 expected 1.0 got {v:.17e}");
}

#[test]
fn cycle_c4_is_zero() {
    // nx: 0.0 — neighbours of each node have no edge between them.
    let v = run(&cycle_edges(4));
    assert!(v.abs() < EPS, "C4 expected 0 got {v:.17e}");
}

#[test]
fn cycle_c5_is_zero() {
    // nx: 0.0
    let v = run(&cycle_edges(5));
    assert!(v.abs() < EPS, "C5 expected 0 got {v:.17e}");
}

#[test]
fn path_p5_is_zero() {
    // nx: 0.0 — every internal node's neighbours are non-adjacent.
    let v = run(&path_edges(5));
    assert!(v.abs() < EPS, "P5 expected 0 got {v:.17e}");
}

#[test]
fn star_s5_is_zero() {
    // nx star_graph(5): center + 5 leaves. nx: 0.0.
    // Centre's neighbours (leaves) share no edges; leaf neighbours = {centre} only → <2 nodes.
    let input = "0 1\n0 2\n0 3\n0 4\n0 5\n";
    let v = run(input);
    assert!(v.abs() < EPS, "S5 expected 0 got {v:.17e}");
}

#[test]
fn empty_graph_is_zero() {
    // nx single-node or empty graph: 0.0.
    let (n, adj) = parse_edge_list_ordered("");
    let v = local_efficiency(n, &adj);
    assert!(v.abs() < EPS, "empty graph expected 0 got {v:.17e}");
}

#[test]
fn two_nodes_edge_is_zero() {
    // nx two_edge: 0.0 — each node has 1 neighbour → neighbour subgraph has 1 node.
    let v = run("0 1\n");
    assert!(v.abs() < EPS, "two-node edge expected 0 got {v:.17e}");
}

#[test]
fn docstring_example() {
    // G = nx.Graph([(0,1),(0,2),(0,3),(1,2),(1,3)])
    // nx: 9.166666666666667e-01 (repr as f64-exact value below)
    let expected = 9.166_666_666_666_667e-1_f64;
    let v = run("0 1\n0 2\n0 3\n1 2\n1 3\n");
    assert!(
        (v - expected).abs() < EPS,
        "docstring: expected {expected:.17e} got {v:.17e}, err={:.3e}",
        (v - expected).abs()
    );
}

#[test]
fn string_nodes_abcd() {
    // G: a-b, b-c, a-c, c-d → nx: 5.833333333333334e-01
    let expected = 5.833_333_333_333_334e-1_f64;
    let v = run("a b\nb c\na c\nc d\n");
    assert!(
        (v - expected).abs() < EPS,
        "string abcd: expected {expected:.17e} got {v:.17e}, err={:.3e}",
        (v - expected).abs()
    );
}

#[test]
fn comment_and_blank_lines_ignored() {
    // Same graph as docstring_example with comments and blanks interspersed.
    let expected = 9.166_666_666_666_667e-1_f64;
    let input = "\
# this is a comment
0 1
# another comment

0 2
0 3

1 2
1 3
";
    let v = run(input);
    assert!(
        (v - expected).abs() < EPS,
        "with comments: expected {expected:.17e} got {v:.17e}"
    );
}

#[test]
fn parallel_edges_deduped() {
    // Repeating edges must not alter the graph (nx.Graph deduplication).
    let expected = 9.166_666_666_666_667e-1_f64;
    let input = "0 1\n0 1\n0 2\n0 3\n1 2\n1 3\n1 3\n";
    let v = run(input);
    assert!(
        (v - expected).abs() < EPS,
        "parallel edges: expected {expected:.17e} got {v:.17e}"
    );
}

// --- Random graph regression tests (gnm, oracle: networkx 3.6.1) ---
// Clippy-truncated f64 literals; all match the stored f64 representation of the nx output.

const GNM_50_200_SEED1: f64 = 2.455_948_273_948_273_8e-1;
const GNM_50_200_SEED2: f64 = 2.429_995_430_495_430_5e-1;
const GNM_50_200_SEED3: f64 = 2.617_199_818_699_818_5e-1;
const GNM_50_200_SEED7: f64 = 2.069_084_415_584_415_7e-1;
const GNM_50_200_SEED42: f64 = 2.750_498_879_427_451e-1;
const GNM_100_500_SEED10: f64 = 1.612_561_899_237_264e-1;
const GNM_100_500_SEED20: f64 = 1.469_988_740_601_135_5e-1;

// Committed golden edge lists (frozen from networkx 3.6.1 `nx.gnm_random_graph(n, m, seed)`),
// embedded at compile time so the tests never invoke Python — they run on any CI machine.
fn gnm_edge_list(n: usize, m: usize, seed: u64) -> String {
    let s: &str = match (n, m, seed) {
        (50, 200, 1) => include_str!("golden/gnm_50_200_seed1.txt"),
        (50, 200, 2) => include_str!("golden/gnm_50_200_seed2.txt"),
        (50, 200, 3) => include_str!("golden/gnm_50_200_seed3.txt"),
        (50, 200, 7) => include_str!("golden/gnm_50_200_seed7.txt"),
        (50, 200, 42) => include_str!("golden/gnm_50_200_seed42.txt"),
        (100, 500, 10) => include_str!("golden/gnm_100_500_seed10.txt"),
        (100, 500, 20) => include_str!("golden/gnm_100_500_seed20.txt"),
        _ => panic!("no committed golden edge list for gnm({n}, {m}, {seed})"),
    };
    s.to_string()
}

#[test]
fn gnm_50_200_seed1() {
    let edges = gnm_edge_list(50, 200, 1);
    let v = run(&edges);
    let err = (v - GNM_50_200_SEED1).abs();
    assert!(
        err < EPS,
        "gnm_50_200_seed1: expected {GNM_50_200_SEED1:.17e} got {v:.17e} err={err:.3e}"
    );
}

#[test]
fn gnm_50_200_seed2() {
    let edges = gnm_edge_list(50, 200, 2);
    let v = run(&edges);
    let err = (v - GNM_50_200_SEED2).abs();
    assert!(
        err < EPS,
        "gnm_50_200_seed2: expected {GNM_50_200_SEED2:.17e} got {v:.17e} err={err:.3e}"
    );
}

#[test]
fn gnm_50_200_seed3() {
    let edges = gnm_edge_list(50, 200, 3);
    let v = run(&edges);
    let err = (v - GNM_50_200_SEED3).abs();
    assert!(
        err < EPS,
        "gnm_50_200_seed3: expected {GNM_50_200_SEED3:.17e} got {v:.17e} err={err:.3e}"
    );
}

#[test]
fn gnm_50_200_seed7() {
    let edges = gnm_edge_list(50, 200, 7);
    let v = run(&edges);
    let err = (v - GNM_50_200_SEED7).abs();
    assert!(
        err < EPS,
        "gnm_50_200_seed7: expected {GNM_50_200_SEED7:.17e} got {v:.17e} err={err:.3e}"
    );
}

#[test]
fn gnm_50_200_seed42() {
    let edges = gnm_edge_list(50, 200, 42);
    let v = run(&edges);
    let err = (v - GNM_50_200_SEED42).abs();
    assert!(
        err < EPS,
        "gnm_50_200_seed42: expected {GNM_50_200_SEED42:.17e} got {v:.17e} err={err:.3e}"
    );
}

#[test]
fn gnm_100_500_seed10() {
    let edges = gnm_edge_list(100, 500, 10);
    let v = run(&edges);
    let err = (v - GNM_100_500_SEED10).abs();
    assert!(
        err < EPS,
        "gnm_100_500_seed10: expected {GNM_100_500_SEED10:.17e} got {v:.17e} err={err:.3e}"
    );
}

#[test]
fn gnm_100_500_seed20() {
    let edges = gnm_edge_list(100, 500, 20);
    let v = run(&edges);
    let err = (v - GNM_100_500_SEED20).abs();
    assert!(
        err < EPS,
        "gnm_100_500_seed20: expected {GNM_100_500_SEED20:.17e} got {v:.17e} err={err:.3e}"
    );
}

// --- Self-loop cases (oracle: networkx 3.6.1) ---
// networkx keeps a self-loop as `v ∈ G[v]`, so the neighbour-induced subgraph
// `G.subgraph(G[v])` gains `v` as a member and its global efficiency changes.
// Each golden edge list is fed through both networkx `read_edgelist` and our
// parser; the constants below are networkx `local_efficiency` outputs.

#[test]
fn selfloop_center() {
    // 0-1,0-2,0-3,1-2 + self-loop 0-0 → v=0's subgraph gains node 0.
    let expected = 0.708_333_333_333_333_4_f64;
    let v = run(include_str!("golden/selfloop_case1.txt"));
    assert!(
        (v - expected).abs() < EPS,
        "selfloop_case1: expected {expected:.17e} got {v:.17e}"
    );
}

#[test]
fn selfloop_leaves() {
    // 0-1,0-2,0-3 + self-loops 1-1,2-2 → leaves 1,2 each get a 2-node subgraph.
    let expected = 0.5_f64;
    let v = run(include_str!("golden/selfloop_case2.txt"));
    assert!(
        (v - expected).abs() < EPS,
        "selfloop_case2: expected {expected:.17e} got {v:.17e}"
    );
}

#[test]
fn selfloop_triangle_all() {
    // Triangle with a self-loop on every node → each node's subgraph is a K3-minus.
    let expected = 1.0_f64;
    let v = run(include_str!("golden/selfloop_case3.txt"));
    assert!(
        (v - expected).abs() < EPS,
        "selfloop_case3: expected {expected:.17e} got {v:.17e}"
    );
}

#[test]
fn selfloop_dense() {
    // 0-1,0-2,0-3,1-2,1-3 + self-loop 0-0.
    let expected = 0.9375_f64;
    let v = run(include_str!("golden/selfloop_case4.txt"));
    assert!(
        (v - expected).abs() < EPS,
        "selfloop_case4: expected {expected:.17e} got {v:.17e}"
    );
}

#[test]
fn selfloop_only() {
    // Single node carrying only a self-loop → subgraph is one node → 0.
    let expected = 0.0_f64;
    let v = run(include_str!("golden/selfloop_case5.txt"));
    assert!(
        (v - expected).abs() < EPS,
        "selfloop_case5: expected {expected:.17e} got {v:.17e}"
    );
}

#[test]
fn selfloop_on_leaf() {
    // 0-1 + self-loop 1-1 → node 1's subgraph is {0,1} with edge 0-1.
    let expected = 0.5_f64;
    let v = run(include_str!("golden/selfloop_case6.txt"));
    assert!(
        (v - expected).abs() < EPS,
        "selfloop_case6: expected {expected:.17e} got {v:.17e}"
    );
}
