//! Backtracking exact coloring — finds the chromatic number.

use crate::graph::{Coloring, Graph};

/// Find the minimum number of colors needed using backtracking.
///
/// Returns a coloring with the chromatic number of colors, or `None` if
/// the graph cannot be colored (shouldn't happen for finite graphs).
///
/// Warning: exponential time complexity. Suitable for small graphs.
pub fn backtrack_coloring(graph: &Graph, max_colors: usize) -> Option<Coloring> {
    let n = graph.vertex_count();
    if n == 0 {
        return Some(vec![]);
    }
    let mut coloring = vec![0; n];
    if solve(graph, &mut coloring, 0, max_colors) {
        Some(coloring)
    } else {
        None
    }
}

fn solve(graph: &Graph, coloring: &mut [usize], vertex: usize, max_colors: usize) -> bool {
    if vertex == graph.vertex_count() {
        return true;
    }
    for c in 0..max_colors {
        if is_safe(graph, coloring, vertex, c) {
            coloring[vertex] = c;
            if solve(graph, coloring, vertex + 1, max_colors) {
                return true;
            }
        }
    }
    coloring[vertex] = 0;
    false
}

fn is_safe(graph: &Graph, coloring: &[usize], vertex: usize, color: usize) -> bool {
    for &neighbor in graph.neighbors(vertex) {
        if neighbor < vertex && coloring[neighbor] == color {
            return false;
        }
    }
    true
}

/// Find the exact chromatic number by trying increasing color counts.
pub fn chromatic_number_backtrack(graph: &Graph) -> usize {
    let n = graph.vertex_count();
    if n == 0 {
        return 0;
    }
    for k in 1..=n {
        if backtrack_coloring(graph, k).is_some() {
            return k;
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{color_count, is_valid_coloring};

    #[test]
    fn test_backtrack_k4() {
        let g = Graph::complete(4);
        let c = backtrack_coloring(&g, 4).unwrap();
        assert!(is_valid_coloring(&g, &c));
        assert_eq!(color_count(&c), 4);
    }

    #[test]
    fn test_backtrack_k4_fails_with_3() {
        let g = Graph::complete(4);
        assert!(backtrack_coloring(&g, 3).is_none());
    }

    #[test]
    fn test_backtrack_bipartite() {
        let g = Graph::complete_bipartite(3, 3);
        let c = backtrack_coloring(&g, 2).unwrap();
        assert!(is_valid_coloring(&g, &c));
    }

    #[test]
    fn test_backtrack_cycle_odd() {
        let g = Graph::cycle(5);
        let c = backtrack_coloring(&g, 3).unwrap();
        assert!(is_valid_coloring(&g, &c));
    }

    #[test]
    fn test_chromatic_k3() {
        let g = Graph::complete(3);
        assert_eq!(chromatic_number_backtrack(&g), 3);
    }

    #[test]
    fn test_chromatic_k4() {
        let g = Graph::complete(4);
        assert_eq!(chromatic_number_backtrack(&g), 4);
    }

    #[test]
    fn test_chromatic_bipartite() {
        let g = Graph::complete_bipartite(2, 3);
        assert_eq!(chromatic_number_backtrack(&g), 2);
    }

    #[test]
    fn test_chromatic_path() {
        let g = Graph::path(5);
        assert_eq!(chromatic_number_backtrack(&g), 2);
    }

    #[test]
    fn test_chromatic_empty() {
        let g = Graph::new(0);
        assert_eq!(chromatic_number_backtrack(&g), 0);
    }

    #[test]
    fn test_backtrack_single() {
        let g = Graph::new(1);
        assert_eq!(chromatic_number_backtrack(&g), 1);
    }
}
