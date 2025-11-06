use petgraph::Graph;
use visgraph::{graph_to_img, settings::SettingsBuilder, Layout, Orientation};

fn main() {
    let mut graph = Graph::new();

    let a = graph.add_node(());
    let b = graph.add_node(());
    let c = graph.add_node(());
    let d = graph.add_node(());
    let e = graph.add_node(());

    graph.add_edge(a, b, ());
    graph.add_edge(b, c, ());
    graph.add_edge(b, d, ());
    graph.add_edge(d, e, ());

    let settings = SettingsBuilder::new()
        // Use hierarchical layout
        .layout(Layout::Hierarchical(Orientation::default()))
        .build()
        .expect("Values should be valid.");

    graph_to_img(
        &graph,
        &settings,
        "examples/results/hierarchical_layout.png",
    )
    .unwrap();
}
