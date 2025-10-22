use petgraph::graph::UnGraph;
use visgraph::graph_to_img;
use visgraph::settings::SettingsBuilder;

fn main() {
    // Create a square graph with four nodes
    // It should look like this:
    // A --- B
    // |     |
    // D --- C
    let mut square_graph = UnGraph::new_undirected();
    let node_a = square_graph.add_node(());
    let node_b = square_graph.add_node(());
    let node_c = square_graph.add_node(());
    let node_d = square_graph.add_node(());

    square_graph.add_edge(node_a, node_b, ());
    square_graph.add_edge(node_b, node_c, ());
    square_graph.add_edge(node_c, node_d, ());
    square_graph.add_edge(node_d, node_a, ());

    // Positions should be between (0.0) and (1.0)
    let position_map = |node_id| match node_id {
        id if id == node_a => (0.25, 0.25),
        id if id == node_b => (0.75, 0.25),
        id if id == node_c => (0.75, 0.75),
        id if id == node_d => (0.25, 0.75),
        _ => (0.5, 0.5),
    };

    // Customize settings using the SettingsBuilder. Values which are not set will use defaults.
    let settings = SettingsBuilder::new()
        .width(500.0)
        .height(500.0)
        .position_map(position_map)
        .build()
        .expect("Values should be valid.");

    // Generate and save the graph image the custom position map.
    graph_to_img(
        &square_graph,
        &settings,
        "examples/results/square_graph_with_position_map.png",
    )
    .unwrap();
}
