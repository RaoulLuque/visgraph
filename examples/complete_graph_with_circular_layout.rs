use petgraph::graph::UnGraph;
use visgraph::graph_to_img_with_layout;
use visgraph::settings::SettingsBuilder;

fn main() {
    // Create a complete graph with 100 nodes.
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

    // Customize settings using the SettingsBuilder.
    let settings = SettingsBuilder::new()
        .width(1000.0)
        .height(1000.0)
        .radius(5.0)
        .stroke_width(2.0)
        .build()
        .expect("Values should be valid.");

    // Generate and save the graph image using a circular layout.
    graph_to_img_with_layout(
        &complete_graph,
        visgraph::Layout::Circular,
        &settings,
        "target/visualizations/graph.png",
    )
    .unwrap();
}
