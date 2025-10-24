use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashMap;
use visgraph::graph_to_img;
use visgraph::settings::SettingsBuilder;

// 7 colors for the 7 layers
const LAYER_COLORS: [&str; 7] = ["red", "orange", "yellow", "green", "cyan", "blue", "purple"];

fn main() {
    let (graph, node_layers, node_positions) = create_graph();

    // Create settings with custom node coloring and position map
    let settings = SettingsBuilder::new()
        .node_radius(15.0)
        .position_map(move |node_id| *node_positions.get(&node_id.index()).unwrap_or(&(0.5, 0.5)))
        // Custom node coloring
        .node_coloring_fn(move |node_id| {
            let layer = node_layers[node_id.index()];
            LAYER_COLORS[layer].to_string()
        })
        .node_label_fn(|_| "".to_string())
        .margin_x(0.01)
        .margin_y(0.01)
        .build()
        .unwrap();

    // Generate and save the graph image
    graph_to_img(
        &graph,
        &settings,
        "examples/results/graph_with_custom_colors.png",
    )
    .unwrap();
}

/// Creates a graph with 7 square layers, each layer connected to the next where each layer
/// has a different color.
fn create_graph() -> (UnGraph<(), ()>, Vec<usize>, HashMap<usize, (f32, f32)>) {
    let mut graph = UnGraph::new_undirected();

    let mut node_layers: Vec<usize> = Vec::new();
    let mut node_positions: HashMap<usize, (f32, f32)> = HashMap::new();

    let mut all_layers: Vec<Vec<NodeIndex>> = Vec::new();

    for layer_idx in 0..7 {
        let side_length = (layer_idx + 1) * 2;
        let layer_nodes = create_square_layer(&mut graph, side_length);

        let positions = calculate_square_positions(side_length, layer_idx);

        for (i, &node) in layer_nodes.iter().enumerate() {
            node_layers.push(layer_idx);
            node_positions.insert(node.index(), positions[i]);
        }

        all_layers.push(layer_nodes);
    }

    for layer_idx in 1..7 {
        connect_layers(
            &mut graph,
            &all_layers[layer_idx - 1],
            &all_layers[layer_idx],
        );
    }

    (graph, node_layers, node_positions)
}

/// Creates a square layer of nodes with the given side length
fn create_square_layer(graph: &mut UnGraph<(), ()>, side_length: usize) -> Vec<NodeIndex> {
    let nodes_per_side = side_length as i32;
    let mut layer_nodes = Vec::new();

    // Top side (including top-left corner, excluding top-right corner to avoid duplication)
    for _ in 0..nodes_per_side {
        layer_nodes.push(graph.add_node(()));
    }

    // Right side (including top-right corner, excluding bottom-right corner)
    for _ in 1..nodes_per_side {
        layer_nodes.push(graph.add_node(()));
    }

    // Bottom side (including bottom-right corner, excluding bottom-left corner)
    for _ in 1..nodes_per_side {
        layer_nodes.push(graph.add_node(()));
    }

    // Left side (including bottom-left corner, excluding top-left corner to complete the loop)
    for _ in 1..nodes_per_side - 1 {
        layer_nodes.push(graph.add_node(()));
    }

    // Connect the nodes in a square
    let num_nodes = layer_nodes.len();
    for i in 0..num_nodes {
        let next = (i + 1) % num_nodes;
        graph.add_edge(layer_nodes[i], layer_nodes[next], ());
    }

    layer_nodes
}

/// Calculate normalized positions (0.0 to 1.0) for nodes in a square layer
/// side_length: number of nodes on each side
/// layer_idx: which layer this is (0 = innermost)
fn calculate_square_positions(side_length: usize, layer_idx: usize) -> Vec<(f32, f32)> {
    let mut positions = Vec::new();
    let nodes_per_side = side_length as f32;

    let max_layer = 6.0;
    let min_scale = 0.15;
    let max_scale = 0.95;

    let scale = min_scale + (max_scale - min_scale) * (layer_idx as f32 / max_layer);

    // Center position
    let center = 0.5;

    // Half the side length in normalized coordinates
    let half_size = scale / 2.0;

    // Top side (left to right)
    for i in 0..side_length {
        let t = i as f32 / (nodes_per_side - 1.0).max(1.0);
        let x = center - half_size + t * 2.0 * half_size;
        let y = center - half_size;
        positions.push((x, y));
    }

    // Right side (top to bottom, excluding top-right corner)
    for i in 1..side_length {
        let t = i as f32 / (nodes_per_side - 1.0).max(1.0);
        let x = center + half_size;
        let y = center - half_size + t * 2.0 * half_size;
        positions.push((x, y));
    }

    // Bottom side (right to left, excluding bottom-right corner)
    for i in 1..side_length {
        let t = i as f32 / (nodes_per_side - 1.0).max(1.0);
        let x = center + half_size - t * 2.0 * half_size;
        let y = center + half_size;
        positions.push((x, y));
    }

    // Left side (bottom to top, excluding both corners)
    for i in 1..side_length.saturating_sub(1) {
        let t = i as f32 / (nodes_per_side - 1.0).max(1.0);
        let x = center - half_size;
        let y = center + half_size - t * 2.0 * half_size;
        positions.push((x, y));
    }

    positions
}

/// Connects corners of the outer layer to corresponding corners of the inner layer
fn connect_layers(
    graph: &mut UnGraph<(), ()>,
    inner_layer: &[NodeIndex],
    outer_layer: &[NodeIndex],
) {
    let inner_corners = get_corner_indices(inner_layer.len());
    let outer_corners = get_corner_indices(outer_layer.len());

    for i in 0..4 {
        if inner_corners[i] < inner_layer.len() && outer_corners[i] < outer_layer.len() {
            graph.add_edge(
                inner_layer[inner_corners[i]],
                outer_layer[outer_corners[i]],
                (),
            );
        }
    }
}

/// Get the indices of the 4 corners of a square perimeter
fn get_corner_indices(total_nodes: usize) -> [usize; 4] {
    if total_nodes == 4 {
        // Special case for the smallest square (2x2)
        return [0, 1, 2, 3];
    }
    let side_segment = total_nodes / 4;

    [0, side_segment, 2 * side_segment, 3 * side_segment]
}
