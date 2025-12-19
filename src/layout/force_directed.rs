//! Module containing functionality for the force-directed layout.
//!
//! The main function is [`force_directed_layout`], which returns a position map function that
//! arranges nodes in a force-directed layout.

use petgraph::visit::{EdgeRef, IntoEdgeReferences, IntoNodeReferences, NodeIndexable, NodeRef};
use std::collections::VecDeque;

/// Default number of iterations for the [`force_directed_layout`] function.
pub const DEFAULT_ITERATIONS: u32 = 1000;
/// Default initial temperature for the [`force_directed_layout`] function.
pub const DEFAULT_INITIAL_TEMPERATURE: f32 = 0.1;
/// Default number of iterations for the [`kamada_kawai_layout`] function.
pub const DEFAULT_KK_ITERATIONS: u32 = 300;
/// Default spring constant for the [`kamada_kawai_layout`] function.
pub const DEFAULT_SPRING_CONSTANT: f32 = 1.0;
/// Default ideal edge length for the [`kamada_kawai_layout`] function.
pub const DEFAULT_IDEAL_EDGE_LENGTH: f32 = 1.0;
const CLIPPING_VALUE: f32 = 0.01;
const EPSILON: f32 = 1e-6;

/// Returns a position map function that arranges nodes using a force-directed layout. The specific
/// algorithm used is the Fruchterman-Reingold algorithm, see [Reference](#reference).
///
/// The returned position map is normalized to [0.0, 1.0].
///
/// # Reference
///
/// This is an implementation of the Fruchterman-Reingold force-directed algorithm as presented in
/// the original paper:
///
/// Fruchterman, T. M. J., Reingold, E. M. (1991). Graph drawing by force-directed placement
/// <https://doi.org/10.1002/spe.4380211102>.
pub fn force_directed_layout<G>(
    graph: &G,
    iterations: u32,
    inital_temperature: f32,
) -> impl Fn(G::NodeId) -> (f32, f32) + '_
where
    G: IntoNodeReferences + IntoEdgeReferences + NodeIndexable,
{
    let node_count = graph.node_references().count();
    let mut positions = vec![(0.0f32, 0.0f32); graph.node_bound()];

    if node_count > 0 {
        // Initialize positions randomly
        let mut rng = fastrand::Rng::new();

        for node_ref in graph.node_references() {
            let x = rng.f32();
            let y = rng.f32();
            let idx = graph.to_index(node_ref.id());
            positions[idx] = (x, y);
        }

        // Simulation parameters
        let k = (1.0 / (node_count as f32)).sqrt();

        let edges: Vec<(usize, usize)> = graph
            .edge_references()
            .map(|edge| (graph.to_index(edge.source()), graph.to_index(edge.target())))
            .collect();

        let node_indices: Vec<usize> = graph
            .node_references()
            .map(|node_ref| graph.to_index(node_ref.id()))
            .collect();

        for iteration in 0..iterations {
            let mut displacements = vec![(0.0f32, 0.0f32); graph.node_bound()];

            // Calculate repulsive forces between all pairs of nodes
            for i in 0..node_indices.len() {
                for j in (i + 1)..node_indices.len() {
                    let idx_i = node_indices[i];
                    let idx_j = node_indices[j];

                    let delta_x = positions[idx_i].0 - positions[idx_j].0;
                    let delta_y = positions[idx_i].1 - positions[idx_j].1;
                    let distance = (delta_x * delta_x + delta_y * delta_y)
                        .sqrt()
                        .max(CLIPPING_VALUE);

                    // Repulsive force: f_r = k^2 / d
                    let repulsion = k * k / distance;
                    let force_x = (delta_x / distance) * repulsion;
                    let force_y = (delta_y / distance) * repulsion;

                    displacements[idx_i].0 += force_x;
                    displacements[idx_i].1 += force_y;
                    displacements[idx_j].0 -= force_x;
                    displacements[idx_j].1 -= force_y;
                }
            }

            // Calculate attractive forces along edges
            for &(source_idx, target_idx) in &edges {
                let delta_x = positions[source_idx].0 - positions[target_idx].0;
                let delta_y = positions[source_idx].1 - positions[target_idx].1;
                let distance = (delta_x * delta_x + delta_y * delta_y)
                    .sqrt()
                    .max(CLIPPING_VALUE);

                let attraction = distance * distance / k;
                let force_x = (delta_x / distance) * attraction;
                let force_y = (delta_y / distance) * attraction;

                displacements[source_idx].0 -= force_x;
                displacements[source_idx].1 -= force_y;
                displacements[target_idx].0 += force_x;
                displacements[target_idx].1 += force_y;
            }

            // Apply displacements with cooling
            let curr_temp =
                inital_temperature - (0.1 * iteration as f32) / ((iterations + 1) as f32);
            for &idx in &node_indices {
                let disp_len = (displacements[idx].0 * displacements[idx].0
                    + displacements[idx].1 * displacements[idx].1)
                    .sqrt();

                if disp_len > 0.0 {
                    let limited_disp_len = disp_len.min(curr_temp);
                    positions[idx].0 += (displacements[idx].0 / disp_len) * limited_disp_len;
                    positions[idx].1 += (displacements[idx].1 / disp_len) * limited_disp_len;
                }
            }
        }

        // Normalize positions to [0.0, 1.0]
        if !positions.is_empty() {
            let mut min_x = f32::INFINITY;
            let mut max_x = f32::NEG_INFINITY;
            let mut min_y = f32::INFINITY;
            let mut max_y = f32::NEG_INFINITY;

            for &idx in &node_indices {
                min_x = min_x.min(positions[idx].0);
                max_x = max_x.max(positions[idx].0);
                min_y = min_y.min(positions[idx].1);
                max_y = max_y.max(positions[idx].1);
            }

            let range_x = max_x - min_x;
            let range_y = max_y - min_y;

            if range_x > 0.0 && range_y > 0.0 {
                for &idx in &node_indices {
                    positions[idx].0 = (positions[idx].0 - min_x) / range_x;
                    positions[idx].1 = (positions[idx].1 - min_y) / range_y;
                }
            } else if range_x > 0.0 {
                for &idx in &node_indices {
                    positions[idx].0 = (positions[idx].0 - min_x) / range_x;
                    positions[idx].1 = 0.5;
                }
            } else if range_y > 0.0 {
                for &idx in &node_indices {
                    positions[idx].0 = 0.5;
                    positions[idx].1 = (positions[idx].1 - min_y) / range_y;
                }
            } else {
                for &idx in &node_indices {
                    positions[idx] = (0.5, 0.5);
                }
            }
        }
    }

    move |node_id| positions[NodeIndexable::to_index(&graph, node_id)]
}

