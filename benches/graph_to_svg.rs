mod common;
use common::build_2d_grid;
use criterion::{criterion_group, criterion_main, Criterion};

fn graph_to_svg_grid_25(c: &mut Criterion) {
    let graph = build_2d_grid(25, 25);

    c.bench_function("graph_to_svg_grid_25", |b| {
        b.iter(|| {
            let position_map = |node_id| {
                let (x, y) = *graph.node_weight(node_id).unwrap();
                (x as f32, y as f32)
            };
            let settings = visgraph::settings::SettingsBuilder::new()
                .position_map(position_map)
                .build()
                .unwrap();
            visgraph::graph_to_svg::graph_to_svg_string(&graph, &settings)
        });
    });
}

fn graph_to_svg_grid_100(c: &mut Criterion) {
    let graph = build_2d_grid(100, 100);

    c.bench_function("graph_to_svg_grid_100", |b| {
        b.iter(|| {
            let position_map = |node_id| {
                let (x, y) = *graph.node_weight(node_id).unwrap();
                (x as f32, y as f32)
            };
            let settings = visgraph::settings::SettingsBuilder::new()
                .position_map(position_map)
                .build()
                .unwrap();
            visgraph::graph_to_svg::graph_to_svg_string(&graph, &settings)
        });
    });
}

fn graph_to_svg_grid_250(c: &mut Criterion) {
    let graph = build_2d_grid(250, 250);

    c.bench_function("graph_to_svg_grid_250", |b| {
        b.iter(|| {
            let position_map = |node_id| {
                let (x, y) = *graph.node_weight(node_id).unwrap();
                (x as f32, y as f32)
            };
            let settings = visgraph::settings::SettingsBuilder::new()
                .position_map(position_map)
                .build()
                .unwrap();
            visgraph::graph_to_svg::graph_to_svg_string(&graph, &settings)
        });
    });
}

criterion_group!(
    benches,
    graph_to_svg_grid_25,
    graph_to_svg_grid_100,
    graph_to_svg_grid_250
);
criterion_main!(benches);
