use petgraph::graph::{NodeIndex, UnGraph};

fn main() {
    // Create a graph with multiple interconnected triangle subgraphs.
    let graph = create_graph();

    // Customize settings using the SettingsBuilder. Values which are not set will use defaults.
    let settings = visgraph::settings::SettingsBuilder::new()
        .width(2000.0)
        .height(2000.0)
        .node_radius(10.0)
        .font_size(10.0)
        .stroke_width(1.0)
        .layout(visgraph::Layout::ForceDirected)
        .build()
        .expect("Values should be valid.");

    // Generate and save the graph image using our settings.
    visgraph::graph_to_img(
        &graph,
        &settings,
        "examples/results/force_directed_layout.png",
    )
    .unwrap();
}

fn create_graph() -> UnGraph<(), ()> {
    let mut graph = UnGraph::new_undirected();
    let mut subgraph_roots = Vec::with_capacity(9);
    for _ in 0..9 {
        subgraph_roots.push(create_triangle_subgraph(&mut graph));
    }

    // Connect the subgraphs
    // Connect the first row of subgraphs
    for i in 0..4 {
        graph.add_edge(subgraph_roots[i].1, subgraph_roots[i + 1].0, ());
    }
    // Connect first with second row
    for i in 0..2 {
        graph.add_edge(subgraph_roots[i * 2].2, subgraph_roots[i + 4].0, ());
        graph.add_edge(subgraph_roots[i * 2 + 1].2, subgraph_roots[i + 4].1, ());
    }
    // Connect second with third row
    graph.add_edge(subgraph_roots[4].2, subgraph_roots[6].0, ());
    graph.add_edge(subgraph_roots[5].2, subgraph_roots[7].1, ());

    // Connect the third row of subgraphs
    graph.add_edge(subgraph_roots[6].1, subgraph_roots[7].0, ());

    // Connect the third row with the forth and last
    graph.add_edge(subgraph_roots[6].2, subgraph_roots[8].0, ());
    graph.add_edge(subgraph_roots[7].2, subgraph_roots[8].1, ());

    graph
}

fn create_triangle_subgraph(graph: &mut UnGraph<(), ()>) -> (NodeIndex, NodeIndex, NodeIndex) {
    // the subgraph will look as follows:
    // 0 - 1 - 2 - 3 - 4
    //  \ / \ /  \ / \ /
    //   5 - 6   7 - 8
    //    \ /     \ /
    //     9 - 10 -11
    //      \ /  \ /
    //       12 - 13
    //        \   /
    //         14
    let mut nodes = Vec::with_capacity(15);
    for _ in 0..15 {
        nodes.push(graph.add_node(()));
    }
    // Create the edges for the subgraph
    for i in 0..4 {
        graph.add_edge(nodes[i], nodes[i + 1], ());
    }
    for i in 0..8 {
        if i % 2 == 0 {
            graph.add_edge(nodes[(i + 1) / 2], nodes[(i + 1) / 2 + 5], ());
        } else {
            graph.add_edge(nodes[(i + 1) / 2], nodes[(i + 1) / 2 + 4], ());
        }
    }
    graph.add_edge(nodes[5], nodes[6], ());
    graph.add_edge(nodes[7], nodes[8], ());

    graph.add_edge(nodes[5], nodes[9], ());
    graph.add_edge(nodes[6], nodes[10], ());
    graph.add_edge(nodes[7], nodes[11], ());
    graph.add_edge(nodes[8], nodes[11], ());

    graph.add_edge(nodes[9], nodes[10], ());
    graph.add_edge(nodes[10], nodes[11], ());

    graph.add_edge(nodes[9], nodes[12], ());
    graph.add_edge(nodes[10], nodes[12], ());
    graph.add_edge(nodes[10], nodes[13], ());
    graph.add_edge(nodes[11], nodes[13], ());

    graph.add_edge(nodes[12], nodes[13], ());

    graph.add_edge(nodes[12], nodes[14], ());
    graph.add_edge(nodes[13], nodes[14], ());

    (nodes[0], nodes[4], nodes[14])
}
