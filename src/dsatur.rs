//! DSATUR (Degree of SATURation) heuristic coloring.

use crate::graph::{Coloring, Graph, color_count};

/// Color the graph using the DSATUR algorithm.
///
/// At each step, selects the uncolored vertex with the highest saturation degree
/// (number of distinct colors among its neighbors). Breaks ties by choosing
/// the vertex with the highest degree.
///
/// Often produces optimal or near-optimal colorings.
pub fn dsatur_coloring(graph: &Graph) -> Coloring {
    let n = graph.vertex_count();
    if n == 0 {
        return vec![];
    }
    let mut coloring = vec![None; n];
    let mut sat_degree = vec![0usize; n]; // Number of distinct neighbor colors
    let mut neighbor_colors: Vec<Vec<bool>> = vec![vec![false; n + 1]; n];
    let mut colored = 0;

    while colored < n {
        // Pick vertex with highest saturation, break ties by degree
        let v = (0..n)
            .filter(|&v| coloring[v].is_none())
            .max_by_key(|&v| (sat_degree[v], graph.degree(v)))
            .unwrap();

        // Find smallest available color
        let mut c = 0;
        while neighbor_colors[v][c] {
            c += 1;
        }
        coloring[v] = Some(c);

        // Update saturation degrees of neighbors
        for &u in graph.neighbors(v) {
            if coloring[u].is_none() && !neighbor_colors[u][c] {
                neighbor_colors[u][c] = true;
                sat_degree[u] += 1;
            }
        }

        colored += 1;
    }

    coloring.into_iter().map(|c| c.unwrap()).collect()
}

/// DSATUR coloring, returns number of colors used.
pub fn dsatur_num_colors(graph: &Graph) -> usize {
    color_count(&dsatur_coloring(graph))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{is_valid_coloring, Graph};

    #[test]
    fn test_dsatur_complete() {
        let g = Graph::complete(5);
        let c = dsatur_coloring(&g);
        assert!(is_valid_coloring(&g, &c));
        assert_eq!(color_count(&c), 5);
    }

    #[test]
    fn test_dsatur_bipartite() {
        let g = Graph::complete_bipartite(3, 4);
        let c = dsatur_coloring(&g);
        assert!(is_valid_coloring(&g, &c));
        assert_eq!(color_count(&c), 2);
    }

    #[test]
    fn test_dsatur_cycle_even() {
        let g = Graph::cycle(6);
        let c = dsatur_coloring(&g);
        assert!(is_valid_coloring(&g, &c));
        assert_eq!(color_count(&c), 2);
    }

    #[test]
    fn test_dsatur_cycle_odd() {
        let g = Graph::cycle(5);
        let c = dsatur_coloring(&g);
        assert!(is_valid_coloring(&g, &c));
        assert_eq!(color_count(&c), 3);
    }

    #[test]
    fn test_dsatur_path() {
        let g = Graph::path(10);
        let c = dsatur_coloring(&g);
        assert!(is_valid_coloring(&g, &c));
        assert_eq!(color_count(&c), 2);
    }

    #[test]
    fn test_dsatur_empty() {
        let g = Graph::new(0);
        let c = dsatur_coloring(&g);
        assert!(c.is_empty());
    }

    #[test]
    fn test_dsatur_num_colors() {
        let g = Graph::complete(3);
        assert_eq!(dsatur_num_colors(&g), 3);
    }
}
