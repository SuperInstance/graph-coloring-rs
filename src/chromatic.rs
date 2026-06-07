//! Chromatic number bounds and checks.

use crate::graph::Graph;

/// Compute a lower bound on the chromatic number.
///
/// The clique number (size of the largest clique) is a lower bound.
/// For a complete graph K_n, this returns n.
/// Uses a simple heuristic: the maximum degree + 1 over all vertices.
pub fn lower_bound(graph: &Graph) -> usize {
    if graph.vertex_count() == 0 {
        return 0;
    }
    // Simple: check if the graph has any edges. If so, χ ≥ 2.
    // Better: find the maximum clique size (expensive), or use ω ≥ max clique found.
    // We use a greedy clique detection as a heuristic.
    find_max_clique_lower_bound(graph)
}

fn find_max_clique_lower_bound(graph: &Graph) -> usize {
    let n = graph.vertex_count();
    if n == 0 {
        return 0;
    }
    let mut best = 1;
    for start in 0..n {
        let mut clique = vec![start];
        for v in 0..n {
            if v == start {
                continue;
            }
            if clique.iter().all(|&u| graph.has_edge(u, v)) {
                clique.push(v);
            }
        }
        best = best.max(clique.len());
    }
    best
}

/// Compute an upper bound on the chromatic number.
///
/// Uses Brook's theorem: χ ≤ Δ + 1, where Δ is the maximum degree.
/// For complete graphs and odd cycles, equality holds.
pub fn upper_bound(graph: &Graph) -> usize {
    if graph.vertex_count() == 0 {
        return 0;
    }
    let max_deg = (0..graph.vertex_count()).map(|v| graph.degree(v)).max().unwrap();
    max_deg + 1
}

/// Check if the graph is bipartite (2-colorable).
///
/// Uses BFS-based 2-coloring attempt.
pub fn is_bipartite(graph: &Graph) -> bool {
    let n = graph.vertex_count();
    if n == 0 {
        return true;
    }
    let mut color = vec![-1i32; n];
    for start in 0..n {
        if color[start] != -1 {
            continue;
        }
        color[start] = 0;
        let mut queue = vec![start];
        let mut head = 0;
        while head < queue.len() {
            let v = queue[head];
            head += 1;
            for &u in graph.neighbors(v) {
                if color[u] == -1 {
                    color[u] = 1 - color[v];
                    queue.push(u);
                } else if color[u] == color[v] {
                    return false;
                }
            }
        }
    }
    true
}

/// Check if the graph is a complete graph.
pub fn is_complete(graph: &Graph) -> bool {
    let n = graph.vertex_count();
    if n <= 1 {
        return true;
    }
    graph.edge_count() == n * (n - 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lower_bound_complete() {
        let g = Graph::complete(4);
        assert_eq!(lower_bound(&g), 4);
    }

    #[test]
    fn test_lower_bound_bipartite() {
        let g = Graph::complete_bipartite(3, 3);
        assert_eq!(lower_bound(&g), 2);
    }

    #[test]
    fn test_upper_bound_complete() {
        let g = Graph::complete(4);
        assert_eq!(upper_bound(&g), 4); // Δ=3, +1 = 4
    }

    #[test]
    fn test_upper_bound_path() {
        let g = Graph::path(5);
        assert_eq!(upper_bound(&g), 3); // Δ=2, +1 = 3
    }

    #[test]
    fn test_is_bipartite_yes() {
        let g = Graph::complete_bipartite(3, 3);
        assert!(is_bipartite(&g));
    }

    #[test]
    fn test_is_bipartite_path() {
        let g = Graph::path(5);
        assert!(is_bipartite(&g));
    }

    #[test]
    fn test_is_bipartite_no() {
        let g = Graph::complete(3);
        assert!(!is_bipartite(&g));
    }

    #[test]
    fn test_is_bipartite_cycle_odd() {
        let g = Graph::cycle(5);
        assert!(!is_bipartite(&g));
    }

    #[test]
    fn test_is_bipartite_cycle_even() {
        let g = Graph::cycle(4);
        assert!(is_bipartite(&g));
    }

    #[test]
    fn test_is_complete() {
        assert!(is_complete(&Graph::complete(4)));
        assert!(!is_complete(&Graph::path(4)));
        assert!(is_complete(&Graph::new(1)));
    }
}
