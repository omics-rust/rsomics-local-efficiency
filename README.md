# rsomics-local-efficiency

Compute the **local efficiency** of an undirected graph — a value-exact port of
`networkx.local_efficiency`.

## Usage

```
rsomics-local-efficiency [--json] < edges.txt
```

Input: one edge per line as `u v` (string node labels). Lines starting with `#`
or blank lines are skipped. Parallel edges are deduplicated. Self-loops are kept
(networkx stores `v` in its own adjacency, so `v` joins its neighbour set).
Output: the scalar local efficiency printed to 17 significant digits.

```
$ echo -e "0 1\n0 2\n0 3\n1 2\n1 3" | rsomics-local-efficiency
9.16666666666666741e-1
```

## Algorithm

`local_efficiency(G) = (Σ_v global_efficiency(G[N(v)])) / |V|`

For each node `v`, the neighbour-induced subgraph `G[N(v)]` is formed from the
set of `v`'s neighbours — which includes `v` itself when `v` carries a
self-loop, matching networkx `G[v]`. Its global efficiency is:

`global_efficiency(H) = (Σ_{i≠j} 1/d(i,j)) / (|V|(|V|-1))`

where `d(i,j)` is the BFS hop-distance within `H`. Nodes with fewer than 2
neighbours contribute 0. Summation follows networkx insertion order.

Ref: Latora & Marchiori, PRL 87(19):198701 (2001).
doi:[10.1103/PhysRevLett.87.198701](https://doi.org/10.1103/PhysRevLett.87.198701)

## Performance

All benchmarks on Apple M2 (single core). Fixture: `nx.gnm_random_graph` with seed
fixed for reproducibility.

| Graph | Rust | networkx 3.6.1 | Ratio |
|---|---|---|---|
| 300 nodes, 8 000 edges | 73 ms | 4 079 ms | **56×** |
| 500 nodes, 15 000 edges | 159 ms | 12 736 ms | **80×** |

Implementation: integer-indexed adjacency (`Vec<Vec<usize>>`); BFS per-source
within the neighbour subgraph using a `u32` distance array and a `VecDeque`
frontier. No `HashMap` in the hot loop.

## Install

```
cargo install rsomics-local-efficiency
```

## Origin

This crate is an independent Rust reimplementation of `networkx.local_efficiency`
based on:

- The NetworkX source (`networkx/algorithms/efficiency_measures.py`, BSD-3-Clause)
- The published method: Latora & Marchiori (2001), doi:10.1103/PhysRevLett.87.198701

NetworkX is MIT/BSD-3-Clause; its source was read to replicate iteration order
exactly. No GPL code was involved. Golden test values are derived directly from
`nx.local_efficiency` calls (networkx 3.6.1).

License: MIT OR Apache-2.0.
Upstream credit: [NetworkX](https://networkx.org/) (BSD-3-Clause).
