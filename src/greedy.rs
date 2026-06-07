//! Greedy graph coloring algorithm.
//!
//! Colors vertices in order 0, 1, 2, ..., assigning the smallest available color
//! that doesn't conflict with already-colored neighbors.

use crate::graph::{Coloring, Graph};

/// Perform greedy coloring on the graph.
///
/// Colors vertices in natural order (0, 1, 2, ...).
/// For each vertex, assigns the smallest non-negative color not used by its neighbors.
///
/// # Example
/// ```
/// use graph_coloring_rs::graph::Graph;
/// use graph_coloring_rs::greedy::greedy_coloring;
///
/// let g = Graph::cycle(5);
/// let coloring = greedy_coloring(&g);
/// assert!(coloring.is_valid(&g));
/// assert_eq!(coloring.num_colors(), 3); // C5 needs 3 colors
/// ```
pub fn greedy_coloring(graph: &Graph) -> Coloring {
    greedy_coloring_with_order(graph, (0..graph.vertex_count()).collect())
}

/// Perform greedy coloring with a specified vertex ordering.
///
/// # Arguments
/// * `graph` - The graph to color
/// * `order` - The order in which to process vertices
pub fn greedy_coloring_with_order(graph: &Graph, order: Vec<usize>) -> Coloring {
    let n = graph.vertex_count();
    let mut colors = vec![None; n];
    let mut used = vec![false; n + 1];

    for v in order {
        // Mark colors used by already-colored neighbors
        for &u in graph.neighbors(v) {
            if let Some(c) = colors[u] {
                used[c] = true;
            }
        }

        // Find the smallest available color
        let color = used.iter().position(|&u| !u).unwrap();
        colors[v] = Some(color);

        // Reset used array
        for &u in graph.neighbors(v) {
            if let Some(c) = colors[u] {
                used[c] = false;
            }
        }
    }

    Coloring::new(colors.iter().map(|c| c.unwrap_or(0)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greedy_empty_graph() {
        let g = Graph::new(0);
        let c = greedy_coloring(&g);
        assert!(c.is_valid(&g));
    }

    #[test]
    fn test_greedy_single_vertex() {
        let g = Graph::new(1);
        let c = greedy_coloring(&g);
        assert!(c.is_valid(&g));
        assert_eq!(c.num_colors(), 1);
    }

    #[test]
    fn test_greedy_k4() {
        let g = Graph::complete(4);
        let c = greedy_coloring(&g);
        assert!(c.is_valid(&g));
        assert_eq!(c.num_colors(), 4);
    }

    #[test]
    fn test_greedy_bipartite() {
        let g = Graph::complete_bipartite(3, 3);
        let c = greedy_coloring(&g);
        assert!(c.is_valid(&g));
        assert_eq!(c.num_colors(), 2);
    }

    #[test]
    fn test_greedy_cycle_even() {
        let g = Graph::cycle(4);
        let c = greedy_coloring(&g);
        assert!(c.is_valid(&g));
        assert_eq!(c.num_colors(), 2);
    }

    #[test]
    fn test_greedy_cycle_odd() {
        let g = Graph::cycle(5);
        let c = greedy_coloring(&g);
        assert!(c.is_valid(&g));
        assert_eq!(c.num_colors(), 3);
    }

    #[test]
    fn test_greedy_path() {
        let g = Graph::path(5);
        let c = greedy_coloring(&g);
        assert!(c.is_valid(&g));
        assert_eq!(c.num_colors(), 2);
    }

    #[test]
    fn test_greedy_no_edges() {
        let g = Graph::new(10);
        let c = greedy_coloring(&g);
        assert!(c.is_valid(&g));
        assert_eq!(c.num_colors(), 1);
    }

    #[test]
    fn test_greedy_custom_order() {
        let g = Graph::complete(4);
        let c = greedy_coloring_with_order(&g, vec![3, 2, 1, 0]);
        assert!(c.is_valid(&g));
        assert_eq!(c.num_colors(), 4);
    }
}
