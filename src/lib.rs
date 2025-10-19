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

pub fn graph_to_img_with_layout<G, FnNodeLabel>(
    graph: G,
    layout: Layout,
    node_label: Option<FnNodeLabel>,
    settings: &Settings,
    path: impl AsRef<std::path::Path>,
) -> Result<(), VisGraphError>
where
    G: IntoNodeReferences + IntoEdgeReferences + NodeIndexable + EdgeIndexable,
    FnNodeLabel: Fn(G::NodeId) -> String,
{
    if let Some(label_map) = node_label {
        let svg_data = graph_to_svg_with_layout(graph, layout, label_map, settings);
        parse_svg_to_img(&svg_data, settings, path)?;
        Ok(())
    } else {
        let svg_data = graph_to_svg_with_layout(
            graph,
            layout,
            |node_id| NodeIndexable::to_index(&graph, node_id).to_string(),
            settings,
        );
        parse_svg_to_img(&svg_data, settings, path)?;
        Ok(())
    }
}

pub fn graph_to_img_with_position_map<G, FnNodeLabel, FnPos>(
    graph: G,
    position_map: FnPos,
    label_map: FnNodeLabel,
    settings: &Settings,
    path: impl AsRef<std::path::Path>,
) -> Result<(), VisGraphError>
where
    G: IntoNodeReferences + IntoEdgeReferences + NodeIndexable + EdgeIndexable,
    FnNodeLabel: Fn(G::NodeId) -> String,
    FnPos: Fn(G::NodeId) -> (f32, f32),
{
    if let Some(label_map) = Some(label_map) {
        let svg_data = graph_to_svg_with_positions(graph, position_map, label_map, settings);
        parse_svg_to_img(&svg_data, settings, path)?;
        Ok(())
    } else {
        let svg_data = graph_to_svg_with_positions(
            graph,
            position_map,
            |node_id| NodeIndexable::to_index(&graph, node_id).to_string(),
            settings,
        );
        parse_svg_to_img(&svg_data, settings, path)?;
        Ok(())
    }
}
