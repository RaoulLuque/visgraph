//! Module containing different layout algorithms for graph visualization.
//!
//! The [Layout] enum is the main entry point to select a layout algorithm using the
//! [`SettingsBuilder`][crate::settings::SettingsBuilder].
//!
//! The layout algorithms can also be called directly from their respective
//! submodules of this module.

use crate::layout::hierarchical::Orientation;

pub(crate) type DefaultPositionMapFn = fn(petgraph::prelude::NodeIndex) -> (f32, f32);

pub mod force_directed;
pub mod hierarchical;

/// Different layout algorithms for graph visualization.
///
/// The layout algorithms can also be called directly from their respective submodules of the
/// [`layout`](crate::layout) module.
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
    /// A layout algorithm.
    Layout(Layout),
    /// A custom position map function.
    PositionMap(PositionMapFn),
}

pub mod circular {
    //! Module containing functionality for the circular layout.
    use petgraph::visit::{IntoNodeReferences, NodeIndexable};

    /// Returns a position map function that arranges nodes in a circular layout.
    ///
    /// The nodes are evenly distributed around a unit circle centered at (0.5, 0.5). That is
    /// all values are in the range [0.0, 1.0].
    pub fn circular_layout<G>(graph: &G) -> impl Fn(G::NodeId) -> (f32, f32) + '_
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
    //! Module containing functionality for the random layout.

    use petgraph::visit::{IntoNodeReferences, NodeIndexable, NodeRef};

    /// Returns a position map function that assigns random positions to nodes.
    ///
    /// The returned position map is normalized to [0.0, 1.0].
    pub fn random_layout<G>(graph: &G) -> impl Fn(G::NodeId) -> (f32, f32) + '_
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
