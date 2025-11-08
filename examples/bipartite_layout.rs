use petgraph::graph::{NodeIndex, UnGraph};
use visgraph::{graph_to_img, settings::SettingsBuilder, Layout};

fn main() {
    let mut graph = UnGraph::new_undirected();
    let nodes: Vec<NodeIndex> = (0..10).map(|_| graph.add_node(())).collect();
    for i in 0..10 {
        graph.add_edge(nodes[i], nodes[(i + 1) % 10], ());
    }
    graph.add_edge(nodes[0], nodes[5], ());
    graph.add_edge(nodes[2], nodes[7], ());
    graph.add_edge(nodes[3], nodes[8], ());

    // Create settings with bipartite layout
    let settings = SettingsBuilder::new()
        .width(1000.0)
        .height(1000.0)
        .node_radius(30.0)
        .font_size(20.0)
        // Set the bipartite layout without predefined partitions
        .layout(Layout::Bipartite(None))
        .build()
        .expect("Values should be valid.");

    // Generate and save the graph image using our settings.
    graph_to_img(&graph, &settings, "examples/results/bipartite_layout.png").unwrap();
}
