use petgraph::graph::UnGraph;
use visgraph::settings::SettingsBuilder;
use visgraph::{graph_to_img_with_layout, graph_to_img_with_position_map};

fn main() {
    // Create a square graph with four nodes
    // It should look like this:
    // A --- B
    // |     |
    // D --- C
    let mut complete_graph = UnGraph::new_undirected();
    let node_a = complete_graph.add_node(());
    let node_b = complete_graph.add_node(());
    let node_c = complete_graph.add_node(());
    let node_d = complete_graph.add_node(());

    complete_graph.add_edge(node_a, node_b, ());
    complete_graph.add_edge(node_b, node_c, ());
    complete_graph.add_edge(node_c, node_d, ());
    complete_graph.add_edge(node_d, node_a, ());

    // Positions should be between (0.0) and (1.0)
    let position_map = |node_id| match node_id {
        node_a => (0.0, 0.0),
        node_b => (1.0, 0.0),
        node_c => (1.0, 1.0),
        node_d => (0.0, 1.0),
        _ => unreachable!(),
    };

    // Customize settings using the SettingsBuilder.
    let settings = SettingsBuilder::new()
        .width(1000.0)
        .height(1000.0)
        .radius(5.0)
        .stroke_width(2.0)
        .build()
        .expect("Values should be valid.");

    // Generate and save the graph image using a circular layout.
    graph_to_img_with_position_map(
        &complete_graph,
        position_map,
        &settings,
        "target/visualizations/graph.png",
    )
    .unwrap();
}
