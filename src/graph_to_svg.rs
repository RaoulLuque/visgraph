//! Functions to convert graphs to SVG representations.
//!
//! The main functions are [`graph_to_svg_with_positions`] and
//! [`graph_to_svg_with_layout`], which generate SVG data from a graph using either a
//! custom position map or a predefined layout algorithm, respectively.
//!
//! Note that the position map used in [`graph_to_svg_with_positions`] should return
//! normalized coordinates between 0.0 and 1.0.
//!
//! For examples, see the `examples/` directory.

use petgraph::visit::{
    EdgeIndexable, EdgeRef, IntoEdgeReferences, IntoNodeReferences, NodeIndexable, NodeRef,
};

use crate::layout::{self, Layout, LayoutOrPositionMap};
use crate::settings::Settings;

const EDGE_CLOSENESS_THRESHOLD: f32 = 0.001;

/// Generates an SVG representation of the graph using the provided position map and settings.
///
/// The position_map parameter should implement Fn(G::NodeId) -> (f32, f32) and return a tuple of
/// normalized (x, y) coordinates in the range [0.0, 1.0].
///
/// Example:
/// ```
/// use petgraph::graph::UnGraph;
/// use visgraph::graph_to_svg::graph_to_svg;
/// use visgraph::settings::SettingsBuilder;
/// use visgraph::Layout;
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
/// // Customize settings using the SettingsBuilder. Values which are not set will use defaults.
/// let settings = SettingsBuilder::new()
///     .width(500.0)
///     .height(500.0)
///     .position_map(position_map)
///     .build()
///     .expect("Values should be valid.");
///
/// // Generate svg output using the custom position map.
/// let svg_data = graph_to_svg(
///     &square_graph,
///     &settings,
/// );
/// ```
pub fn graph_to_svg<G, PositionMapFn, NodeLabelFn, EdgeLabelFn>(
    graph: G,
    settings: &Settings<PositionMapFn, NodeLabelFn, EdgeLabelFn>,
) -> String
where
    G: IntoNodeReferences + IntoEdgeReferences + NodeIndexable + EdgeIndexable,
    PositionMapFn: Fn(G::NodeId) -> (f32, f32),
    NodeLabelFn: Fn(G::NodeId) -> String,
    EdgeLabelFn: Fn(G::EdgeId) -> String,
{
    match &settings.layout {
        LayoutOrPositionMap::Layout(Layout::Circular) => {
            let position_map = layout::get_circular_position_map(&graph);
            internal_graph_to_svg_with_positions(graph, position_map, settings)
        }
        LayoutOrPositionMap::Layout(Layout::Hierarchical) => {
            let position_map = layout::get_hierarchical_position_map(&graph);
            internal_graph_to_svg_with_positions(graph, position_map, settings)
        }
        LayoutOrPositionMap::PositionMap(position_map) => {
            internal_graph_to_svg_with_positions(graph, position_map, settings)
        }
    }
}

fn internal_graph_to_svg_with_positions<G, PositionMapFn, NodeLabelFn, EdgeLabelFn, S>(
    graph: G,
    position_map: PositionMapFn,
    settings: &Settings<S, NodeLabelFn, EdgeLabelFn>,
) -> String
where
    G: IntoNodeReferences + IntoEdgeReferences + NodeIndexable + EdgeIndexable,
    PositionMapFn: Fn(G::NodeId) -> (f32, f32),
    NodeLabelFn: Fn(G::NodeId) -> String,
    EdgeLabelFn: Fn(G::EdgeId) -> String,
{
    match (&settings.node_label, &settings.edge_label) {
        (Some(node_label_map), Some(edge_label_map)) => {
            internal_graph_to_svg_with_positions_and_labels(
                graph,
                position_map,
                node_label_map,
                edge_label_map,
                settings,
            )
        }
        (Some(node_label_map), None) => internal_graph_to_svg_with_positions_and_labels(
            graph,
            position_map,
            node_label_map,
            |_| "".to_string(),
            settings,
        ),
        (None, Some(edge_label_map)) => internal_graph_to_svg_with_positions_and_labels(
            graph,
            position_map,
            |node_id| NodeIndexable::to_index(&graph, node_id).to_string(),
            edge_label_map,
            settings,
        ),
        (None, None) => internal_graph_to_svg_with_positions_and_labels(
            graph,
            position_map,
            |node_id| NodeIndexable::to_index(&graph, node_id).to_string(),
            |_| "".to_string(),
            settings,
        ),
    }
}

fn internal_graph_to_svg_with_positions_and_labels<
    G,
    PositionMapFn,
    NodeLabelFn,
    EdgeLabelFn,
    S,
    T,
    U,
