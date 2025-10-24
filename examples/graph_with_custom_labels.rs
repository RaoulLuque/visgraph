use petgraph::graph::UnGraph;
use visgraph::{graph_to_img, settings::SettingsBuilder};

fn main() {
    let mut graph = UnGraph::new_undirected();
    let node_a = graph.add_node("Ljubljana".to_string());
    let node_b = graph.add_node("Bielefeld".to_string());
    let node_c = graph.add_node("Cape Town".to_string());
    let node_d = graph.add_node("Lima".to_string());

    graph.add_edge(node_a, node_b, ());
    graph.add_edge(node_b, node_c, ());
    graph.add_edge(node_c, node_d, ());
    graph.add_edge(node_d, node_a, ());

    // Define custom node and edge label functions. They should impl Fn(NodeIndex) -> String and
    // Fn(EdgeIndex) -> String, respectively.
    let node_labels = |node_id| graph.node_weight(node_id).unwrap().to_owned();
    let edge_labels = |_| "An edge".to_string();

    // Customize settings using the SettingsBuilder. Values which are not set will use defaults.
    let settings = SettingsBuilder::new()
        .width(1000.0)
        .height(1000.0)
        .node_radius(50.0)
        .margin_x(0.1)
        .margin_y(0.1)
        .node_label_fn(node_labels)
        .edge_label_fn(edge_labels)
        .build()
        .expect("Values should be valid.");

    // Generate and save the graph image using a circular layout and the settings with the custom
    // node and edge labels.
    graph_to_img(
        &graph,
        &settings,
        "examples/results/graph_with_custom_labels.png",
    )
    .unwrap();
}