/// Returns a position map function that arranges nodes using the Kamada-Kawai force-directed layout.
///
/// The Kamada-Kawai algorithm uses spring forces based on graph-theoretic distances (shortest paths)
/// to position nodes. It minimizes the energy of the system by iteratively adjusting node positions
/// to match ideal distances derived from the shortest path lengths between nodes.
///
/// The returned position map is normalized to [0.0, 1.0].
///
/// # Parameters
///
/// * `graph` - The graph to lay out
/// * `iterations` - Maximum number of iterations for energy minimization
/// * `spring_constant` - Spring constant for the spring forces (K in the paper)
/// * `ideal_edge_length` - The ideal length for an edge of length 1 (L in the paper)
///
/// # Reference
///
/// This is an implementation of the Kamada-Kawai force-directed algorithm as presented in
/// the original paper:
///
/// Kamada, T., Kawai, S. (1989). An algorithm for drawing general undirected graphs.
/// Information Processing Letters, 31(1), 7-15.
/// <https://doi.org/10.1016/0020-0190(89)90102-6>
pub fn kamada_kawai_layout<G>(
    graph: &G,
    iterations: u32,
    spring_constant: f32,
    ideal_edge_length: f32,
) -> impl Fn(G::NodeId) -> (f32, f32) + '_
where
    G: IntoNodeReferences + IntoEdgeReferences + NodeIndexable,
{
    let node_count = graph.node_references().count();
    let mut positions = vec![(0.0f32, 0.0f32); graph.node_bound()];

    if node_count > 1 {
        // Collect node indices
        let node_indices: Vec<usize> = graph
            .node_references()
            .map(|node_ref| graph.to_index(node_ref.id()))
            .collect();

        // Compute all-pairs shortest path distances using BFS
        let distances = compute_shortest_paths(graph, &node_indices);

        // Find the maximum distance to compute L (as per the paper)
        let mut max_distance = 0.0f32;
        for row in &distances {
            for &d in row {
                if d != f32::INFINITY && d > max_distance {
                    max_distance = d;
                }
            }
        }

        // Compute L = L0 / max(d_ij)
        // L0 is the desired diameter, we'll use ideal_edge_length as L0
        // This ensures the layout is properly scaled
        let l_value = if max_distance > 0.0 {
            ideal_edge_length / max_distance
        } else {
            ideal_edge_length
        };

        // Initialize positions on a circle as per the Kamada-Kawai paper:
        // "initialization by which the particles are placed on the nodes of the
        // regular n-polygon circumscribed by a circle whose diameter is L0"
        let radius = l_value / 2.0; // radius = diameter / 2
        let center = 0.5; // Center of our [0, 1] normalized space

        for (i, &node_idx) in node_indices.iter().enumerate() {
            // Distribute nodes evenly around the circle
            let angle = (i as f32) / (node_count as f32) * std::f32::consts::TAU
                - std::f32::consts::FRAC_PI_2; // Start from top
            let x = center + radius * angle.cos();
            let y = center + radius * angle.sin();
            positions[node_idx] = (x, y);
        }

        // Compute ideal distances (l_ij = L * d_ij where d_ij is the graph distance)
        let ideal_distances: Vec<Vec<f32>> = distances
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&d| {
                        if d == f32::INFINITY {
                            // For disconnected nodes, use a large distance
                            ideal_edge_length * (node_count as f32)
                        } else {
                            ideal_edge_length * d
                        }
                    })
                    .collect()
            })
            .collect();

        // Compute spring strengths (k_ij = K / d_ij^2)
        let spring_strengths: Vec<Vec<f32>> = distances
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&d| {
                        if d == 0.0 || d == f32::INFINITY {
                            0.0
                        } else {
                            spring_constant / (d * d)
                        }
                    })
                    .collect()
            })
            .collect();

        // Iteratively minimize energy
        // Use the energy threshold approach: reduce energy of max vertex until all below threshold
        const ENERGY_THRESHOLD: f32 = 1e-2;
        const MAX_STEADY_ENERGY_ITERS: u32 = 50;

        let mut steady_energy_count = 0;
        let mut iteration_count = 0;
        let mut max_vertex_energy = find_max_vertex_energy(
            &positions,
            &node_indices,
            &ideal_distances,
            &spring_strengths,
        );

        while max_vertex_energy.1 > ENERGY_THRESHOLD
            && steady_energy_count < MAX_STEADY_ENERGY_ITERS
            && iteration_count < iterations
        {
            let max_node_i = max_vertex_energy.0;
            let max_node_idx = node_indices[max_node_i];

            // Optimize position of the vertex with maximum energy
            optimize_node_position(
                max_node_idx,
                max_node_i,
                &mut positions,
                &node_indices,
                &ideal_distances,
                &spring_strengths,
            );

            let prev_max_energy = max_vertex_energy.1;
            max_vertex_energy = find_max_vertex_energy(
                &positions,
                &node_indices,
                &ideal_distances,
                &spring_strengths,
            );

            // Check if energy has stalled
            if (max_vertex_energy.1 - prev_max_energy).abs() < 1e-20 {
                steady_energy_count += 1;
            } else {
                steady_energy_count = 0;
            }

            iteration_count += 1;
        }

        // Normalize positions to [0.0, 1.0]
        normalize_positions(&mut positions, &node_indices);
    } else if node_count == 1 {
        // Single node at center
        let node_ref = graph.node_references().next().unwrap();
        let idx = graph.to_index(node_ref.id());
        positions[idx] = (0.5, 0.5);
    }

    move |node_id| positions[NodeIndexable::to_index(&graph, node_id)]
}