>(
    graph: G,
    position_map: PositionMapFn,
    node_label_map: NodeLabelFn,
    edge_label_map: EdgeLabelFn,
    settings: &Settings<S, T, U>,
) -> String
where
    G: IntoNodeReferences + IntoEdgeReferences + NodeIndexable + EdgeIndexable,
    PositionMapFn: Fn(G::NodeId) -> (f32, f32),
    NodeLabelFn: Fn(G::NodeId) -> String,
    EdgeLabelFn: Fn(G::EdgeId) -> String,
{
    let mut svg_buffer = String::with_capacity(graph.node_bound() * 120 + graph.edge_bound() * 50);
    svg_buffer.push_str(&format!(
        "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">\n",
        settings.width, settings.height
    ));

    for node in graph.node_references() {
        let id = node.id();
        let (scaled_x, scaled_y) = scale(
            position_map(id),
            settings.margin_x,
            settings.margin_y,
            settings.width,
            settings.height,
        );
        let node_label = node_label_map(id);
        draw_node(
            &mut svg_buffer,
            scaled_x,
            scaled_y,
            &node_label,
            settings.radius,
            settings.font_size,
        );
    }

    for edge in graph.edge_references() {
        let source = edge.source();
        let target = edge.target();
        let (scaled_x_source, scaled_y_source) = scale(
            position_map(source),
            settings.margin_x,
            settings.margin_y,
            settings.width,
            settings.height,
        );
        let (scaled_x_target, scaled_y_target) = scale(
            position_map(target),
            settings.margin_x,
            settings.margin_y,
            settings.width,
            settings.height,
        );
        let edge_label = edge_label_map(edge.id());

        draw_edge(
            &mut svg_buffer,
            (scaled_x_source, scaled_y_source),
            (scaled_x_target, scaled_y_target),
            &edge_label,
            settings.radius,
            settings.stroke_width,
            settings.font_size,
        );
    }

    svg_buffer.push_str("</svg>");
    svg_buffer
}

/// Draws a node as a circle with a text label by writing appropriate <circle> and <text> tags to the provided svg_buffer.
fn draw_node(
    svg_buffer: &mut String,
    coord_x: f32,
    coord_y: f32,
    node_label: &str,
    radius: f32,
    font_size: f32,
) {
    svg_buffer.push_str(&format!("
    <circle cx=\"{coord_x}\" cy=\"{coord_y}\" r=\"{radius}\" fill=\"white\" stroke=\"black\"/>
    <text x=\"{coord_x}\" y=\"{coord_y}\" font-size=\"{font_size}px\" font-family='Arial, sans-serif' fill=\"black\" text-anchor=\"middle\" dominant-baseline=\"central\">{node_label}</text>\n",
        ));
}

/// Draws an edge as a line between two nodes by writing an appropriate <line> tag to the provided svg_buffer.
/// Adjusting for the radius of the nodes so that the line starts and ends at the edge of the nodes rather than their centers.
#[allow(clippy::too_many_arguments)]
fn draw_edge(
    svg_buffer: &mut String,
    coord_source: (f32, f32),
    coord_target: (f32, f32),
    edge_label: &str,
    radius: f32,
    stroke_width: f32,
    font_size: f32,
) {
    let (coord_x_source, coord_y_source) = coord_source;
    let (coord_x_target, coord_y_target) = coord_target;

    // To properly draw the edge from the edge of the source node to the edge of the target node,
    // we need to multiply the radius of the nodes by the normalized direction vector and use that
    // as the start and end points of the edge.
    let dir_vec_x = coord_x_target - coord_x_source;
    let dir_vec_y = coord_y_target - coord_y_source;
    let distance = (dir_vec_x * dir_vec_x + dir_vec_y * dir_vec_y).sqrt();

    // For very close nodes, we skip drawing the edge to avoid division by zero.
    if distance < EDGE_CLOSENESS_THRESHOLD {
        return;
    }

    // Normalize the direction vector
    let unit_dir_vec_x = dir_vec_x / distance;
    let unit_dir_vec_y = dir_vec_y / distance;

    // Calculate the start and end point point (on the boundary of the circles)
    let start_x = coord_x_source + radius * unit_dir_vec_x;
    let start_y = coord_y_source + radius * unit_dir_vec_y;
    let end_x = coord_x_target - radius * unit_dir_vec_x;
    let end_y = coord_y_target - radius * unit_dir_vec_y;

    svg_buffer.push_str(&format!(
        "
    <line x1=\"{start_x}\" y1=\"{start_y}\" x2=\"{end_x}\" y2=\"{end_y}\" stroke=\"black\" stroke-width=\"{stroke_width}\"/>
    <text x= \"{}\" y=\"{}\" font-size=\"{font_size}px\" font-family='Arial, sans-serif' fill=\"blue\" text-anchor=\"middle\" dominant-baseline=\"central\">{edge_label}</text>\n",
    (start_x + end_x) / 2.0, (start_y + end_y) / 2.0
    ));
}

