use petgraph::visit::{IntoNodeReferences, NodeIndexable};

pub(crate) use hierarchical::get_hierarchical_position_map;
pub use hierarchical::Orientation;

pub(crate) type DefaultPositionMapFn = fn(petgraph::prelude::NodeIndex) -> (f32, f32);

mod hierarchical;

/// Different layout algorithms for graph visualization.
///
/// For examples, see the [`examples`](https://github.com/RaoulLuque/visgraph/tree/main/examples) directory.
#[derive(Debug, Clone, Copy)]
pub enum Layout {
    /// Nodes are arranged in a [circular layout](https://en.wikipedia.org/wiki/Circular_layout).
    Circular,
    /// Nodes are arranged in a hierarchical layout.
    Hierarchical(Orientation),
}

/// Enum to represent either a layout algorithm or a custom position map function. Only used for
/// [`SettingsBuilder`][crate::settings::SettingsBuilder].
#[derive(Debug, Clone, Copy)]
pub enum LayoutOrPositionMap<PositionMapFn = DefaultPositionMapFn> {
    Layout(Layout),
    PositionMap(PositionMapFn),
}

/// Returns a position map function that arranges nodes in a circular layout.
pub(crate) fn get_circular_position_map<G>(graph: &G) -> impl Fn(G::NodeId) -> (f32, f32) + '_
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
