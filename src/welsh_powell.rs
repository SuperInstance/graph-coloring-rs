//! Welsh-Powell graph coloring algorithm.
//!
//! Welsh-Powell orders vertices by decreasing degree and greedily colors them.
//! This guarantees at most Δ(G) + 1 colors, where Δ is the maximum degree.

use crate::graph::{Coloring, Graph};
use crate::greedy::greedy_coloring_with_order;

/// Perform Welsh-Powell coloring on the graph.
///
/// Vertices are sorted by decreasing degree, then colored greedily.
///
/// # Example
/// ```
/// use graph_coloring_rs::graph::Graph;
/// use graph_coloring_rs::welsh_powell::welsh_powell_coloring;
///
/// let g = Graph::cycle(5);
/// let coloring = welsh_powell_coloring(&g);
/// assert!(coloring.is_valid(&g));
/// ```
pub fn welsh_powell_coloring(graph: &Graph) -> Coloring {
    let n = graph.vertex_count();
    if n == 0 {
        return Coloring::new(vec![]);
    }

    // Sort vertices by decreasing degree
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by_key(|&v| std::cmp::Reverse(graph.degree(v)));

    greedy_coloring_with_order(graph, order)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welsh_powell_empty() {
        let g = Graph::new(0);
        let c = welsh_powell_coloring(&g);
        assert!(c.is_valid(&g));
    }

    #[test]
    fn test_welsh_powell_k4() {
        let g = Graph::complete(4);
        let c = welsh_powell_coloring(&g);
        assert!(c.is_valid(&g));
        assert_eq!(c.num_colors(), 4);
    }

    #[test]
    fn test_welsh_powell_bipartite() {
        let g = Graph::complete_bipartite(3, 3);
        let c = welsh_powell_coloring(&g);
        assert!(c.is_valid(&g));
        assert_eq!(c.num_colors(), 2);
    }

    #[test]
    fn test_welsh_powell_cycle_odd() {
        let g = Graph::cycle(5);
        let c = welsh_powell_coloring(&g);
        assert!(c.is_valid(&g));
        assert_eq!(c.num_colors(), 3);
    }

    #[test]
    fn test_welsh_powell_cycle_even() {
        let g = Graph::cycle(4);
        let c = welsh_powell_coloring(&g);
        assert!(c.is_valid(&g));
        assert_eq!(c.num_colors(), 2);
    }

    #[test]
    fn test_welsh_powell_no_edges() {
        let g = Graph::new(10);
        let c = welsh_powell_coloring(&g);
        assert!(c.is_valid(&g));
        assert_eq!(c.num_colors(), 1);
    }

    #[test]
    fn test_welsh_powell_bound() {
        // Welsh-Powell should never use more than Δ(G) + 1 colors
        let g = Graph::cycle(7);
        let c = welsh_powell_coloring(&g);
        assert!(c.num_colors() <= g.max_degree() + 1);
    }
}
