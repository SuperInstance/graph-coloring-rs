//! Greedy sequential coloring.

use crate::graph::{Coloring, Graph, color_count};

/// Color the graph using the greedy (sequential) algorithm.
///
/// Processes vertices in order `0, 1, 2, ...` and assigns each vertex
/// the smallest available color not used by its neighbors.
///
/// Returns a valid coloring.
pub fn greedy_coloring(graph: &Graph) -> Coloring {
    let n = graph.vertex_count();
    let mut coloring = vec![0; n];
    let mut used = vec![false; n + 1];

    for v in 0..n {
        // Mark colors used by neighbors
        for &u in graph.neighbors(v) {
            if u < v {
                used[coloring[u]] = true;
            }
        }
        // Find smallest available color
        let mut c = 0;
        while used[c] {
            c += 1;
        }
        coloring[v] = c;
        // Reset used
        for &u in graph.neighbors(v) {
            if u < v {
                used[coloring[u]] = false;
            }
        }
    }

    coloring
}

/// Color the graph using greedy with a custom vertex ordering.
///
/// Processes vertices in the given order.
pub fn greedy_coloring_ordered(graph: &Graph, order: &[usize]) -> Coloring {
    let n = graph.vertex_count();
    let mut coloring = vec![0; n];
    let mut used = vec![false; n + 1];

    for &v in order {
        for &u in graph.neighbors(v) {
            if coloring[u] > 0 || u == order[0] {
                // Check if u was already colored (before v in order)
                let u_pos = order.iter().position(|&x| x == u).unwrap();
                let v_pos = order.iter().position(|&x| x == v).unwrap();
                if u_pos < v_pos {
                    used[coloring[u]] = true;
                }
            }
        }
        let mut c = 0;
        while used[c] {
            c += 1;
        }
        coloring[v] = c;
        for &u in graph.neighbors(v) {
            used[coloring[u]] = false;
        }
    }

    coloring
}

/// Greedy coloring with vertex ordering, returns number of colors used.
pub fn greedy_num_colors(graph: &Graph) -> usize {
    color_count(&greedy_coloring(graph))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::is_valid_coloring;

    #[test]
    fn test_greedy_complete_graph() {
        let g = Graph::complete(4);
        let c = greedy_coloring(&g);
        assert!(is_valid_coloring(&g, &c));
        assert_eq!(color_count(&c), 4); // K4 needs 4 colors
    }

    #[test]
    fn test_greedy_bipartite() {
        let g = Graph::complete_bipartite(3, 3);
        let c = greedy_coloring(&g);
        assert!(is_valid_coloring(&g, &c));
        assert_eq!(color_count(&c), 2); // Bipartite needs 2
    }

    #[test]
    fn test_greedy_cycle_even() {
        let g = Graph::cycle(4);
        let c = greedy_coloring(&g);
        assert!(is_valid_coloring(&g, &c));
        assert_eq!(color_count(&c), 2);
    }

    #[test]
    fn test_greedy_cycle_odd() {
        let g = Graph::cycle(5);
        let c = greedy_coloring(&g);
        assert!(is_valid_coloring(&g, &c));
        assert_eq!(color_count(&c), 3); // Odd cycle needs 3
    }

    #[test]
    fn test_greedy_path() {
        let g = Graph::path(5);
        let c = greedy_coloring(&g);
        assert!(is_valid_coloring(&g, &c));
        assert_eq!(color_count(&c), 2);
    }

    #[test]
    fn test_greedy_empty() {
        let g = Graph::new(0);
        let c = greedy_coloring(&g);
        assert!(c.is_empty());
    }

    #[test]
    fn test_greedy_single() {
        let g = Graph::new(1);
        let c = greedy_coloring(&g);
        assert!(is_valid_coloring(&g, &c));
        assert_eq!(color_count(&c), 1);
    }

    #[test]
    fn test_greedy_num_colors() {
        let g = Graph::complete(3);
        assert_eq!(greedy_num_colors(&g), 3);
    }
}
