use petgraph::graph::{EdgeIndex, UnGraph};
use visgraph::settings::SettingsBuilder;
use visgraph::{graph_to_img, Layout};

const RAINBOW_COLORS: [&str; 7] = [
    "RED", "ORANGE", "YELLOW", "GREEN", "BLUE", "INDIGO", "VIOLET",
];

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

    let edge_coloring_fn = |edge_id: EdgeIndex| {
        let index = edge_id.index() % RAINBOW_COLORS.len();
        RAINBOW_COLORS[index].to_string()
    };

    // Customize settings using the SettingsBuilder. Values which are not set will use defaults.
    let settings = SettingsBuilder::new()
        .width(1000.0)
        .height(1000.0)
        .node_radius(7.5)
        .stroke_width(0.1)
        .font_size(7.5)
        .layout(Layout::Circular)
        .edge_coloring_fn(edge_coloring_fn)
        .build()
        .expect("Values should be valid.");

    // Generate and save the graph image using a circular layout.
    graph_to_img(
        &complete_graph,
        &settings,
        "examples/results/complete_graph_with_circular_layout.png",
    )
    .unwrap();
}
