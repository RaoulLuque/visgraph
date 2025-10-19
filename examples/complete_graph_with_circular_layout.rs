use petgraph::graph::UnGraph;
use visgraph::graph_to_img_with_layout;
use visgraph::settings::SettingsBuilder;

fn main() {
    let mut complete_graph = UnGraph::new_undirected();
    let num_nodes = 100;
    let nodes: Vec<_> = (0..num_nodes)
        .map(|_| complete_graph.add_node(()))
        .collect();

    for i in 0..num_nodes {
        for j in (i + 1)..num_nodes {
            complete_graph.add_edge(nodes[i], nodes[j], ());
        }
    }
    let settings = SettingsBuilder::new()
        .width(1000.0)
        .height(1000.0)
        .radius(5.0)
        .stroke_width(2.0)
        .build()
        .expect("Values should be valid.");
    graph_to_img_with_layout(
        &complete_graph,
        visgraph::Layout::Circular,
        // We don't provide a custom label function, so node indices will be used.
        &settings,
        "target/visualizations/graph.png",
    )
    .unwrap();
}
