use petgraph::visit::{EdgeRef, IntoEdgeReferences, IntoNodeReferences, NodeRef};

const RADIUS: f32 = 20.0;
const FONT_SIZE: f32 = 14.0;

pub fn graph_to_svg<G, FnPos, FnLabel>(graph: G, position_map: FnPos, label_map: FnLabel) -> String
where
    G: IntoNodeReferences + IntoEdgeReferences,
    FnPos: Fn(G::NodeId) -> (f32, f32),
    FnLabel: Fn(G::NodeId) -> String,
{
    let mut svg_data = String::new();
    svg_data.push_str(r#"<svg width="300" height="200" xmlns="http://www.w3.org/2000/svg">"#);

    // Add nodes
    for node in graph.node_references() {
        let id = node.id();
        let (coord_x, coord_y) = position_map(id);
        let node_label = label_map(id);
        svg_data.push_str(&format!(
            "<circle cx=\"{coord_x}\" cy=\"{coord_y}\" r=\"{RADIUS}\" fill=\"white\" stroke=\"black\"/>\n
            <text x=\"{coord_x}\" y=\"{coord_y}\" font-size=\"{FONT_SIZE}\" font-family='Arial, sans-serif' fill=\"black\" text-anchor=\"middle\" dominant-baseline=\"central\">{node_label}</text>\n",
        ));
    }

    // Add edges
    for edge in graph.edge_references() {
        let source = edge.source();
        let target = edge.target();
        let (coord_x_source, coord_y_source) = position_map(source);
        let (coord_x_target, coord_y_target) = position_map(target);
        svg_data.push_str(&format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"black\"/>\n",
            coord_x_source + RADIUS,
            coord_y_source,
            coord_x_target - RADIUS,
            coord_y_target
        ));
    }

    svg_data.push_str("</svg>");
    svg_data
}
