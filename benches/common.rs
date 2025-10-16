use petgraph::{Undirected, graph::{Graph, NodeIndex}};

pub fn build_2d_grid(width: usize, height: usize) -> Graph<(usize, usize), usize, Undirected> {
    let mut g = Graph::new_undirected();

    for j in 0..height {
        for i in 0..width {
            let node = g.add_node((i, j));

            if j > 0 {
                let up = NodeIndex::new(node.index() - width);
                g.add_edge(node, up, 1);
            }
            if i > 0 {
                let right = NodeIndex::new(node.index() - 1);
                g.add_edge(node, right, 1);
            }
        }
    }

    g
}
