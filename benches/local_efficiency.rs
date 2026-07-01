use criterion::{Criterion, criterion_group, criterion_main};
use rsomics_local_efficiency::{local_efficiency, parse_edge_list_ordered};
use std::process::Command;

fn gnm_edge_list(n: usize, m: usize, seed: u64) -> String {
    let script = format!(
        r#"import networkx as nx
G = nx.gnm_random_graph({n}, {m}, seed={seed})
for u,v in G.edges():
    print(f"{{u}} {{v}}")
"#
    );
    let out = Command::new("/opt/homebrew/Caskroom/miniforge/base/envs/scanpy/bin/python3")
        .arg("-c")
        .arg(&script)
        .output()
        .expect("python3");
    String::from_utf8(out.stdout).unwrap()
}

fn bench_local_efficiency(c: &mut Criterion) {
    // ~300-node dense graph
    let edges = gnm_edge_list(300, 8000, 99);
    let (n, adj) = parse_edge_list_ordered(&edges);

    c.bench_function("local_efficiency_300n_8000e", |b| {
        b.iter(|| {
            let v = local_efficiency(n, &adj);
            std::hint::black_box(v);
        });
    });

    // ~500-node graph
    let edges_500 = gnm_edge_list(500, 15000, 77);
    let (n2, adj2) = parse_edge_list_ordered(&edges_500);

    c.bench_function("local_efficiency_500n_15000e", |b| {
        b.iter(|| {
            let v = local_efficiency(n2, &adj2);
            std::hint::black_box(v);
        });
    });
}

criterion_group!(benches, bench_local_efficiency);
criterion_main!(benches);