/// Compute all-pairs shortest path distances using BFS
fn compute_shortest_paths<G>(graph: &G, node_indices: &[usize]) -> Vec<Vec<f32>>
where
    G: IntoEdgeReferences + NodeIndexable,
{
    let n = node_indices.len();
    let mut distances = vec![vec![f32::INFINITY; n]; n];

    // Build adjacency list
    let mut adj_list: Vec<Vec<usize>> = vec![vec![]; graph.node_bound()];
    for edge in graph.edge_references() {
        let u = graph.to_index(edge.source());
        let v = graph.to_index(edge.target());
        adj_list[u].push(v);
        adj_list[v].push(u);
    }

    // Run BFS from each node
    for (i, &start_idx) in node_indices.iter().enumerate() {
        distances[i][i] = 0.0;
        let mut queue = VecDeque::new();
        let mut visited = vec![false; graph.node_bound()];

        queue.push_back((start_idx, 0));
        visited[start_idx] = true;

        while let Some((current, dist)) = queue.pop_front() {
            for &neighbor in &adj_list[current] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back((neighbor, dist + 1));

                    // Find the index in node_indices
                    if let Some(j) = node_indices.iter().position(|&idx| idx == neighbor) {
                        distances[i][j] = (dist + 1) as f32;
                    }
                }
            }
        }
    }

    distances
}

