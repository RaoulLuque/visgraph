#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, missing_docs)]

mod errors;
pub mod graph_to_svg;
mod layout;
pub mod settings;
mod svg_to_img;

use petgraph::visit::{EdgeIndexable, IntoEdgeReferences, IntoNodeReferences, NodeIndexable};
pub use svg_to_img::parse_svg_to_img;

pub use layout::Layout;

use crate::{
    errors::VisGraphError,
    graph_to_svg::{graph_to_svg_with_layout, graph_to_svg_with_positions},
    settings::Settings,
};

pub fn graph_to_img_with_layout<G, FnNodeLabel, FnEdgeLabel>(
    graph: G,
    layout: Layout,
    settings: &Settings<FnNodeLabel, FnEdgeLabel>,
    path: impl AsRef<std::path::Path>,
) -> Result<(), VisGraphError>
where
    G: IntoNodeReferences + IntoEdgeReferences + NodeIndexable + EdgeIndexable,
    FnNodeLabel: Fn(G::NodeId) -> String,
    FnEdgeLabel: Fn(G::EdgeId) -> String,
{
    let svg_data = graph_to_svg_with_layout(graph, layout, settings);
    parse_svg_to_img(&svg_data, settings, path)?;
    Ok(())
}

pub fn graph_to_img_with_position_map<G, FnNodeLabel, FnEdgeLabel, FnPos>(
    graph: G,
    position_map: FnPos,
    settings: &Settings<FnNodeLabel, FnEdgeLabel>,
    path: impl AsRef<std::path::Path>,
) -> Result<(), VisGraphError>
where
    G: IntoNodeReferences + IntoEdgeReferences + NodeIndexable + EdgeIndexable,
    FnNodeLabel: Fn(G::NodeId) -> String,
    FnEdgeLabel: Fn(G::EdgeId) -> String,
    FnPos: Fn(G::NodeId) -> (f32, f32),
{
    let svg_data = graph_to_svg_with_positions(graph, position_map, settings);
    parse_svg_to_img(&svg_data, settings, path)?;
    Ok(())
}
