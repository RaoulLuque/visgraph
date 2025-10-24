use petgraph::graph::UnGraph;
use visgraph::{graph_to_svg::graph_to_svg, settings::Settings};

fn main() {
    // Create a complete graph with 4 nodes.
    let mut complete_graph = UnGraph::new_undirected();
    let num_nodes = 4;
    let nodes: Vec<_> = (0..num_nodes)
        .map(|_| complete_graph.add_node(()))
        .collect();

    for i in 0..num_nodes {
        for j in (i + 1)..num_nodes {
            complete_graph.add_edge(nodes[i], nodes[j], ());
        }
    }

    // Generate and save the graph image using default settings.
    graph_to_svg(
        &complete_graph,
        &Settings::default(),
        "examples/results/graph_to_svg.svg",
    )
    .unwrap();
}