/// Find the vertex with maximum energy and return its index and energy value
fn find_max_vertex_energy(
    positions: &[(f32, f32)],
    node_indices: &[usize],
    ideal_distances: &[Vec<f32>],
    spring_strengths: &[Vec<f32>],
) -> (usize, f32) {
    let mut max_energy = -1.0;
    let mut max_energy_idx = 0;

    for (i, &node_idx) in node_indices.iter().enumerate() {
        let energy = compute_vertex_energy(
            node_idx,
            i,
            positions,
            node_indices,
            ideal_distances,
            spring_strengths,
        );
        if energy > max_energy {
            max_energy = energy;
            max_energy_idx = i;
        }
    }

    (max_energy_idx, max_energy)
}

/// Compute the energy of a single vertex (magnitude of its energy gradient)
fn compute_vertex_energy(
    node_idx: usize,
    node_i: usize,
    positions: &[(f32, f32)],
    node_indices: &[usize],
    ideal_distances: &[Vec<f32>],
    spring_strengths: &[Vec<f32>],
) -> f32 {
    let mut x_energy = 0.0;
    let mut y_energy = 0.0;

    let pos_i = positions[node_idx];

    for (j, &other_idx) in node_indices.iter().enumerate() {
        if node_i == j {
            continue;
        }

        let pos_j = positions[other_idx];
        let dx = pos_i.0 - pos_j.0;
        let dy = pos_i.1 - pos_j.1;
        let dist = ((dx * dx + dy * dy).sqrt()).max(CLIPPING_VALUE);

        let k_ij = spring_strengths[node_i][j];
        let l_ij = ideal_distances[node_i][j];

        let factor = k_ij * (1.0 - l_ij / dist);
        x_energy += factor * dx;
        y_energy += factor * dy;
    }

    (x_energy * x_energy + y_energy * y_energy).sqrt()
}

