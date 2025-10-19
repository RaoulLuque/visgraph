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
        id if id == node_a => (0.25, 0.25),
        id if id == node_b => (0.75, 0.25),
        id if id == node_c => (0.75, 0.75),
        id if id == node_d => (0.25, 0.75),
        _ => (0.5, 0.5),
    };

    // Customize settings using the SettingsBuilder.
    let settings = SettingsBuilder::new()
        .width(3000.0)
        .height(3000.0)
        .build()
        .expect("Values should be valid.");

    // Generate and save the graph image using a circular layout.
    graph_to_img_with_position_map(
        &complete_graph,
        position_map,
        &settings,
        "examples/results/square_graph_with_position_map.png",
    )
    .unwrap();
}
