mod common;
use common::build_2d_grid;

use criterion::{criterion_group, criterion_main, Criterion};
use visgraph;

fn graph_to_svg_benchmark(c: &mut Criterion) {
    let graph = build_2d_grid(100, 100);

    c.bench_function("graph_to_svg", |b| {
        b.iter(|| {
            visgraph::graph_to_svg_with_positions(
                &graph,
                |node_id| {
                    let (x, y) = *graph.node_weight(node_id).unwrap();
                    (x as f32, y as f32)
                },
                |node_id| format!("{:?}", graph.node_weight(node_id).unwrap()),
                visgraph::settings::Settings::default(),
            )
        })
    });
}

criterion_group!(benches, graph_to_svg_benchmark);
criterion_main!(benches);
