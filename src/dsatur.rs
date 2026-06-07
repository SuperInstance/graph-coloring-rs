//! DSATUR (Degree of SATURation) graph coloring algorithm.
//!
//! DSATUR is a heuristic that colors vertices by choosing the uncolored vertex
//! with the highest saturation degree (number of distinct colors among its neighbors),
//! breaking ties by choosing the vertex with the highest degree.

use crate::graph::{Coloring, Graph};

/// Perform DSATUR coloring on the graph.
///
/// This heuristic often produces optimal or near-optimal colorings.
///
/// # Example
/// ```
/// use graph_coloring_rs::graph::Graph;
/// use graph_coloring_rs::dsatur::dsatur_coloring;
///
/// let g = Graph::cycle(5);
/// let coloring = dsatur_coloring(&g);
/// assert!(coloring.is_valid(&g));
/// ```
pub fn dsatur_coloring(graph: &Graph) -> Coloring {
    let n = graph.vertex_count();
    if n == 0 {
        return Coloring::new(vec![]);
    }

    let mut colors = vec![0; n];
    let mut colored = vec![false; n];

    // Find vertex with maximum degree to start
    let first = (0..n).max_by_key(|&v| graph.degree(v)).unwrap();
    colors[first] = 0;
    colored[first] = true;

    for _ in 1..n {
        // Compute saturation degree for each uncolored vertex
        let mut best_vertex = 0;
        let mut best_saturation = 0;
        let mut best_degree = 0;

        for v in 0..n {
            if colored[v] {
                continue;
            }

            let saturation = saturation_degree(graph, &colors, &colored, v);
            let degree = graph.degree(v);

            if saturation > best_saturation
                || (saturation == best_saturation && degree > best_degree)
            {
                best_saturation = saturation;
                best_degree = degree;
                best_vertex = v;
            }
        }

        // Color the best vertex with the smallest available color
        let color = smallest_available_color(graph, &colors, &colored, best_vertex);
        colors[best_vertex] = color;
        colored[best_vertex] = true;
    }

    Coloring::new(colors)
}

/// Compute the saturation degree of a vertex: the number of distinct colors
/// used by its already-colored neighbors.
fn saturation_degree(graph: &Graph, colors: &[usize], colored: &[bool], v: usize) -> usize {
    let mut neighbor_colors = std::collections::HashSet::new();
    for &u in graph.neighbors(v) {
        if colored[u] {
            neighbor_colors.insert(colors[u]);
        }
    }
    neighbor_colors.len()
}

/// Find the smallest available color for vertex `v`.
fn smallest_available_color(
    graph: &Graph,
    colors: &[usize],
    colored: &[bool],
    v: usize,
) -> usize {
    let n = graph.vertex_count();
    let mut used = vec![false; n];

    for &u in graph.neighbors(v) {
        if colored[u] {
            used[colors[u]] = true;
        }
    }

    let mut color = 0;
    while used[color] {
        color += 1;
    }
    color
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dsatur_empty() {
        let g = Graph::new(0);
        let c = dsatur_coloring(&g);
        assert!(c.is_valid(&g));
    }

    #[test]
    fn test_dsatur_single() {
        let g = Graph::new(1);
        let c = dsatur_coloring(&g);
        assert!(c.is_valid(&g));
        assert_eq!(c.num_colors(), 1);
    }

    #[test]
    fn test_dsatur_k4() {
        let g = Graph::complete(4);
        let c = dsatur_coloring(&g);
        assert!(c.is_valid(&g));
        assert_eq!(c.num_colors(), 4);
    }

    #[test]
    fn test_dsatur_bipartite() {
        let g = Graph::complete_bipartite(3, 3);
        let c = dsatur_coloring(&g);
        assert!(c.is_valid(&g));
        assert_eq!(c.num_colors(), 2);
    }

    #[test]
    fn test_dsatur_cycle_odd() {
        let g = Graph::cycle(5);
        let c = dsatur_coloring(&g);
        assert!(c.is_valid(&g));
        assert_eq!(c.num_colors(), 3);
    }

    #[test]
    fn test_dsatur_cycle_even() {
        let g = Graph::cycle(4);
        let c = dsatur_coloring(&g);
        assert!(c.is_valid(&g));
        assert_eq!(c.num_colors(), 2);
    }

    #[test]
    fn test_dsatur_no_edges() {
        let g = Graph::new(10);
        let c = dsatur_coloring(&g);
        assert!(c.is_valid(&g));
        assert_eq!(c.num_colors(), 1);
    }
}
