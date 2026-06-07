//! Chromatic number computation and bounds.
//!
//! Provides exact and approximate methods for determining the chromatic number
//! of a graph (the minimum number of colors needed for a valid coloring).

use crate::backtrack::chromatic_number_backtrack;
use crate::graph::Graph;

/// Compute the exact chromatic number using backtracking.
///
/// This is an NP-hard problem and may be very slow for large graphs.
/// For small graphs (≤ 20 vertices), it's practical.
///
/// # Example
/// ```
/// use graph_coloring_rs::graph::Graph;
/// use graph_coloring_rs::chromatic::chromatic_number_exact;
///
/// let g = Graph::complete_bipartite(3, 3);
/// assert_eq!(chromatic_number_exact(&g), 2);
/// ```
pub fn chromatic_number_exact(graph: &Graph) -> usize {
    chromatic_number_backtrack(graph)
}

/// Lower bound on the chromatic number.
///
/// The clique number ω(G) is a lower bound: χ(G) ≥ ω(G).
/// We compute a greedy clique number as a practical lower bound.
pub fn chromatic_lower_bound(graph: &Graph) -> usize {
    if graph.vertex_count() == 0 {
        return 0;
    }

    // Find a maximal clique greedily
    let n = graph.vertex_count();
    let mut best_clique = 1;

    for start in 0..n {
        let mut clique = vec![start];

        'outer: for v in 0..n {
            if v == start {
                continue;
            }
            // Check if v is connected to all clique members
            for &c in &clique {
                if !graph.are_adjacent(v, c) {
                    continue 'outer;
                }
            }
            clique.push(v);
        }

        best_clique = best_clique.max(clique.len());
    }

    best_clique
}

/// Upper bound on the chromatic number.
///
/// Uses Brook's theorem: χ(G) ≤ Δ(G) + 1 for all graphs,
/// and χ(G) ≤ Δ(G) for connected graphs that aren't complete or odd cycles.
pub fn chromatic_upper_bound(graph: &Graph) -> usize {
    if graph.vertex_count() == 0 {
        return 0;
    }

    let delta = graph.max_degree();

    // For complete graphs: χ = n = Δ + 1
    if is_complete_graph(graph) {
        return graph.vertex_count();
    }

    // For odd cycles: χ = 3, Δ = 2, so Δ + 1 = 3 (no improvement)
    if is_odd_cycle(graph) {
        return 3;
    }

    // Brooks' theorem: χ(G) ≤ Δ(G) for non-complete, non-odd-cycle connected graphs
    delta.max(1)
}

/// Check if the graph is a complete graph.
fn is_complete_graph(graph: &Graph) -> bool {
    let n = graph.vertex_count();
    if n <= 1 {
        return true;
    }
    graph.edge_count() == n * (n - 1) / 2
}

/// Check if the graph is an odd cycle.
fn is_odd_cycle(graph: &Graph) -> bool {
    let n = graph.vertex_count();
    if n < 3 || n.is_multiple_of(2) {
        return false;
    }
    // Check every vertex has degree 2
    for v in 0..n {
        if graph.degree(v) != 2 {
            return false;
        }
    }
    // It's a 2-regular graph on odd number of vertices = odd cycle
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_k4() {
        let g = Graph::complete(4);
        assert_eq!(chromatic_number_exact(&g), 4);
    }

    #[test]
    fn test_exact_bipartite() {
        let g = Graph::complete_bipartite(2, 3);
        assert_eq!(chromatic_number_exact(&g), 2);
    }

    #[test]
    fn test_exact_cycle_odd() {
        let g = Graph::cycle(5);
        assert_eq!(chromatic_number_exact(&g), 3);
    }

    #[test]
    fn test_exact_cycle_even() {
        let g = Graph::cycle(4);
        assert_eq!(chromatic_number_exact(&g), 2);
    }

    #[test]
    fn test_exact_empty() {
        let g = Graph::new(0);
        assert_eq!(chromatic_number_exact(&g), 0);
    }

    #[test]
    fn test_exact_path() {
        let g = Graph::path(5);
        assert_eq!(chromatic_number_exact(&g), 2);
    }

    #[test]
    fn test_lower_bound_k4() {
        let g = Graph::complete(4);
        let lb = chromatic_lower_bound(&g);
        assert!(lb <= 4); // ω(G) ≤ χ(G) = 4
        assert!(lb >= 2); // K4 has clique of at least 2
    }

    #[test]
    fn test_lower_bound_bipartite() {
        let g = Graph::complete_bipartite(3, 3);
        let lb = chromatic_lower_bound(&g);
        assert_eq!(lb, 2); // Bipartite has clique number 2 (any edge)
    }

    #[test]
    fn test_upper_bound_k4() {
        let g = Graph::complete(4);
        let ub = chromatic_upper_bound(&g);
        assert_eq!(ub, 4);
    }

    #[test]
    fn test_upper_bound_bipartite() {
        let g = Graph::complete_bipartite(3, 3);
        let ub = chromatic_upper_bound(&g);
        assert!(ub >= 2);
    }

    #[test]
    fn test_bounds_consistency() {
        // For small graphs, lower bound ≤ exact ≤ upper bound
        let g = Graph::cycle(5);
        let lb = chromatic_lower_bound(&g);
        let exact = chromatic_number_exact(&g);
        let ub = chromatic_upper_bound(&g);
        assert!(lb <= exact);
        assert!(exact <= ub);
    }

    #[test]
    fn test_upper_bound_odd_cycle() {
        let g = Graph::cycle(5);
        assert_eq!(chromatic_upper_bound(&g), 3);
    }
}
