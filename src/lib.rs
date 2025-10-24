#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, missing_docs)]

mod errors;
pub mod graph_to_svg;
mod layout;
pub mod settings;
#[cfg(feature = "svg_to_img")]
pub mod svg_to_img;

pub use layout::{Layout, Orientation};
#[cfg(feature = "svg_to_img")]
use petgraph::visit::IntoNeighborsDirected;
use petgraph::visit::{EdgeIndexable, IntoEdgeReferences, IntoNodeReferences, NodeIndexable};
#[cfg(feature = "svg_to_img")]
use svg_to_img::svg_to_img;

use crate::{errors::VisGraphError, graph_to_svg::graph_to_svg_string, settings::Settings};

/// Generate and save an image of a graph to the specified path.
///
/// # Usage
/// 
/// The following is an example taken from
/// [`examples/graph_with_default_settings.rs`](https://github.com/RaoulLuque/visgraph/blob/main/examples/graph_with_default_settings.rs):
/// ```
#[allow(clippy::needless_doctest_main)]
#[doc = include_str!("../examples/graph_with_default_settings.rs")]
/// ```
/// More examples can be found in the [`examples`](https://github.com/RaoulLuque/visgraph/tree/main/examples)
/// directory.
#[cfg(feature = "svg_to_img")]
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

#[cfg(test)]
#[allow(missing_docs)]
pub mod tests {
    use petgraph::graph::{EdgeIndex, NodeIndex, UnGraph};

    use crate::{
        layout::DefaultPositionMapFn,
        settings::{DefaultEdgeLabelFn, DefaultNodeLabelFn, Settings, SettingsBuilder},
        Layout,
    };

    /// Create a test graph with custom node and edge labels, along with the corresponding settings.
    ///
    /// The same as in examples/graph_with_custom_labels.rs
    #[allow(clippy::type_complexity)]
    pub fn test_graph_with_custom_labels() -> (
        UnGraph<String, ()>,
        Settings<DefaultPositionMapFn, impl Fn(NodeIndex) -> String, impl Fn(EdgeIndex) -> String>,
    ) {
        let mut graph = UnGraph::new_undirected();
        let node_a = graph.add_node("Ljubljana".to_string());
        let node_b = graph.add_node("Bielefeld".to_string());
        let node_c = graph.add_node("Cape Town".to_string());
        let node_d = graph.add_node("Lima".to_string());

        graph.add_edge(node_a, node_b, ());
        graph.add_edge(node_b, node_c, ());
        graph.add_edge(node_c, node_d, ());
        graph.add_edge(node_d, node_a, ());

        let graph_clone = graph.clone();

        let node_labels = move |node_id| graph_clone.node_weight(node_id).unwrap().to_owned();
        let edge_labels = |_| "An edge".to_string();

        let settings = SettingsBuilder::new()
            .width(1000.0)
            .height(1000.0)
            .node_radius(50.0)
            .margin_x(0.1)
            .margin_y(0.1)
            .layout(Layout::Circular)
            .node_label_fn(node_labels)
            .edge_label_fn(edge_labels)
            .build()
            .expect("Values should be valid.");

        (graph, settings)
    }

    /// Create a test square graph with a custom position map, along with the corresponding
    /// settings.
    ///
    /// The same as in examples/graph_with_position_map.rs
    #[allow(clippy::type_complexity)]
    pub fn test_graph_with_position_map() -> (
        UnGraph<(), ()>,
        Settings<impl Fn(NodeIndex) -> (f32, f32), DefaultNodeLabelFn, DefaultEdgeLabelFn>,
    ) {
        let mut square_graph = UnGraph::new_undirected();
        let node_a = square_graph.add_node(());
        let node_b = square_graph.add_node(());
        let node_c = square_graph.add_node(());
        let node_d = square_graph.add_node(());

        square_graph.add_edge(node_a, node_b, ());
        square_graph.add_edge(node_b, node_c, ());
        square_graph.add_edge(node_c, node_d, ());
        square_graph.add_edge(node_d, node_a, ());

        // Positions should be between (0.0) and (1.0)
        let position_map = move |node_id| match node_id {
            id if id == node_a => (0.25, 0.25),
            id if id == node_b => (0.75, 0.25),
            id if id == node_c => (0.75, 0.75),
            id if id == node_d => (0.25, 0.75),
            _ => (0.5, 0.5),
        };

        // Customize settings using the SettingsBuilder.
        let settings = SettingsBuilder::new()
            .width(500.0)
            .height(500.0)
            .position_map(position_map)
            .build()
            .expect("Values should be valid.");
        (square_graph, settings)
    }
}
