#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, missing_docs)]

mod errors;
pub mod graph_to_svg;
mod layout;
pub mod settings;
#[cfg(feature = "svg_to_img")]
pub mod svg_to_img;

pub use layout::Layout;
use petgraph::visit::{EdgeIndexable, IntoEdgeReferences, IntoNodeReferences, NodeIndexable};
#[cfg(feature = "svg_to_img")]
use svg_to_img::svg_to_img;

use crate::{
    errors::VisGraphError,
    graph_to_svg::{graph_to_svg_with_layout, graph_to_svg_with_positions},
    settings::Settings,
};

/// Generate and save an image of a graph to the specified path.
///
/// # Parameters
///
/// The layout parameter determines how the nodes are positioned in the image.
///
/// The settings parameter allows customization of various visual aspects of the graph. Either use
/// [`Settings::default()`](settings/struct.Settings.html#method.default) for default settings or
/// create custom settings using the [`SettingsBuilder`](settings/struct.SettingsBuilder.html).
///
/// # Usage
/// The following is an example taken from
/// [`examples/complete_graph_with_circular_layout.rs`](https://github.com/RaoulLuque/visgraph/blob/main/examples/complete_graph_with_circular_layout.rs):
/// ```
#[doc = include_str!("../examples/complete_graph_with_circular_layout.rs")]
/// ```
#[cfg(feature = "svg_to_img")]
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
    svg_to_img(&svg_data, settings.width, settings.height, path)?;
    Ok(())
}

/// Generate and save an image of a graph to the specified path using a custom position map.
///
/// # Parameters
///
/// The position_map parameter is a function that takes a node ID and returns its (x, y)
/// coordinates, which **should be normalized between 0.0 and 1.0**.
///
/// The settings parameter allows
/// customization of various visual aspects of the graph. Either use
/// [`Settings::default()`](settings/struct.Settings.html#method.default) for default settings or
/// create custom settings using the [`SettingsBuilder`](settings/struct.SettingsBuilder.html).
///
/// # Usage
///
/// The following is an example taken from
/// [`examples/square_graph_with_position_map.rs`](https://github.com/RaoulLuque/visgraph/blob/main/examples/square_graph_with_position_map.rs):
/// ```
#[doc = include_str!("../examples/square_graph_with_position_map.rs")]
/// ```
#[cfg(feature = "svg_to_img")]
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
    svg_to_img(&svg_data, settings.width, settings.height, path)?;
    Ok(())
}

#[cfg(test)]
#[allow(missing_docs)]
pub mod tests {
    use petgraph::graph::{NodeIndex, UnGraph};

    use crate::settings::{DefaultEdgeLabelFn, DefaultNodeLabelFn, Settings, SettingsBuilder};

    /// Create a test graph with custom node and edge labels, along with the corresponding settings.
    ///
    /// The same as in examples/graph_with_custom_labels.rs
    #[allow(clippy::type_complexity)]
    pub fn test_graph_with_custom_labels() -> (
        UnGraph<String, ()>,
        Settings<
            impl Fn(petgraph::prelude::NodeIndex) -> String,
            impl Fn(petgraph::prelude::EdgeIndex) -> String,
        >,
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
            .node_label_fn(node_labels)
            .edge_label_fn(edge_labels)
            .build()
            .expect("Values should be valid.");

        (graph, settings)
    }

    /// Create a test square graph with a custom position map, along with the corresponding settings.
    ///
    /// The same as in examples/square_graph_with_position_map.rs
    #[allow(clippy::type_complexity)]
    pub fn test_square_graph_with_position_map() -> (
        UnGraph<(), ()>,
        Settings<DefaultNodeLabelFn, DefaultEdgeLabelFn>,
        impl Fn(NodeIndex) -> (f32, f32),
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
            .build()
            .expect("Values should be valid.");
        (square_graph, settings, position_map)
    }
}
