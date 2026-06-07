//! Welsh-Powell ordering-based greedy coloring.

use crate::graph::{Coloring, Graph, color_count};

/// Color the graph using the Welsh-Powell algorithm.
///
/// Sorts vertices by decreasing degree, then colors them in order,
/// assigning the smallest available color that doesn't conflict with
/// previously colored neighbors.
pub fn welsh_powell_coloring(graph: &Graph) -> Coloring {
    let n = graph.vertex_count();
    if n == 0 {
        return vec![];
    }

    // Sort vertices by decreasing degree
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by_key(|&v| std::cmp::Reverse(graph.degree(v)));

    let mut coloring = vec![0; n];
    let mut colored = vec![false; n];

    for &v in &order {
        if colored[v] {
            continue;
        }
        // Find smallest available color for v
        let mut c = 0;
        loop {
            let mut conflict = false;
            for &u in graph.neighbors(v) {
                if colored[u] && coloring[u] == c {
                    conflict = true;
                    break;
                }
            }
            if !conflict {
                break;
            }
            c += 1;
        }
        coloring[v] = c;
        colored[v] = true;

        // Try to color uncolored non-adjacent vertices with the same color
        for &w in &order {
            if colored[w] || w == v {
                continue;
            }
            // Check if w can take color c
            let mut conflict = false;
            for &u in graph.neighbors(w) {
                if colored[u] && coloring[u] == c {
                    conflict = true;
                    break;
                }
            }
            if !conflict {
                coloring[w] = c;
                colored[w] = true;
            }
        }
    }

    coloring
}

/// Welsh-Powell coloring, returns number of colors used.
pub fn welsh_powell_num_colors(graph: &Graph) -> usize {
    color_count(&welsh_powell_coloring(graph))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::is_valid_coloring;

    #[test]
    fn test_wp_complete() {
        let g = Graph::complete(4);
        let c = welsh_powell_coloring(&g);
        assert!(is_valid_coloring(&g, &c));
        assert_eq!(color_count(&c), 4);
    }

    #[test]
    fn test_wp_bipartite() {
        let g = Graph::complete_bipartite(3, 3);
        let c = welsh_powell_coloring(&g);
        assert!(is_valid_coloring(&g, &c));
        assert_eq!(color_count(&c), 2);
    }

    #[test]
    fn test_wp_cycle() {
        let g = Graph::cycle(6);
        let c = welsh_powell_coloring(&g);
        assert!(is_valid_coloring(&g, &c));
    }

    #[test]
    fn test_wp_path() {
        let g = Graph::path(5);
        let c = welsh_powell_coloring(&g);
        assert!(is_valid_coloring(&g, &c));
        assert_eq!(color_count(&c), 2);
    }

    #[test]
    fn test_wp_empty() {
        let g = Graph::new(0);
        assert!(welsh_powell_coloring(&g).is_empty());
    }

    #[test]
    fn test_wp_num_colors() {
        let g = Graph::complete(3);
        assert_eq!(welsh_powell_num_colors(&g), 3);
    }
}
