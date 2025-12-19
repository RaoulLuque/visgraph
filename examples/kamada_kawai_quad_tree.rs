use petgraph::graph::UnGraph;

#[cfg(feature = "img")]
use visgraph::graph_to_img;

use visgraph::{settings::SettingsBuilder, Layout};

fn main() {
    #[allow(unused_variables)]
    let graph = create_quad_tree();

    #[allow(unused_variables)]
    let settings = SettingsBuilder::new()
        .width(2000.0)
        .height(2000.0)
        .node_radius(8.0)
        .font_size(8.0)
        .stroke_width(1.0)
        .layout(Layout::KamadaKawai)
        .build()
        .expect("Values should be valid.");

    #[cfg(feature = "img")]
    {
        graph_to_img(
            &graph,
            &settings,
            "examples/results/kamada_kawai_quad_tree.png",
        )
        .unwrap();
        println!("Generated examples/results/kamada_kawai_quad_tree.png");
    }

    #[cfg(not(feature = "img"))]
    {
        println!("Note: Compile with --features img to generate PNG output");
    }
}

fn create_quad_tree() -> UnGraph<(), ()> {
    let mut graph = UnGraph::new_undirected();
    let mut nodes = Vec::new();

    for _ in 0..100 {
        nodes.push(graph.add_node(()));
    }

    graph.add_edge(nodes[0], nodes[1], ());
    graph.add_edge(nodes[0], nodes[2], ());
    graph.add_edge(nodes[0], nodes[3], ());
    graph.add_edge(nodes[0], nodes[4], ());

    graph.add_edge(nodes[1], nodes[5], ());

    graph.add_edge(nodes[2], nodes[6], ());
    graph.add_edge(nodes[2], nodes[7], ());

    graph.add_edge(nodes[3], nodes[8], ());
    graph.add_edge(nodes[3], nodes[9], ());

    graph.add_edge(nodes[4], nodes[10], ());
    graph.add_edge(nodes[4], nodes[11], ());

    graph.add_edge(nodes[5], nodes[12], ());
    graph.add_edge(nodes[5], nodes[13], ());
    graph.add_edge(nodes[5], nodes[14], ());
    graph.add_edge(nodes[5], nodes[15], ());

    graph.add_edge(nodes[6], nodes[16], ());
    graph.add_edge(nodes[6], nodes[17], ());
    graph.add_edge(nodes[6], nodes[18], ());
    graph.add_edge(nodes[6], nodes[19], ());

    graph.add_edge(nodes[7], nodes[20], ());
    graph.add_edge(nodes[7], nodes[21], ());
    graph.add_edge(nodes[7], nodes[22], ());

    graph.add_edge(nodes[8], nodes[23], ());
    graph.add_edge(nodes[8], nodes[24], ());
    graph.add_edge(nodes[8], nodes[25], ());

    graph.add_edge(nodes[9], nodes[26], ());

    graph.add_edge(nodes[10], nodes[27], ());

    graph.add_edge(nodes[11], nodes[28], ());
    graph.add_edge(nodes[11], nodes[29], ());

    graph.add_edge(nodes[12], nodes[30], ());
    graph.add_edge(nodes[12], nodes[31], ());
    graph.add_edge(nodes[12], nodes[32], ());

    graph.add_edge(nodes[13], nodes[33], ());
    graph.add_edge(nodes[13], nodes[34], ());
    graph.add_edge(nodes[13], nodes[35], ());

    graph.add_edge(nodes[14], nodes[36], ());
    graph.add_edge(nodes[14], nodes[37], ());

    graph.add_edge(nodes[15], nodes[38], ());

    graph.add_edge(nodes[16], nodes[39], ());
    graph.add_edge(nodes[16], nodes[40], ());

    graph.add_edge(nodes[17], nodes[41], ());
    graph.add_edge(nodes[17], nodes[42], ());
    graph.add_edge(nodes[17], nodes[43], ());

    graph.add_edge(nodes[18], nodes[44], ());

    graph.add_edge(nodes[19], nodes[45], ());
    graph.add_edge(nodes[19], nodes[46], ());
    graph.add_edge(nodes[19], nodes[47], ());

    graph.add_edge(nodes[20], nodes[48], ());
    graph.add_edge(nodes[20], nodes[49], ());
    graph.add_edge(nodes[20], nodes[50], ());

    graph.add_edge(nodes[21], nodes[51], ());
    graph.add_edge(nodes[21], nodes[52], ());

    graph.add_edge(nodes[22], nodes[53], ());
    graph.add_edge(nodes[22], nodes[54], ());
    graph.add_edge(nodes[22], nodes[55], ());

    graph.add_edge(nodes[23], nodes[56], ());

    graph.add_edge(nodes[24], nodes[57], ());

    graph.add_edge(nodes[25], nodes[58], ());
    graph.add_edge(nodes[25], nodes[59], ());

    graph.add_edge(nodes[26], nodes[60], ());
    graph.add_edge(nodes[26], nodes[61], ());
    graph.add_edge(nodes[26], nodes[62], ());

    graph.add_edge(nodes[27], nodes[63], ());

    graph.add_edge(nodes[28], nodes[64], ());
    graph.add_edge(nodes[28], nodes[65], ());

    graph.add_edge(nodes[29], nodes[66], ());
    graph.add_edge(nodes[29], nodes[67], ());
    graph.add_edge(nodes[29], nodes[68], ());
    graph.add_edge(nodes[29], nodes[69], ());

    graph.add_edge(nodes[30], nodes[70], ());
    graph.add_edge(nodes[30], nodes[71], ());

    graph.add_edge(nodes[31], nodes[72], ());
    graph.add_edge(nodes[31], nodes[73], ());

    graph.add_edge(nodes[33], nodes[74], ());
    graph.add_edge(nodes[34], nodes[75], ());
    graph.add_edge(nodes[35], nodes[76], ());

    graph.add_edge(nodes[36], nodes[77], ());
    graph.add_edge(nodes[37], nodes[78], ());

    graph.add_edge(nodes[39], nodes[79], ());
    graph.add_edge(nodes[40], nodes[80], ());

    graph.add_edge(nodes[41], nodes[81], ());
    graph.add_edge(nodes[42], nodes[82], ());

    graph.add_edge(nodes[44], nodes[83], ());
    graph.add_edge(nodes[44], nodes[84], ());

    graph.add_edge(nodes[48], nodes[85], ());
    graph.add_edge(nodes[49], nodes[86], ());

    graph.add_edge(nodes[51], nodes[87], ());
    graph.add_edge(nodes[52], nodes[88], ());

    graph.add_edge(nodes[53], nodes[89], ());
    graph.add_edge(nodes[54], nodes[90], ());

    graph.add_edge(nodes[60], nodes[91], ());
    graph.add_edge(nodes[61], nodes[92], ());

    graph.add_edge(nodes[64], nodes[93], ());
    graph.add_edge(nodes[65], nodes[94], ());

    graph.add_edge(nodes[66], nodes[95], ());
    graph.add_edge(nodes[67], nodes[96], ());
    graph.add_edge(nodes[68], nodes[97], ());
    graph.add_edge(nodes[69], nodes[98], ());

    graph.add_edge(nodes[70], nodes[99], ());

    graph
}
