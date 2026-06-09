//! Tutorial: Fleet resource allocation and register allocation
//!
//! Demonstrates using graph coloring to solve real scheduling problems.

use graph_coloring_rs::graph::Graph;
use graph_coloring_rs::dsatur::dsatur_coloring;
use graph_coloring_rs::greedy::greedy_coloring;
use graph_coloring_rs::welsh_powell::welsh_powell_coloring;
use graph_coloring_rs::chromatic::{chromatic_lower_bound, chromatic_number_exact};

fn main() {
    println!("=== Graph Coloring Tutorial ===\n");

    // === Part 1: Compare all algorithms on a Petersen graph ===
    println!("Part 1: Algorithm comparison on Petersen-like graph");
    let mut g = Graph::new(10);
    // Outer cycle
    for i in 0..5 { g.add_edge(i, (i + 1) % 5); }
    // Inner star
    g.add_edge(5, 7); g.add_edge(5, 8);
    g.add_edge(6, 8); g.add_edge(6, 9);
    g.add_edge(7, 9);
    // Spokes
    for i in 0..5 { g.add_edge(i, i + 5); }

    let greedy_c = greedy_coloring(&g);
    let wp_c = welsh_powell_coloring(&g);
    let dsatur_c = dsatur_coloring(&g);
    let exact = chromatic_number_exact(&g);
    let lower = chromatic_lower_bound(&g);

    println!("  Greedy:       {} colors", greedy_c.num_colors());
    println!("  Welsh-Powell: {} colors", wp_c.num_colors());
    println!("  DSATUR:       {} colors", dsatur_c.num_colors());
    println!("  Exact χ(G):   {}", exact);
    println!("  Lower bound:  {}", lower);
    println!();

    // === Part 2: Fleet scheduling ===
    println!("Part 2: Fleet agent scheduling");
    let mut fleet = Graph::new(8);
    // GPU cluster conflicts
    fleet.add_edge(0, 1); fleet.add_edge(0, 2); fleet.add_edge(0, 3);
    fleet.add_edge(1, 2); fleet.add_edge(1, 3); fleet.add_edge(2, 3);
    // DB conflicts
    fleet.add_edge(4, 5);
    // Network conflicts
    fleet.add_edge(5, 6); fleet.add_edge(6, 7);

    let schedule = dsatur_coloring(&fleet);
    println!("  Minimum time slots: {}", schedule.num_colors());
    for slot in 0..schedule.num_colors() {
        let agents: Vec<usize> = (0..8)
            .filter(|&a| schedule.color_of(a) == slot)
            .collect();
        println!("    Slot {}: Agents {:?}", slot, agents);
    }
    println!();

    // === Part 3: Register allocation ===
    println!("Part 3: Compiler register allocation");
    let mut liveness = Graph::new(6);
    // Variables a=0, b=1, c=2, d=3, e=4, f=5
    // a = b + c  → b,c live together
    // d = a * e  → a,e live together
    // f = d - b  → d,b live together
    liveness.add_edge(0, 1); liveness.add_edge(0, 2);
    liveness.add_edge(0, 4);
    liveness.add_edge(1, 2); liveness.add_edge(1, 3);
    liveness.add_edge(3, 4);

    let alloc = greedy_coloring(&liveness);
    let names = ["a", "b", "c", "d", "e", "f"];
    println!("  Registers needed: {}", alloc.num_colors());
    for (i, name) in names.iter().enumerate() {
        println!("    Variable {} → register {}", name, alloc.color_of(i));
    }
}
