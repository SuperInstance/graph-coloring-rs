//! # graph-coloring-rs
//!
//! A pure-Rust graph coloring library implementing greedy, backtracking, DSATUR,
//! Welsh-Powell algorithms, and chromatic number bounds.
//!
//! # Example
//! ```
//! use graph_coloring_rs::graph::Graph;
//! use graph_coloring_rs::greedy::greedy_coloring;
//! use graph_coloring_rs::chromatic::chromatic_number_exact;
//!
//! // Create K4 (complete graph on 4 vertices)
//! let mut g = Graph::new(4);
//! g.add_edge(0, 1);
//! g.add_edge(0, 2);
//! g.add_edge(0, 3);
//! g.add_edge(1, 2);
//! g.add_edge(1, 3);
//! g.add_edge(2, 3);
//!
//! let coloring = greedy_coloring(&g);
//! assert!(coloring.is_valid(&g));
//!
//! let chi = chromatic_number_exact(&g);
//! assert_eq!(chi, 4);
//! ```

pub mod graph;
pub mod greedy;
pub mod backtrack;
pub mod dsatur;
pub mod welsh_powell;
pub mod chromatic;
