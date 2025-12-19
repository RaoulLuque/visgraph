use petgraph::graph::{NodeIndex, UnGraph};
use visgraph::{graph_to_svg, Layout};

#[cfg(feature = "img")]
use visgraph::graph_to_img;

fn main() {
    // Create a cube graph (regular hexahedron) as used in the original Kamada-Kawai paper
    // A cube has 8 vertices and 12 edges
    let mut graph = UnGraph::new_undirected();
    let nodes: Vec<NodeIndex> = (0..8).map(|_| graph.add_node(())).collect();

    // Bottom face edges (nodes 0-3)
    graph.add_edge(nodes[0], nodes[1], ());
    graph.add_edge(nodes[1], nodes[2], ());
    graph.add_edge(nodes[2], nodes[3], ());
    graph.add_edge(nodes[3], nodes[0], ());

    // Top face edges (nodes 4-7)
    graph.add_edge(nodes[4], nodes[5], ());
    graph.add_edge(nodes[5], nodes[6], ());
    graph.add_edge(nodes[6], nodes[7], ());
    graph.add_edge(nodes[7], nodes[4], ());

    // Vertical edges connecting bottom to top
    graph.add_edge(nodes[0], nodes[4], ());
    graph.add_edge(nodes[1], nodes[5], ());
    graph.add_edge(nodes[2], nodes[6], ());
    graph.add_edge(nodes[3], nodes[7], ());

    // Create settings with Kamada-Kawai layout
    let settings = visgraph::settings::SettingsBuilder::new()
        .width(1000.0)
        .height(1000.0)
        .node_radius(30.0)
        .font_size(20.0)
        .layout(Layout::KamadaKawai)
        .build()
        .expect("Values should be valid.");

    // Generate and save the graph image using our settings
    #[cfg(feature = "img")]
    {
        graph_to_img(
            &graph,
            &settings,
            "examples/results/kamada_kawai_layout.png",
        )
        .unwrap();
        println!("Generated examples/results/kamada_kawai_layout.png");
    }

    // Also generate SVG version
    graph_to_svg(
        &graph,
        &settings,
        "examples/results/kamada_kawai_layout.svg",
    )
    .unwrap();
    println!("Generated examples/results/kamada_kawai_layout.svg");
}
