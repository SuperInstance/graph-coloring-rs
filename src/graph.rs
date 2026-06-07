//! Graph representation for coloring algorithms.

/// A simple undirected graph represented as an adjacency list.
#[derive(Debug, Clone)]
pub struct Graph {
    /// Number of vertices.
    n: usize,
    /// Adjacency list: `adj[v]` contains all neighbors of vertex `v`.
    adj: Vec<Vec<usize>>,
}

impl Graph {
    /// Create a new graph with `n` vertices and no edges.
    pub fn new(n: usize) -> Self {
        Self {
            n,
            adj: vec![vec![]; n],
        }
    }

    /// Add an undirected edge between vertices `u` and `v`.
    ///
    /// Does nothing if `u == v` or if the edge already exists.
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

    /// Number of vertices in the graph.
    pub fn vertex_count(&self) -> usize {
        self.n
    }

    /// Number of edges in the graph.
    pub fn edge_count(&self) -> usize {
        self.adj.iter().map(|a| a.len()).sum::<usize>() / 2
    }

    /// Degree of vertex `v`.
    pub fn degree(&self, v: usize) -> usize {
        self.adj[v].len()
    }

    /// Maximum degree of any vertex.
    pub fn max_degree(&self) -> usize {
        (0..self.n).map(|v| self.degree(v)).max().unwrap_or(0)
    }

    /// Check if vertices `u` and `v` are adjacent.
    pub fn are_adjacent(&self, u: usize, v: usize) -> bool {
        self.adj[u].contains(&v)
    }

    /// Iterate over all vertices.
    pub fn vertices(&self) -> std::ops::Range<usize> {
        0..self.n
    }

    /// Create a complete graph K_n.
    pub fn complete(n: usize) -> Self {
        let mut g = Self::new(n);
        for i in 0..n {
            for j in (i + 1)..n {
                g.add_edge(i, j);
            }
        }
        g
    }

    /// Create a complete bipartite graph K_{n1, n2}.
    pub fn complete_bipartite(n1: usize, n2: usize) -> Self {
        let mut g = Self::new(n1 + n2);
        for i in 0..n1 {
            for j in 0..n2 {
                g.add_edge(i, n1 + j);
            }
        }
        g
    }

    /// Create a cycle graph C_n.
    pub fn cycle(n: usize) -> Self {
        let mut g = Self::new(n);
        for i in 0..n {
            g.add_edge(i, (i + 1) % n);
        }
        g
    }

    /// Create a path graph P_n.
    pub fn path(n: usize) -> Self {
        let mut g = Self::new(n);
        for i in 1..n {
            g.add_edge(i - 1, i);
        }
        g
    }
}

/// A graph coloring: assigns a color (non-negative integer) to each vertex.
#[derive(Debug, Clone)]
pub struct Coloring {
    colors: Vec<usize>,
}

impl Coloring {
    /// Create a new coloring from a vector of color assignments.
    pub fn new(colors: Vec<usize>) -> Self {
        Self { colors }
    }

    /// Get the color of vertex `v`.
    pub fn color_of(&self, v: usize) -> usize {
        self.colors[v]
    }

    /// Number of distinct colors used.
    pub fn num_colors(&self) -> usize {
        if self.colors.is_empty() {
            return 0;
        }
        *self.colors.iter().max().unwrap() + 1
    }

    /// Check if this is a valid coloring of the given graph
    /// (no two adjacent vertices share the same color).
    pub fn is_valid(&self, graph: &Graph) -> bool {
        for v in graph.vertices() {
            for &u in graph.neighbors(v) {
                if self.colors[v] == self.colors[u] {
                    return false;
                }
            }
        }
        true
    }

    /// Get the underlying color assignments.
    pub fn as_slice(&self) -> &[usize] {
        &self.colors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_graph() {
        let g = Graph::new(0);
        assert_eq!(g.vertex_count(), 0);
        assert_eq!(g.edge_count(), 0);
    }

    #[test]
    fn test_add_edge() {
        let mut g = Graph::new(3);
        g.add_edge(0, 1);
        assert_eq!(g.edge_count(), 1);
        assert!(g.are_adjacent(0, 1));
        assert!(g.are_adjacent(1, 0));
        assert!(!g.are_adjacent(0, 2));
    }

    #[test]
    fn test_no_self_loop() {
        let mut g = Graph::new(3);
        g.add_edge(0, 0);
        assert_eq!(g.edge_count(), 0);
    }

    #[test]
    fn test_no_duplicate_edge() {
        let mut g = Graph::new(3);
        g.add_edge(0, 1);
        g.add_edge(0, 1);
        assert_eq!(g.edge_count(), 1);
    }

    #[test]
    fn test_complete_graph() {
        let g = Graph::complete(4);
        assert_eq!(g.vertex_count(), 4);
        assert_eq!(g.edge_count(), 6);
        assert_eq!(g.max_degree(), 3);
    }

    #[test]
    fn test_bipartite_graph() {
        let g = Graph::complete_bipartite(3, 3);
        assert_eq!(g.vertex_count(), 6);
        assert_eq!(g.edge_count(), 9);
    }

    #[test]
    fn test_cycle_graph() {
        let g = Graph::cycle(5);
        assert_eq!(g.vertex_count(), 5);
        assert_eq!(g.edge_count(), 5);
        assert_eq!(g.degree(0), 2);
    }

    #[test]
    fn test_path_graph() {
        let g = Graph::path(4);
        assert_eq!(g.vertex_count(), 4);
        assert_eq!(g.edge_count(), 3);
        assert_eq!(g.degree(0), 1);
        assert_eq!(g.degree(1), 2);
    }

    #[test]
    fn test_valid_coloring() {
        let g = Graph::complete(3);
        let c = Coloring::new(vec![0, 1, 2]);
        assert!(c.is_valid(&g));
        assert_eq!(c.num_colors(), 3);
    }

    #[test]
    fn test_invalid_coloring() {
        let g = Graph::complete(3);
        let c = Coloring::new(vec![0, 0, 1]);
        assert!(!c.is_valid(&g));
    }
}
