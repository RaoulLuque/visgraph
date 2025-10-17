use visgraph::settings::Settings;

fn main() {
    let mut graph = petgraph::Graph::<&str, &str>::new();
    let node_a = graph.add_node("ABCDEDF");
    let node_b = graph.add_node("B");
    let node_c = graph.add_node("C");
    let node_d = graph.add_node("D");

    graph.add_edge(node_a, node_b, "edge1");
    graph.add_edge(node_b, node_c, "edge2");
    graph.add_edge(node_c, node_d, "edge3");

    let svg_data = visgraph::graph_to_svg_with_layout(
        &graph,
        visgraph::Layout::Circular,
        |node_id| graph.node_weight(node_id).unwrap().to_string(),
        Settings::default(),
    );

    visgraph::parse_svg_to_img(&svg_data, Settings::default(), "graph.png").unwrap();
}
