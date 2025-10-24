use petgraph::Graph;
use petgraph::graph::UnGraph;
use visgraph::settings::SettingsBuilder;
use visgraph::{graph_to_img, Layout, Orientation};

fn main() {
    // Create a complete graph with 100 nodes.
    let mut graph = UnGraph::new_undirected();
    let num_nodes = 5;
    let nodes: Vec<_> = (0..num_nodes)
        .map(|_| graph.add_node(()))
        .collect();

    for i in 0..num_nodes {
        for j in (i + 1)..num_nodes {
            graph.add_edge(nodes[i], nodes[j], ());
        }
    }

    let settings = SettingsBuilder::new()
        .layout(Layout::Hierarchical(Orientation::RightToLeft))
        .build()
        .expect("Values should be valid.");

    graph_to_img(
        &graph,
        &settings,
        "examples/results/graph_with_hierarchical_layout.png",
    )
    .unwrap();
}
