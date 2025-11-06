#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, missing_docs)]

mod errors;
#[cfg(feature = "img")]
pub mod graph_to_img;
pub mod graph_to_svg;
pub mod layout;
pub mod settings;
#[cfg(feature = "img")]
pub mod svg_to_img;

#[cfg(feature = "img")]
pub use graph_to_img::graph_to_img;
pub use graph_to_svg::graph_to_svg;
pub use layout::{hierarchical::Orientation, Layout};

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
    /// The same as in `examples/custom_labels.rs`
    #[allow(clippy::type_complexity)]
    pub fn test_custom_labels() -> (
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
    /// The same as in `examples/position_map.rs`
    #[allow(clippy::type_complexity)]
    pub fn test_position_map() -> (
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
