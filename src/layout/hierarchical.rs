use std::collections::{HashMap, HashSet};

use petgraph::visit::{IntoNeighborsDirected, NodeIndexable, NodeRef};

#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    /// Top to Bottom orientation.
    TopToBottom,
    /// Bottom to Top orientation.
    BottomToTop,
    /// Left to Right orientation.
    LeftToRight,
    /// Right to Left orientation.
    RightToLeft,
}

/// Returns a position map function that arranges nodes in a hierarchical layout.
pub(crate) fn get_hierarchical_position_map<G>(
    graph: &G,
    orientation: Orientation,
) -> impl Fn(G::NodeId) -> (f32, f32) + '_
where
    G: petgraph::visit::IntoNodeReferences
        + petgraph::visit::NodeIndexable
        + petgraph::visit::IntoNeighborsDirected,
{
    let mut visited = HashSet::new();
    let mut positions = HashMap::new();

    let mut next_col = 0;
    let roots = graph
        .node_references()
        .filter(|node_ref| {
            graph
                .neighbors_directed(node_ref.id(), petgraph::Direction::Incoming)
                .next()
                .is_none()
        })
        .map(|node_ref| NodeIndexable::to_index(&graph, node_ref.id()))
        .collect::<Vec<_>>();

    let mut max_row = 0;
    let mut max_col = 0;

    // Assign levels starting from root nodes
    for root in roots {
        if visited.contains(&root) {
            continue;
        }

        let (curr_max_col, curr_max_row) =
            assign_levels(graph, &mut visited, &mut positions, root, next_col, 0);

        max_row = max_row.max(curr_max_row);
        max_col = max_col.max(curr_max_col);
        next_col = curr_max_col + 1;
    }

    // We might not find any roots, especially in undirected graphs.
    let all_nodes_sorted_by_desc_deg = {
        let mut nodes: Vec<_> = graph.node_references().collect();
        nodes.sort_by_key(|n| {
            graph
                .neighbors_directed(n.id(), petgraph::Direction::Outgoing)
                .count()
        });
        nodes.reverse();
        nodes
    };
    for root in all_nodes_sorted_by_desc_deg
        .iter()
        .map(|node_ref| NodeIndexable::to_index(&graph, node_ref.id()))
    {
        if visited.contains(&root) {
            continue;
        }

        let (curr_max_col, curr_max_row) =
            assign_levels(graph, &mut visited, &mut positions, root, next_col, 0);

        max_row = max_row.max(curr_max_row);
        max_col = max_col.max(curr_max_col);
        next_col = curr_max_col + 1;
    }

    normalize_positions(&mut positions, max_col, max_row, orientation);

    move |node_id| positions[&NodeIndexable::to_index(&graph, node_id)]
}

fn assign_levels<G>(
    graph: &G,
    visited: &mut HashSet<usize>,
    positions: &mut HashMap<usize, (f32, f32)>,
    node: usize,
    start_col: usize,
    row: usize,
) -> (usize, usize)
where
    G: IntoNeighborsDirected + NodeIndexable,
{
    if visited.contains(&node) {
        return (start_col, row);
    }

    visited.insert(node);

    let children: Vec<usize> = graph
        .neighbors_directed(graph.from_index(node), petgraph::Direction::Outgoing)
        .map(|child| graph.to_index(child))
        .collect();

    let mut child_positions = Vec::new();
    let mut child_col = start_col;
    let mut max_col = start_col;
    let mut max_row = row;

    for child in children {
        if visited.contains(&child) {
            continue;
        }

        let (child_max_col, child_max_row) =
            assign_levels(graph, visited, positions, child, child_col, row + 1);

        child_positions.push(positions[&child]);

        max_col = max_col.max(child_max_col);
        max_row = max_row.max(child_max_row);
        child_col = child_max_col + 1;
    }

    let parent_col = if !child_positions.is_empty() {
        let leftmost = child_positions.first().unwrap().0;
        let rightmost = child_positions.last().unwrap().0;
        (leftmost + rightmost) / 2.0
    } else {
        start_col as f32
    };

    positions.insert(node, (parent_col, row as f32));

    (max_col, max_row)
}

fn normalize_positions(
    positions: &mut HashMap<usize, (f32, f32)>,
    max_col: usize,
    max_row: usize,
    orientation: Orientation,
) {
    let row_scale = if max_row > 0 {
        1.0 / max_row as f32
    } else {
        1.0
    };
    let col_scale = if max_col > 0 {
        1.0 / max_col as f32
    } else {
        1.0
    };

    for (_, (col, row)) in positions.iter_mut() {
        match orientation {
            Orientation::TopToBottom => {
                *row = *row * row_scale;
                *col = *col * col_scale;
            }
            Orientation::BottomToTop => {
                *row = 1.0 - (*row * row_scale);
                *col = *col * col_scale;
            }
            Orientation::LeftToRight => {
                let temp = *row;
                *row = *col * col_scale;
                *col = temp * row_scale;
            }
            Orientation::RightToLeft => {
                let temp = *row;
                *row = *col * col_scale;
                *col = 1.0 - (temp * row_scale);
            }
        }
    }
}
