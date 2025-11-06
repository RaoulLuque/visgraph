//! Functionality to convert graphs to PNGs.
//!
//! The main function is [`graph_to_img`] which generates a PNG from a graph using either a
//! custom position map or a predefined layout algorithm, respectively.
//!
//! Note that if a position map is used, it should return normalized coordinates between 0.0 and
//! 1.0.
//!
//! For examples, see the `examples/` directory.

use petgraph::visit::{
    EdgeIndexable, IntoEdgeReferences, IntoNeighborsDirected, IntoNodeReferences, NodeIndexable,
};

use crate::{
    errors::VisGraphError, graph_to_svg::graph_to_svg_string, settings::Settings,
    svg_to_img::svg_to_img,
};

/// Generate and save an image of a graph to the specified path.
///
/// # Settings
///
/// To configure the graph rendering, use the [`SettingsBuilder`](crate::settings::SettingsBuilder)
/// struct.
///
/// # Usage
///
/// The following is an example taken from
/// [`examples/default_settings.rs`](https://github.com/RaoulLuque/visgraph/blob/main/examples/default_settings.rs):
/// ```
#[allow(clippy::needless_doctest_main)]
#[doc = include_str!("../examples/default_settings.rs")]
/// ```
/// More examples can be found in the [`examples`](https://github.com/RaoulLuque/visgraph/tree/main/examples)
/// directory.
#[cfg(feature = "img")]
pub fn graph_to_img<G, PositionMapFn, NodeLabelFn, EdgeLabelFn, NodeColoringFn, EdgeColoringFn>(
    graph: G,
    settings: &Settings<PositionMapFn, NodeLabelFn, EdgeLabelFn, NodeColoringFn, EdgeColoringFn>,
    path: impl AsRef<std::path::Path>,
) -> Result<(), VisGraphError>
where
    G: IntoNodeReferences
        + IntoEdgeReferences
        + NodeIndexable
        + EdgeIndexable
        + IntoNeighborsDirected,
    PositionMapFn: Fn(G::NodeId) -> (f32, f32),
    NodeLabelFn: Fn(G::NodeId) -> String,
    EdgeLabelFn: Fn(G::EdgeId) -> String,
    NodeColoringFn: Fn(G::NodeId) -> String,
    EdgeColoringFn: Fn(G::EdgeId) -> String,
{
    let svg_data = graph_to_svg_string(graph, settings);
    svg_to_img(&svg_data, settings.width, settings.height, path)?;
    Ok(())
}
