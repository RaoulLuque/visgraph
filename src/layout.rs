use petgraph::visit::NodeRef;

/// Different layout algorithms for graph visualization.
///
/// For examples, see the `examples/` directory.
#[derive(Debug, Clone, Copy)]
pub enum Layout {
    /// Nodes are arranged in a [circular layout](https://en.wikipedia.org/wiki/Circular_layout).
    Circular,
    /// Nodes are arranged in a hierarchical layout.
    Hierarchical,
}

pub(crate) fn get_circular_position_map<G>(graph: &G) -> impl Fn(G::NodeId) -> (f32, f32) + '_
where
    G: petgraph::visit::IntoNodeReferences + petgraph::visit::NodeIndexable,
{
    let node_count = graph.node_references().count() as f32;
    move |node_id| {
        let index = graph.to_index(node_id) as f32;
        let angle = index / node_count * std::f32::consts::TAU;
        let x = 0.5 + 0.5 * angle.cos();
        let y = 0.5 + 0.5 * angle.sin();
        (x, y)
    }
}

// TODO: Implement proper hierarchical layout algorithm.
pub(crate) fn get_hierarchical_position_map<G>(graph: &G) -> impl Fn(G::NodeId) -> (f32, f32) + '_
where
    G: petgraph::visit::IntoNodeReferences + petgraph::visit::NodeIndexable,
{
    let mut levels: Vec<Vec<G::NodeId>> = Vec::new();
    for node in graph.node_references() {
        let level = graph.to_index(node.id()) / 2;
        if levels.len() <= level {
            levels.push(Vec::new());
        }
        levels[level].push(node.id());
    }
    move |node_id| {
        let index = graph.to_index(node_id);
        let level = index / 2;
        let position_in_level = levels[level].iter().position(|&id| id == node_id).unwrap();
        let nodes_in_level = levels[level].len();
        let x = 50.0 + (position_in_level as f32) * (200.0 / (nodes_in_level as f32));
        let y = 50.0 + (level as f32) * 100.0;
        (x, y)
    }
}
