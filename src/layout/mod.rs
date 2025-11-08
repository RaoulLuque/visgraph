//! Different layout algorithms for graph visualization.
//!
//! The [Layout] enum is the main entry point to select a layout algorithm using the
//! [`SettingsBuilder`][crate::settings::SettingsBuilder].
//!
//! The layout algorithms can also be called directly from their respective
//! submodules of this module.

use std::collections::HashSet;

use petgraph::graph::NodeIndex;

use crate::layout::hierarchical::Orientation;

pub(crate) type DefaultPositionMapFn = fn(NodeIndex) -> (f32, f32);

pub mod bipartite;
pub mod force_directed;
pub mod hierarchical;

/// Different layout algorithms for graph visualization.
///
/// The layout algorithms can also be called directly from their respective submodules of the
/// [`layout`](crate::layout) module.
///
/// For examples, see the [`examples`](https://github.com/RaoulLuque/visgraph/tree/main/examples) directory.
///
/// This enum is marked as non-exhaustive to allow for adding more layout algorithms without
/// necessitating a breaking change.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Layout {
    /// Nodes are arranged in a [circular layout](https://en.wikipedia.org/wiki/Circular_layout).
    ///
    /// See [`circular_layout`][crate::layout::circular::circular_layout] for more details or
    /// calling the layout function directly.
    Circular,
    /// Nodes are arranged in a hierarchical layout.
    ///
    /// See [`hierarchical_layout`][crate::layout::hierarchical::hierarchical_layout] for more
    /// details or calling the layout function directly.
    Hierarchical(Orientation),
    /// Nodes are arranged using a [force-directed layout](https://en.wikipedia.org/wiki/Force-directed_graph_drawing).
    ///
    /// See [`force_directed_layout`][crate::layout::force_directed::force_directed_layout] for
    /// more details or calling the layout function directly.
    ForceDirected,
    /// Nodes are arranged in a [bipartite layout](https://en.wikipedia.org/wiki/Bipartite_graph).
    ///
    /// The provided `HashSet` contains the node IDs for the left partition. If `None` is
    /// provided, the layout function will attempt to determine the bipartition using a
    /// breadth-first traversal.
    Bipartite(Option<HashSet<NodeIndex>>),
    /// Nodes are arranged randomly.
    ///
    /// See [`random_layout`][crate::layout::random::random_layout] for more details or calling the
    /// layout function directly.
    Random,
}

/// Enum to represent either a layout algorithm or a custom position map function. Only used for
/// [`SettingsBuilder`][crate::settings::SettingsBuilder].
#[derive(Debug, Clone)]
pub enum LayoutOrPositionMap<PositionMapFn = DefaultPositionMapFn> {
    /// A layout algorithm.
    Layout(Layout),
    /// A custom position map function.
    PositionMap(PositionMapFn),
}

pub mod circular {
    //! Module containing functionality for the circular layout.
    //!
    //! The main function is [`circular_layout`], which returns a position map function that
    //! arranges nodes in a circular layout.
    use petgraph::visit::{IntoNodeReferences, NodeIndexable};

    /// Returns a position map function that arranges nodes in a circular layout.
    ///
    /// The nodes are evenly distributed around a unit circle centered at (0.5, 0.5). That is
    /// all values are in the range [0.0, 1.0].
    ///
    /// The node with the first (usually index 0) index is placed at the topmost point of the
    /// circle. Following nodes are placed in a clockwise manner.
    pub fn circular_layout<G>(graph: &G) -> impl Fn(G::NodeId) -> (f32, f32) + '_
    where
        G: IntoNodeReferences + NodeIndexable,
    {
        let node_count = graph.node_references().count() as f32;
        move |node_id| {
            let index = graph.to_index(node_id) as f32;
            let angle = index / node_count * std::f32::consts::TAU - std::f32::consts::FRAC_PI_2;
            let x = 0.5 + 0.5 * angle.cos();
            let y = 0.5 + 0.5 * angle.sin();
            (x, y)
        }
    }
}

pub mod random {
    //! Module containing functionality for the random layout.
    //!
    //! The main function is [`random_layout`], which returns a position map function that
    //! assigns random positions to nodes.

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
