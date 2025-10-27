use petgraph::visit::{EdgeRef, IntoEdgeReferences, IntoNodeReferences, NodeIndexable, NodeRef};

pub(crate) fn get_force_directed_position_map<G>(graph: &G) -> impl Fn(G::NodeId) -> (f32, f32) + '_
where
    G: IntoNodeReferences + IntoEdgeReferences + NodeIndexable,
{
    let node_count = graph.node_references().count();
    let mut positions = vec![(0.0f32, 0.0f32); graph.node_bound()];

    if node_count > 0 {
        // Initialize positions randomly in a circle to avoid pathological cases
        for (i, node_ref) in graph.node_references().enumerate() {
            let idx = graph.to_index(node_ref.id());
            let angle = (i as f32) / (node_count as f32) * std::f32::consts::TAU;
            positions[idx] = (angle.cos(), angle.sin());
        }

        // Simulation parameters
        let area = 1.0f32;
        let k = (area / (node_count as f32)).sqrt(); // Optimal distance between nodes
        let iterations = 100000;
        let initial_temp = 0.1f32;

        let edges: Vec<_> = graph
            .edge_references()
            .map(|edge| (graph.to_index(edge.source()), graph.to_index(edge.target())))
            .collect();

        let node_indices: Vec<_> = graph
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
                    let distance = (delta_x * delta_x + delta_y * delta_y).sqrt().max(0.01);

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
                let distance = (delta_x * delta_x + delta_y * delta_y).sqrt().max(0.01);

                let attraction = distance * distance / k;
                let force_x = (delta_x / distance) * attraction;
                let force_y = (delta_y / distance) * attraction;

                displacements[source_idx].0 -= force_x;
                displacements[source_idx].1 -= force_y;
                displacements[target_idx].0 += force_x;
                displacements[target_idx].1 += force_y;
            }

            // Apply displacements with cooling
            let temp = initial_temp * (1.0 - (iteration as f32) / (iterations as f32));
            for &idx in &node_indices {
                let disp_len = (displacements[idx].0 * displacements[idx].0
                    + displacements[idx].1 * displacements[idx].1)
                    .sqrt();

                if disp_len > 0.0 {
                    let limited_disp_len = disp_len.min(temp);
                    positions[idx].0 += (displacements[idx].0 / disp_len) * limited_disp_len;
                    positions[idx].1 += (displacements[idx].1 / disp_len) * limited_disp_len;
                }
            }
        }

        // Normalize positions
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
