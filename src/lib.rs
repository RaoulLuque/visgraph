#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, missing_docs)]

mod errors;
pub mod graph_to_svg;
mod layout;
pub mod settings;
mod svg_to_img;

use petgraph::visit::{EdgeIndexable, IntoEdgeReferences, IntoNodeReferences, NodeIndexable};
pub use svg_to_img::svg_to_img;

pub use layout::Layout;

use crate::{
    errors::VisGraphError,
    graph_to_svg::{graph_to_svg_with_layout, graph_to_svg_with_positions},
    settings::Settings,
};

/// Generate and save an image of a graph to the specified path.
///
/// The layout parameter determines how the nodes are positioned in the image.
///
/// The settings parameter allows customization of various visual aspects of the graph. Either use
/// [`Settings::default()`](settings/struct.Settings.html#method.default) for default settings or
/// create custom settings using the [`SettingsBuilder`](settings/struct.SettingsBuilder.html).
///
/// Example:
/// ```rust
/// use petgraph::graph::UnGraph;
/// use visgraph::graph_to_img_with_layout;
/// use visgraph::settings::SettingsBuilder;
///
/// // Create a complete graph with 100 nodes.
/// let mut complete_graph = UnGraph::new_undirected();
/// let num_nodes = 100;
/// let nodes: Vec<_> = (0..num_nodes)
///     .map(|_| complete_graph.add_node(()))
///     .collect();

/// for i in 0..num_nodes {
///     for j in (i + 1)..num_nodes {
///         complete_graph.add_edge(nodes[i], nodes[j], ());
///     }
/// }

/// // Customize settings using the SettingsBuilder.
/// let settings = SettingsBuilder::new()
///     .width(1000.0)
///     .height(1000.0)
///     .radius(7.5)
///     .stroke_width(0.1)
///     .font_size(7.5)
///     .build()
///     .expect("Values should be valid.");

/// // Generate and save the graph image using a circular layout.
/// graph_to_img_with_layout(
///     &complete_graph,
///     visgraph::Layout::Circular,
///     &settings,
///     "examples/results/complete_graph_with_circular_layout.png",
/// )
/// .unwrap();
/// ```
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
    svg_to_img(&svg_data, settings, path)?;
    Ok(())
}

/// Generate and save an image of a graph to the specified path using a custom position map.
///
/// The position_map parameter is a function that takes a node ID and returns its (x, y)
/// coordinates, which **should be normalized between 0.0 and 1.0**.
///
/// The settings parameter allows
/// customization of various visual aspects of the graph. Either use
/// [`Settings::default()`](settings/struct.Settings.html#method.default) for default settings or
/// create custom settings using the [`SettingsBuilder`](settings/struct.SettingsBuilder.html).
///
/// Example:
/// ```rust
/// use petgraph::graph::UnGraph;
/// use visgraph::graph_to_img_with_position_map;
/// use visgraph::settings::SettingsBuilder;
///
/// // Create a square graph with four nodes
/// // It should look like this:
/// // A --- B
/// // |     |
/// // D --- C
/// let mut square_graph = UnGraph::new_undirected();
/// let node_a = square_graph.add_node(());
/// let node_b = square_graph.add_node(());
/// let node_c = square_graph.add_node(());
/// let node_d = square_graph.add_node(());
///
/// square_graph.add_edge(node_a, node_b, ());
/// square_graph.add_edge(node_b, node_c, ());
/// square_graph.add_edge(node_c, node_d, ());
/// square_graph.add_edge(node_d, node_a, ());
///
/// // Positions should be between (0.0) and (1.0)
/// let position_map = |node_id| match node_id {
///     id if id == node_a => (0.25, 0.25),
///     id if id == node_b => (0.75, 0.25),
///     id if id == node_c => (0.75, 0.75),
///     id if id == node_d => (0.25, 0.75),
///     _ => (0.5, 0.5),
/// };
///
/// // Customize settings using the SettingsBuilder.
/// let settings = SettingsBuilder::new()
///     .width(3000.0)
///     .height(3000.0)
///     .build()
///     .expect("Values should be valid.");
///
/// // Generate and save the graph image the custom position map.
/// graph_to_img_with_position_map(
///     &square_graph,
///     position_map,
///     &settings,
///     "examples/results/square_graph_with_position_map.png",
/// )
/// .unwrap();
/// ```
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
    svg_to_img(&svg_data, settings, path)?;
    Ok(())
}