/// Optimize the position of a single node using Newton-Raphson method
fn optimize_node_position(
    node_idx: usize,
    node_i: usize,
    positions: &mut [(f32, f32)],
    node_indices: &[usize],
    ideal_distances: &[Vec<f32>],
    spring_strengths: &[Vec<f32>],
) {
    // Use Newton-Raphson method as described in the Kamada-Kawai paper
    // This computes the Hessian matrix (second-order partial derivatives)
    // and uses it to find the optimal position update
    const MAX_INNER_ITERATIONS: u32 = 50;

    for _ in 0..MAX_INNER_ITERATIONS {
        let pos_i = positions[node_idx];

        // First-order partial derivatives (energy gradient)
        let mut x_energy = 0.0;
        let mut y_energy = 0.0;

        // Second-order partial derivatives (Hessian matrix components)
        let mut xx_energy = 0.0;
        let mut xy_energy = 0.0;
        let mut yy_energy = 0.0;

        for (j, &other_idx) in node_indices.iter().enumerate() {
            if node_i == j {
                continue;
            }

            let pos_j = positions[other_idx];
            let dx = pos_i.0 - pos_j.0;
            let dy = pos_i.1 - pos_j.1;
            let dist = ((dx * dx + dy * dy).sqrt()).max(CLIPPING_VALUE);
            let cubed_dist = dist * dist * dist;

            let k_ij = spring_strengths[node_i][j];
            let l_ij = ideal_distances[node_i][j];

            // First-order partial derivatives
            x_energy += dx * k_ij * (1.0 - l_ij / dist);
            y_energy += dy * k_ij * (1.0 - l_ij / dist);

            // Second-order partial derivatives (Hessian)
            xy_energy += k_ij * l_ij * dx * dy / cubed_dist;
            xx_energy += k_ij * (1.0 - l_ij * dy * dy / cubed_dist);
            yy_energy += k_ij * (1.0 - l_ij * dx * dx / cubed_dist);
        }

        // Check if energy is below threshold
        let energy = (x_energy * x_energy + y_energy * y_energy).sqrt();
        if energy < EPSILON {
            break;
        }

        // yx_energy = xy_energy (Hessian is symmetric)
        let yx_energy = xy_energy;

        // Compute determinant of Hessian
        let denom = xx_energy * yy_energy - xy_energy * yx_energy;

        // Avoid division by zero
        if denom.abs() < EPSILON {
            break;
        }

        // Newton-Raphson update: position += H^(-1) * gradient
        // where H^(-1) is the inverse of the Hessian matrix
        let delta_x = (xy_energy * y_energy - yy_energy * x_energy) / denom;
        let delta_y = (yx_energy * x_energy - xx_energy * y_energy) / denom;

        positions[node_idx].0 += delta_x;
        positions[node_idx].1 += delta_y;
    }
}

/// Normalize positions to [0.0, 1.0] range
fn normalize_positions(positions: &mut [(f32, f32)], node_indices: &[usize]) {
    if node_indices.is_empty() {
        return;
    }

    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    for &idx in node_indices {
        min_x = min_x.min(positions[idx].0);
        max_x = max_x.max(positions[idx].0);
        min_y = min_y.min(positions[idx].1);
        max_y = max_y.max(positions[idx].1);
    }

    let range_x = max_x - min_x;
    let range_y = max_y - min_y;

    if range_x > 0.0 && range_y > 0.0 {
        for &idx in node_indices {
            positions[idx].0 = (positions[idx].0 - min_x) / range_x;
            positions[idx].1 = (positions[idx].1 - min_y) / range_y;
        }
    } else if range_x > 0.0 {
        for &idx in node_indices {
            positions[idx].0 = (positions[idx].0 - min_x) / range_x;
            positions[idx].1 = 0.5;
        }
    } else if range_y > 0.0 {
        for &idx in node_indices {
            positions[idx].0 = 0.5;
            positions[idx].1 = (positions[idx].1 - min_y) / range_y;
        }
    } else {
        for &idx in node_indices {
            positions[idx] = (0.5, 0.5);
        }
    }
}
