//! Backtracking graph coloring algorithm.
//!
//! Finds a valid k-coloring using systematic backtracking, or determines
//! that no such coloring exists.

use crate::graph::{Coloring, Graph};

/// Try to find a valid coloring using at most `max_colors` colors.
///
/// Uses backtracking with pruning to find a valid coloring.
/// Returns `Some(Coloring)` if a valid coloring exists, `None` otherwise.
///
/// # Example
/// ```
/// use graph_coloring_rs::graph::Graph;
/// use graph_coloring_rs::backtrack::backtrack_coloring;
///
/// let g = Graph::complete(3);
/// let result = backtrack_coloring(&g, 2);
/// assert!(result.is_none()); // K3 needs 3 colors
///
/// let result = backtrack_coloring(&g, 3);
/// assert!(result.is_some()); // K3 can be 3-colored
/// ```
pub fn backtrack_coloring(graph: &Graph, max_colors: usize) -> Option<Coloring> {
    if graph.vertex_count() == 0 {
        return Some(Coloring::new(vec![]));
    }

    let n = graph.vertex_count();
    let mut colors = vec![0usize; n];

    if solve(graph, &mut colors, 0, max_colors) {
        Some(Coloring::new(colors))
    } else {
        None
    }
}

/// Recursive backtracking solver.
fn solve(graph: &Graph, colors: &mut [usize], vertex: usize, max_colors: usize) -> bool {
    if vertex >= graph.vertex_count() {
        return true;
    }

    for color in 0..max_colors {
        if is_safe(graph, colors, vertex, color) {
            colors[vertex] = color;
            if solve(graph, colors, vertex + 1, max_colors) {
                return true;
            }
            colors[vertex] = 0; // Backtrack
        }
    }

    false
}

/// Check if assigning `color` to `vertex` is safe (no conflict with colored neighbors).
fn is_safe(graph: &Graph, colors: &[usize], vertex: usize, color: usize) -> bool {
    for &neighbor in graph.neighbors(vertex) {
        if neighbor < vertex && colors[neighbor] == color {
            return false;
        }
    }
    true
}

/// Find the minimum number of colors needed (chromatic number) using backtracking.
///
/// Tries increasing numbers of colors starting from 1.
/// May be slow for large graphs.
pub fn chromatic_number_backtrack(graph: &Graph) -> usize {
    let n = graph.vertex_count();
    if n == 0 {
        return 0;
    }

    // Upper bound: max_degree + 1
    let upper = graph.max_degree() + 1;

    for k in 1..=upper {
        if backtrack_coloring(graph, k).is_some() {
            return k;
        }
    }

    upper
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backtrack_k4_needs_4() {
        let g = Graph::complete(4);
        assert!(backtrack_coloring(&g, 3).is_none());
        assert!(backtrack_coloring(&g, 4).is_some());
    }

    #[test]
    fn test_backtrack_bipartite_needs_2() {
        let g = Graph::complete_bipartite(3, 3);
        assert!(backtrack_coloring(&g, 1).is_none());
        assert!(backtrack_coloring(&g, 2).is_some());
    }

    #[test]
    fn test_backtrack_empty_graph() {
        let g = Graph::new(0);
        let result = backtrack_coloring(&g, 1);
        assert!(result.is_some());
    }

    #[test]
    fn test_backtrack_single_vertex() {
        let g = Graph::new(1);
        assert!(backtrack_coloring(&g, 1).is_some());
    }

    #[test]
    fn test_backtrack_cycle_odd() {
        let g = Graph::cycle(5);
        assert!(backtrack_coloring(&g, 2).is_none());
        assert!(backtrack_coloring(&g, 3).is_some());
    }

    #[test]
    fn test_backtrack_cycle_even() {
        let g = Graph::cycle(4);
        assert!(backtrack_coloring(&g, 2).is_some());
    }

    #[test]
    fn test_chromatic_number_k4() {
        let g = Graph::complete(4);
        assert_eq!(chromatic_number_backtrack(&g), 4);
    }

    #[test]
    fn test_chromatic_number_bipartite() {
        let g = Graph::complete_bipartite(2, 3);
        assert_eq!(chromatic_number_backtrack(&g), 2);
    }

    #[test]
    fn test_chromatic_number_path() {
        let g = Graph::path(5);
        assert_eq!(chromatic_number_backtrack(&g), 2);
    }

    #[test]
    fn test_chromatic_number_empty() {
        let g = Graph::new(0);
        assert_eq!(chromatic_number_backtrack(&g), 0);
    }
}
