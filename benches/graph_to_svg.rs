mod common;
use common::build_2d_grid;

use criterion::{criterion_group, criterion_main, Criterion};

fn graph_to_svg_benchmark(c: &mut Criterion) {
    let graph = build_2d_grid(100, 100);

    c.bench_function("graph_to_svg", |b| {
        b.iter(|| {
            let position_map = |node_id| {
                let (x, y) = *graph.node_weight(node_id).unwrap();
                (x as f32, y as f32)
            };
            let settings = visgraph::settings::SettingsBuilder::new()
                .layout(visgraph::Layout::PositionMap(position_map))
                .build()
                .unwrap();
            visgraph::graph_to_svg::graph_to_svg(&graph, &settings)
        })
    });
}

criterion_group!(benches, graph_to_svg_benchmark);
criterion_main!(benches);
