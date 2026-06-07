//! Graph representation as an adjacency list.

/// An undirected graph represented as an adjacency list.
///
/// Vertices are numbered `0..n`. Edges are stored symmetrically.
#[derive(Clone, Debug)]
pub struct Graph {
    n: usize,
    adj: Vec<Vec<usize>>,
}

impl Graph {
    /// Create an empty graph with `n` vertices and no edges.
    pub fn new(n: usize) -> Self {
        Self {
            n,
            adj: vec![vec![]; n],
        }
    }

    /// Number of vertices.
    pub fn vertex_count(&self) -> usize {
        self.n
    }

    /// Number of edges.
    pub fn edge_count(&self) -> usize {
        self.adj.iter().map(|v| v.len()).sum::<usize>() / 2
    }

    /// Add an undirected edge between `u` and `v`.
    ///
    /// Does nothing if the edge already exists or if `u == v`.
    pub fn add_edge(&mut self, u: usize, v: usize) {
        if u == v || u >= self.n || v >= self.n {
            return;
        }
        if !self.adj[u].contains(&v) {
            self.adj[u].push(v);
            self.adj[v].push(u);
        }
    }

    /// Get the neighbors of vertex `v`.
    pub fn neighbors(&self, v: usize) -> &[usize] {
        &self.adj[v]
    }

    /// Get the degree of vertex `v`.
    pub fn degree(&self, v: usize) -> usize {
        self.adj[v].len()
    }

    /// Check if there is an edge between `u` and `v`.
    pub fn has_edge(&self, u: usize, v: usize) -> bool {
        self.adj[u].contains(&v)
    }

    /// Build a complete graph `K_n` with all pairwise edges.
    pub fn complete(n: usize) -> Self {
        let mut g = Self::new(n);
        for u in 0..n {
            for v in (u + 1)..n {
                g.add_edge(u, v);
            }
        }
        g
    }

    /// Build a complete bipartite graph `K_{n1, n2}`.
    pub fn complete_bipartite(n1: usize, n2: usize) -> Self {
        let mut g = Self::new(n1 + n2);
        for u in 0..n1 {
            for v in n1..(n1 + n2) {
                g.add_edge(u, v);
            }
        }
        g
    }

    /// Build a cycle graph `C_n`.
    pub fn cycle(n: usize) -> Self {
        let mut g = Self::new(n);
        for i in 0..n {
            g.add_edge(i, (i + 1) % n);
        }
        g
    }

    /// Build a path graph `P_n`.
    pub fn path(n: usize) -> Self {
        let mut g = Self::new(n);
        for i in 1..n {
            g.add_edge(i - 1, i);
        }
        g
    }
}

/// A vertex coloring: maps each vertex to a color (non-negative integer).
pub type Coloring = Vec<usize>;

/// Validate that a coloring is proper (no two adjacent vertices share a color).
pub fn is_valid_coloring(graph: &Graph, coloring: &Coloring) -> bool {
    if coloring.len() != graph.vertex_count() {
        return false;
    }
    for u in 0..graph.vertex_count() {
        for &v in graph.neighbors(u) {
            if u < v && coloring[u] == coloring[v] {
                return false;
            }
        }
    }
    true
}

/// Count the number of distinct colors used in a coloring.
pub fn color_count(coloring: &Coloring) -> usize {
    if coloring.is_empty() {
        return 0;
    }
    *coloring.iter().max().unwrap() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_new() {
        let g = Graph::new(5);
        assert_eq!(g.vertex_count(), 5);
        assert_eq!(g.edge_count(), 0);
    }

    #[test]
    fn test_graph_add_edge() {
        let mut g = Graph::new(3);
        g.add_edge(0, 1);
        assert_eq!(g.edge_count(), 1);
        assert!(g.has_edge(0, 1));
        assert!(g.has_edge(1, 0));
    }

    #[test]
    fn test_graph_no_self_loop() {
        let mut g = Graph::new(3);
        g.add_edge(0, 0);
        assert_eq!(g.edge_count(), 0);
    }

    #[test]
    fn test_graph_complete() {
        let g = Graph::complete(4);
        assert_eq!(g.vertex_count(), 4);
        assert_eq!(g.edge_count(), 6); // C(4,2) = 6
    }

    #[test]
    fn test_graph_bipartite() {
        let g = Graph::complete_bipartite(2, 3);
        assert_eq!(g.vertex_count(), 5);
        assert_eq!(g.edge_count(), 6); // 2*3
    }

    #[test]
    fn test_graph_cycle() {
        let g = Graph::cycle(4);
        assert_eq!(g.edge_count(), 4);
    }

    #[test]
    fn test_graph_path() {
        let g = Graph::path(4);
        assert_eq!(g.edge_count(), 3);
    }

    #[test]
    fn test_valid_coloring() {
        let g = Graph::complete(3);
        let coloring = vec![0, 1, 2];
        assert!(is_valid_coloring(&g, &coloring));
    }

    #[test]
    fn test_invalid_coloring() {
        let g = Graph::complete(3);
        let coloring = vec![0, 0, 1];
        assert!(!is_valid_coloring(&g, &coloring));
    }

    #[test]
    fn test_color_count() {
        assert_eq!(color_count(&vec![0, 1, 2, 1, 0]), 3);
        assert_eq!(color_count(&vec![]), 0);
    }
}
