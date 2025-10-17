use petgraph::visit::{
    EdgeIndexable, EdgeRef, IntoEdgeReferences, IntoNodeReferences, NodeIndexable, NodeRef,
};

use crate::layout::Layout;

const RADIUS: f32 = 20.0;
const FONT_SIZE: f32 = 14.0;
const EDGE_CLOSENESS_THRESHOLD: f32 = 0.001;

pub fn graph_to_svg_with_positions<G, FnPos, FnLabel>(
    graph: G,
    position_map: FnPos,
    label_map: FnLabel,
) -> String
where
    G: IntoNodeReferences + IntoEdgeReferences + NodeIndexable + EdgeIndexable,
    FnPos: Fn(G::NodeId) -> (f32, f32),
    FnLabel: Fn(G::NodeId) -> String,
{
    let mut svg_buffer = String::with_capacity(graph.node_bound() * 120 + graph.edge_bound() * 50);
    svg_buffer.push_str(r#"<svg width="300" height="200" xmlns="http://www.w3.org/2000/svg">"#);

    for node in graph.node_references() {
        let id = node.id();
        let (coord_x, coord_y) = position_map(id);
        let node_label = label_map(id);
        draw_node(&mut svg_buffer, coord_x, coord_y, &node_label);
    }

    for edge in graph.edge_references() {
        let source = edge.source();
        let target = edge.target();
        let (coord_x_source, coord_y_source) = position_map(source);
        let (coord_x_target, coord_y_target) = position_map(target);
        draw_edge(
            &mut svg_buffer,
            coord_x_source,
            coord_y_source,
            coord_x_target,
            coord_y_target,
        );
    }

    svg_buffer.push_str("</svg>");
    svg_buffer
}

pub fn graph_to_svg_with_layout<G, FnLabel>(graph: G, layout: Layout, label_map: FnLabel) -> String
where
    G: IntoNodeReferences + IntoEdgeReferences + NodeIndexable + EdgeIndexable,
    FnLabel: Fn(G::NodeId) -> String,
{
    match layout {
        Layout::Circular => {
            let node_count = graph.node_references().count() as f32;
            let position_map = move |node_id| {
                let index = NodeIndexable::to_index(&graph, node_id) as f32;
                let angle = index / node_count * std::f32::consts::TAU;
                let x = 150.0 + 100.0 * angle.cos();
                let y = 150.0 + 100.0 * angle.sin();
                (x, y)
            };

            graph_to_svg_with_positions(graph, position_map, label_map)
        }
        Layout::Hierarchical => {
            let mut levels: Vec<Vec<G::NodeId>> = Vec::new();
            for node in graph.node_references() {
                let level = NodeIndexable::to_index(&graph, node.id()) / 2;
                if levels.len() <= level {
                    levels.push(Vec::new());
                }
                levels[level].push(node.id());
            }
            let position_map = move |node_id| {
                let index = NodeIndexable::to_index(&graph, node_id);
                let level = index / 2;
                let position_in_level = levels[level].iter().position(|&id| id == node_id).unwrap();
                let nodes_in_level = levels[level].len();
                let x = 50.0 + (position_in_level as f32) * (200.0 / (nodes_in_level as f32));
                let y = 50.0 + (level as f32) * 100.0;
                (x, y)
            };

            graph_to_svg_with_positions(graph, position_map, label_map)
        }
    }
}

fn draw_node(svg_buffer: &mut String, coord_x: f32, coord_y: f32, node_label: &str) {
    svg_buffer.push_str(&format!(
            "   <circle cx=\"{coord_x}\" cy=\"{coord_y}\" r=\"{RADIUS}\" fill=\"white\" stroke=\"black\"/>\n
    <text x=\"{coord_x}\" y=\"{coord_y}\" font-size=\"{FONT_SIZE}\" font-family='Arial, sans-serif' fill=\"black\" text-anchor=\"middle\" dominant-baseline=\"central\">{node_label}</text>\n",
        ));
}

fn draw_edge(
    svg_buffer: &mut String,
    coord_x_source: f32,
    coord_y_source: f32,
    coord_x_target: f32,
    coord_y_target: f32,
) {
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
    let start_x = coord_x_source + RADIUS * unit_dir_vec_x;
    let start_y = coord_y_source + RADIUS * unit_dir_vec_y;
    let end_x = coord_x_target - RADIUS * unit_dir_vec_x;
    let end_y = coord_y_target - RADIUS * unit_dir_vec_y;

    svg_buffer.push_str(&format!(
        "   <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"black\"/>\n",
        start_x, start_y, end_x, end_y
    ));
}
