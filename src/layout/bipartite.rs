//! Module containing functionality for the bipartite layout.
//!
//! The main function is [`bipartite_layout`], which returns a position map function that arranges
//! nodes in a bipartite layout.

use std::{collections::HashSet, hash::Hash};

use fixedbitset::FixedBitSet;
use petgraph::{
    graph::NodeIndex,
    visit::{IntoNeighbors, IntoNodeIdentifiers, NodeIndexable},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum NodePosition {
    Left,
    Right,
}

/// Returns a position map function that arranges nodes using a
/// [bipartite layout](https://en.wikipedia.org/wiki/Bipartite_graph).
/// The left partition is placed on the left side (x = 0.25) and the right partition on the
/// right side (x = 0.75).
///
/// If the `left` parameter is `None`, the function will attempt to determine the bipartition
/// using a breadth-first traversal. If the graph is not bipartite, the layout will still assign
/// nodes to left and right positions based on the traversal.
///
/// The returned position map is normalized to [0.0, 1.0].
pub fn bipartite_layout<'a, G>(
    graph: &'a G,
    left: Option<&HashSet<NodeIndex>>,
) -> impl Fn(G::NodeId) -> (f32, f32) + 'a
where
    G: IntoNodeIdentifiers + NodeIndexable + IntoNeighbors,
    G::NodeId: Hash + Eq,
{
    let mut visited: FixedBitSet = FixedBitSet::with_capacity(graph.node_bound());
    let mut dfs_stack = Vec::new();

    let (node_lr_positions, left_count, right_count) = if left.is_none() {
        let mut node_lr_positions = vec![None; graph.node_bound()];
        let mut left_count = 0;
        let mut right_count = 0;
        for outer_node_index in graph.node_identifiers().map(|id| graph.to_index(id)) {
            if !visited.contains(outer_node_index) {
                dfs_stack.push((outer_node_index, 0));
                while let Some((current_node_index, layer)) = dfs_stack.pop() {
                    if visited.contains(current_node_index) {
                        continue;
                    }
                    visited.insert(current_node_index);
                    if layer % 2 == 0 {
                        node_lr_positions.insert(current_node_index, Some(NodePosition::Left));
                        left_count += 1;
                    } else {
                        node_lr_positions.insert(current_node_index, Some(NodePosition::Right));
                        right_count += 1;
                    }
                    for neighbor in graph.neighbors(graph.from_index(current_node_index)) {
                        let neighbor_idx = graph.to_index(neighbor);
                        if !visited.contains(neighbor_idx) {
                            dfs_stack.push((neighbor_idx, layer + 1));
                        }
                    }
                }
            }
        }
        (node_lr_positions, left_count, right_count)
    } else {
        let left_nodes = left.expect("Left nodes should be Some by if case");
        let mut node_lr_positions = vec![None; graph.node_bound()];
        let left_count = left_nodes.len();
        let right_count = graph.node_identifiers().count() - left_count;

        for node_id in graph.node_identifiers() {
            let node_index = graph.to_index(node_id);
            if left_nodes.contains(&NodeIndex::new(node_index)) {
                node_lr_positions.insert(node_index, Some(NodePosition::Left));
            } else {
                node_lr_positions.insert(node_index, Some(NodePosition::Right));
            }
        }

        (node_lr_positions, left_count, right_count)
    };

    let mut node_positions: Vec<(f32, f32)> = vec![(0.0, 0.0); graph.node_bound()];

    let left_spacing = if left_count > 1 {
        1.0 / (left_count - 1) as f32
    } else {
        0.0
    };
    let right_spacing = if right_count > 1 {
        1.0 / (right_count - 1) as f32
    } else {
        0.0
    };

    let mut left_index = 0;
    let mut right_index = 0;

    for (node_index, position) in node_lr_positions.iter().enumerate() {
        if let Some(position) = position {
            match position {
                NodePosition::Left => {
                    let y = left_index as f32 * left_spacing;
                    node_positions.insert(node_index, (0.25, y));
                    left_index += 1;
                }
                NodePosition::Right => {
                    let y = right_index as f32 * right_spacing;
                    node_positions.insert(node_index, (0.75, y));
                    right_index += 1;
                }
            }
        }
    }

    move |node_id| {
        let index = graph.to_index(node_id);
        node_positions[index]
    }
}
