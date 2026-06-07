//! # graph-coloring-rs
//!
//! Graph coloring algorithms in pure Rust:
//! - **Graph** representation (adjacency list)
//! - **Greedy** coloring
//! - **Backtracking** (exact) coloring
//! - **DSATUR** (saturation degree) heuristic
//! - **Welsh-Powell** ordering-based greedy
//! - **Chromatic number** lower and upper bounds

/// Graph representation as an adjacency list.
pub mod graph;
/// Greedy sequential coloring.
pub mod greedy;
/// Backtracking exact coloring.
pub mod backtrack;
/// DSATUR (degree of saturation) heuristic.
pub mod dsatur;
/// Welsh-Powell ordering-based coloring.
pub mod welsh_powell;
/// Chromatic number bounds and checks.
pub mod chromatic;