/// Scales normalized coordinates (0.0 to 1.0, 0.0 to 1.0) to actual canvas coordinates (0 to width, 0 to height).
/// Takes into account the margins specified in the settings. Margins are specified as a fraction of the total width/height
/// and are applied on both sides (left/right and top/bottom).
///
/// E.g. if margin_x is 0.1, then 10% of the width is reserved as margin on the left and 10% on the right,
/// leaving 80% of the width for the actual graph drawing area.
fn scale(
    (normalized_x, normalized_y): (f32, f32),
    margin_x: f32,
    margin_y: f32,
    width: f32,
    height: f32,
) -> (f32, f32) {
    let upscaled_x = normalized_x * width;
    let upscaled_y = normalized_y * height;

    let margin_adjusted_upscaled_x = margin_x * width + upscaled_x * (1.0 - 2.0 * margin_x);
    let margin_adjusted_upscaled_y = margin_y * height + upscaled_y * (1.0 - 2.0 * margin_y);

    (margin_adjusted_upscaled_x, margin_adjusted_upscaled_y)
}

#[cfg(test)]
mod tests {
    use crate::{graph_to_svg::graph_to_svg, tests::test_square_graph_with_position_map};

    #[test]
    fn test_scale() {
        let (scaled_x, scaled_y) = super::scale((0.5, 0.5), 0.1, 0.1, 1000.0, 1000.0);
        assert!((scaled_x - 500.0).abs() < f32::EPSILON);
        assert!((scaled_y - 500.0).abs() < f32::EPSILON);

        let (scaled_x, scaled_y) = super::scale((0.0, 0.0), 0.1, 0.1, 1000.0, 1000.0);
        assert!((scaled_x - 100.0).abs() < f32::EPSILON);
        assert!((scaled_y - 100.0).abs() < f32::EPSILON);

        let (scaled_x, scaled_y) = super::scale((1.0, 1.0), 0.1, 0.1, 1000.0, 1000.0);
        assert!((scaled_x - 900.0).abs() < f32::EPSILON);
        assert!((scaled_y - 900.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_graph_to_svg_with_position_map() {
        let (graph, settings) = test_square_graph_with_position_map();
        let svg_output = graph_to_svg(&graph, &settings);

        println!("{}", svg_output);

        let expected_output = "<svg width=\"500\" height=\"500\" xmlns=\"http://www.w3.org/2000/svg\">

    <circle cx=\"137.5\" cy=\"137.5\" r=\"25\" fill=\"white\" stroke=\"black\"/>
    <text x=\"137.5\" y=\"137.5\" font-size=\"16px\" font-family='Arial, sans-serif' fill=\"black\" text-anchor=\"middle\" dominant-baseline=\"central\">0</text>

    <circle cx=\"362.5\" cy=\"137.5\" r=\"25\" fill=\"white\" stroke=\"black\"/>
    <text x=\"362.5\" y=\"137.5\" font-size=\"16px\" font-family='Arial, sans-serif' fill=\"black\" text-anchor=\"middle\" dominant-baseline=\"central\">1</text>

    <circle cx=\"362.5\" cy=\"362.5\" r=\"25\" fill=\"white\" stroke=\"black\"/>
    <text x=\"362.5\" y=\"362.5\" font-size=\"16px\" font-family='Arial, sans-serif' fill=\"black\" text-anchor=\"middle\" dominant-baseline=\"central\">2</text>

    <circle cx=\"137.5\" cy=\"362.5\" r=\"25\" fill=\"white\" stroke=\"black\"/>
    <text x=\"137.5\" y=\"362.5\" font-size=\"16px\" font-family='Arial, sans-serif' fill=\"black\" text-anchor=\"middle\" dominant-baseline=\"central\">3</text>

    <line x1=\"162.5\" y1=\"137.5\" x2=\"337.5\" y2=\"137.5\" stroke=\"black\" stroke-width=\"5\"/>
    <text x= \"250\" y=\"137.5\" font-size=\"16px\" font-family='Arial, sans-serif' fill=\"blue\" text-anchor=\"middle\" dominant-baseline=\"central\"></text>

    <line x1=\"362.5\" y1=\"162.5\" x2=\"362.5\" y2=\"337.5\" stroke=\"black\" stroke-width=\"5\"/>
    <text x= \"362.5\" y=\"250\" font-size=\"16px\" font-family='Arial, sans-serif' fill=\"blue\" text-anchor=\"middle\" dominant-baseline=\"central\"></text>

    <line x1=\"337.5\" y1=\"362.5\" x2=\"162.5\" y2=\"362.5\" stroke=\"black\" stroke-width=\"5\"/>
    <text x= \"250\" y=\"362.5\" font-size=\"16px\" font-family='Arial, sans-serif' fill=\"blue\" text-anchor=\"middle\" dominant-baseline=\"central\"></text>

    <line x1=\"137.5\" y1=\"337.5\" x2=\"137.5\" y2=\"162.5\" stroke=\"black\" stroke-width=\"5\"/>
    <text x= \"137.5\" y=\"250\" font-size=\"16px\" font-family='Arial, sans-serif' fill=\"blue\" text-anchor=\"middle\" dominant-baseline=\"central\"></text>
</svg>".to_owned();

        assert_eq!(svg_output, expected_output);
    }
}
