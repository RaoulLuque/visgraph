use resvg::render;
use resvg::tiny_skia;

fn main() {
    let mut graph = petgraph::Graph::<&str, &str>::new();
    let node_a = graph.add_node("A");
    let node_b = graph.add_node("B");
    graph.add_edge(node_a, node_b, "edge1");
    // let svg_data = r#"
    // <svg width="300" height="200" xmlns="http://www.w3.org/2000/svg">
    //     <circle cx="100" cy="100" r="20" fill="lightblue" stroke="black"/>
    //     <circle cx="200" cy="100" r="20" fill="lightgreen" stroke="black"/>
    //     <line x1="120" y1="100" x2="180" y2="100" stroke="black"/>
    //     <text x="80" y="105" font-size="14" font-family="Arial, sans-serif" fill="black">A</text>
    //     <text x="195" y="105" font-size="14" font-family="Arial, sans-serif" fill="black">B</text>
    // </svg>
    // "#;
    let svg_data = visgraph::graph_to_svg(
        &graph,
        |node_id| match node_id.index() {
            0 => (100.0, 100.0),
            1 => (200.0, 100.0),
            _ => (0.0, 0.0),
        },
        |node_id| graph.node_weight(node_id).unwrap().to_string(),
    );

    // Parse SVG
    let mut opt = resvg::usvg::Options::default();
    opt.dpi = 300.0;
    opt.fontdb_mut().load_system_fonts();

    let rtree = resvg::usvg::Tree::from_data(svg_data.as_bytes(), &opt).unwrap();

    // Render to pixmap
    let mut pixmap = tiny_skia::Pixmap::new(300, 200).unwrap();
    render(
        &rtree,
        tiny_skia::Transform::identity(),
        &mut pixmap.as_mut(),
    );
    pixmap.save_png("graph_with_text.png").unwrap();
}
