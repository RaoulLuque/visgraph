fn main() {
    let mut graph = petgraph::Graph::<&str, &str>::new();
    let node_a = graph.add_node("A");
    let node_b = graph.add_node("B");
    graph.add_edge(node_a, node_b, "edge1");

    let svg_data = visgraph::graph_to_svg(
        &graph,
        |node_id| match node_id.index() {
            0 => (100.0, 100.0),
            1 => (200.0, 100.0),
            _ => (0.0, 0.0),
        },
        |node_id| graph.node_weight(node_id).unwrap().to_string(),
    );

    visgraph::parse_svg_to_img(&svg_data, 1000.0, 1000.0, "graph.png").unwrap();
}
