use crate::layout::hierarchical::Orientation;

pub(crate) type DefaultPositionMapFn = fn(petgraph::prelude::NodeIndex) -> (f32, f32);

pub mod force_directed;
pub mod hierarchical;

/// Different layout algorithms for graph visualization.
///
/// For examples, see the [`examples`](https://github.com/RaoulLuque/visgraph/tree/main/examples) directory.
#[derive(Debug, Clone, Copy)]
pub enum Layout {
    /// Nodes are arranged in a [circular layout](https://en.wikipedia.org/wiki/Circular_layout).
    Circular,
    /// Nodes are arranged in a hierarchical layout.
    Hierarchical(Orientation),
    /// Nodes are arranged using a [force-directed layout](https://en.wikipedia.org/wiki/Force-directed_graph_drawing).
    ForceDirected,
    /// Nodes are arranged randomly.
    Random,
}

/// Enum to represent either a layout algorithm or a custom position map function. Only used for
/// [`SettingsBuilder`][crate::settings::SettingsBuilder].
#[derive(Debug, Clone, Copy)]
pub enum LayoutOrPositionMap<PositionMapFn = DefaultPositionMapFn> {
    Layout(Layout),
    PositionMap(PositionMapFn),
}

pub mod circular {
    use petgraph::visit::{IntoNodeReferences, NodeIndexable};

    pub fn get_circular_position_map<G>(graph: &G) -> impl Fn(G::NodeId) -> (f32, f32) + '_
    where
        G: IntoNodeReferences + NodeIndexable,
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
}

pub mod random {
    use petgraph::visit::{IntoNodeReferences, NodeIndexable, NodeRef};

    pub fn get_random_position_map<G>(graph: &G) -> impl Fn(G::NodeId) -> (f32, f32) + '_
    where
        G: IntoNodeReferences + NodeIndexable,
    {
        let mut rng = fastrand::Rng::new();
        let mut positions = vec![(0.0f32, 0.0f32); graph.node_bound()];
        for node_ref in graph.node_references() {
            let x = rng.f32();
            let y = rng.f32();
            let idx = graph.to_index(node_ref.id());
            positions[idx] = (x, y);
        }
        move |node_id| {
            let index = graph.to_index(node_id);
            positions[index]
        }
    }
}
