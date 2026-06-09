# graph-coloring-rs

**Graph coloring algorithms in pure Rust.**

Greedy, backtracking, DSATUR, Welsh-Powell, and exact chromatic number — all in ~1000 lines with 55 tests. Graph coloring assigns colors to vertices so no two adjacent vertices share a color. It's NP-hard in general, but the right heuristic makes all the difference.

## The Key Insight

Graph coloring is fundamentally about **conflict resolution**. Two connected vertices "conflict" — they can't share a color. The chromatic number χ(G) tells you the minimum resources (colors, time slots, frequencies, CPU registers) needed to resolve all conflicts.

The algorithms in this crate represent a spectrum from fast-but-approximate to slow-but-exact:

```
Fast ←————————————————————————————————→ Exact

Greedy → Welsh-Powell → DSATUR → Backtracking → Exact χ(G)
  ~O(V)     O(V·E)       O(V²)     O(V!)          NP-hard
```

**When to use which:**
- **Greedy**: Quick estimate, always valid coloring, might use too many colors
- **Welsh-Powell**: Orders vertices by degree (descending) before greedy — often much better
- **DSATUR**: Picks the most "saturated" vertex next (one with most distinct neighbor colors) — best heuristic for most graphs
- **Backtracking**: Finds exact chromatic number — only practical for V < 20
- **Exact**: Wraps backtracking with bounds pruning

## Quick Start

```rust
use graph_coloring_rs::graph::Graph;
use graph_coloring_rs::greedy::greedy_coloring;
use graph_coloring_rs::dsatur::dsatur_coloring;
use graph_coloring_rs::chromatic::chromatic_number_exact;

// Build a graph: 5 agents competing for 3 resources
let mut g = Graph::new(5);
g.add_edge(0, 1); // Agent 0 and 1 conflict
g.add_edge(0, 2);
g.add_edge(1, 3);
g.add_edge(2, 3);
g.add_edge(3, 4);

// Greedy: fast, valid, maybe not optimal
let coloring = greedy_coloring(&g);
assert!(coloring.is_valid(&g));
println!("Greedy: {} colors", coloring.num_colors());

// DSATUR: usually closer to optimal
let coloring = dsatur_coloring(&g);
println!("DSATUR: {} colors", coloring.num_colors());

// Exact: the true minimum (only for small graphs!)
let chi = chromatic_number_exact(&g);
println!("Chromatic number: {}", chi);
```

## Tutorial: Fleet Resource Allocation

Agents in a fleet need exclusive access to shared resources. Two agents that share a resource can't use it simultaneously — they "conflict". Graph coloring tells us the minimum number of time slots needed.

```rust
use graph_coloring_rs::graph::Graph;
use graph_coloring_rs::dsatur::dsatur_coloring;
use graph_coloring_rs::chromatic::{chromatic_lower_bound, chromatic_upper_bound};

fn main() {
    // 8 agents with resource conflicts
    let mut conflicts = Graph::new(8);
    // GPU cluster: agents 0-3 all need GPU
    conflicts.add_edge(0, 1);
    conflicts.add_edge(0, 2);
    conflicts.add_edge(0, 3);
    conflicts.add_edge(1, 2);
    conflicts.add_edge(1, 3);
    conflicts.add_edge(2, 3);
    // Database: agents 4-5 both need DB
    conflicts.add_edge(4, 5);
    // Network: agents 5-6 share bandwidth
    conflicts.add_edge(5, 6);
    // Bridge: agent 3 and 4 conflict via shared state
    conflicts.add_edge(3, 4);

    // How many time slots do we need?
    let coloring = dsatur_coloring(&conflicts);
    println!("Scheduling with {} time slots:", coloring.num_colors());

    for slot in 0..coloring.num_colors() {
        let agents: Vec<usize> = (0..8)
            .filter(|&a| coloring.color_of(a) == slot)
            .collect();
        println!("  Slot {}: Agents {:?}", slot, agents);
    }

    // Bounds: is our answer optimal?
    let lower = chromatic_lower_bound(&conflicts);
    let upper = chromatic_upper_bound(&conflicts);
    println!("\nChromatic number bounds: [{}, {}]", lower, upper);
    println!("Our answer: {} {}", 
        coloring.num_colors(),
        if coloring.num_colors() == lower { "✅ OPTIMAL" } else { "(may not be optimal)" }
    );
}
```

## Tutorial: Register Allocation

In compiler design, graph coloring allocates CPU registers to variables. Each variable is a vertex, edges connect variables that are "live" simultaneously.

```rust
use graph_coloring_rs::graph::Graph;
use graph_coloring_rs::greedy::greedy_coloring;

fn main() {
    // Simulated liveness graph for: a = b + c; d = a * e; f = d - b;
    let mut liveness = Graph::new(5);
    // Variables: 0=a, 1=b, 2=c, 3=d, 4=e, 5=f... simplified to 5
    liveness.add_edge(0, 1); // a and b both live
    liveness.add_edge(0, 2); // a and c both live  
    liveness.add_edge(0, 4); // a and e both live
    liveness.add_edge(3, 1); // d and b both live
    liveness.add_edge(3, 4); // d and e both live

    let allocation = greedy_coloring(&liveness);
    println!("Register allocation ({} registers needed):", allocation.num_colors());
    let var_names = ["a", "b", "c", "d", "e"];
    for (i, name) in var_names.iter().enumerate() {
        println!("  {} → register {}", name, allocation.color_of(i));
    }
}
```

## Algorithms Compared

### Greedy Coloring
Colors vertices in order 0, 1, 2, ..., assigning the smallest available color. **O(V + E)**. Simple but order-dependent — the same graph can need very different numbers of colors depending on vertex order.

### Welsh-Powell
Sorts vertices by degree (descending) before greedy coloring. **O(V·E)**. The intuition: high-degree vertices are hardest to color, so color them first when you have the most flexibility.

### DSATUR (Degree of Saturation)
At each step, picks the vertex with the most distinct neighbor colors ("saturation degree"). **O(V²)**. Ties broken by degree. This is the best general-purpose heuristic — it's optimal for bipartite graphs and cycle graphs.

### Backtracking
Tries all possible colorings with k colors, incrementing k until a valid coloring is found. **O(V!)** worst case. Only practical for small graphs (V ≤ 20), but gives the **exact** chromatic number.

### Chromatic Bounds
- **Lower bound**: Clique number ω(G) — found greedily
- **Upper bound**: Δ(G) + 1 (maximum degree + 1) — Brooks' theorem tightens this to Δ(G) for non-complete non-odd-cycle graphs

## Architecture

```
graph-coloring-rs
├── src/
│   ├── lib.rs          # Crate root with module declarations
│   ├── graph.rs        # Graph struct, Coloring struct, factory methods
│   ├── greedy.rs       # Greedy + custom-order greedy
│   ├── backtrack.rs    # Backtracking search + chromatic number
│   ├── dsatur.rs       # DSATUR heuristic
│   ├── welsh_powell.rs # Welsh-Powell (degree-sorted greedy)
│   └── chromatic.rs    # Exact χ(G), lower/upper bounds
├── examples/
│   └── basic.rs
├── tests/              # 55 tests across all modules
└── Cargo.toml          # Zero external dependencies
```

## API Reference

### `Graph` — Undirected adjacency list

```rust
let mut g = Graph::new(n);              // n vertices, no edges
let g = Graph::complete(n);             // K_n
let g = Graph::cycle(n);               // C_n
let g = Graph::path(n);                // P_n
let g = Graph::complete_bipartite(m, n); // K_{m,n}
g.add_edge(u, v);                      // undirected edge
g.neighbors(v)                          // &[usize]
g.are_adjacent(u, v)                    // bool
```

### `Coloring` — Color assignment result

```rust
let c: Coloring = greedy_coloring(&g);
c.is_valid(&g)           // bool — checks all edges
c.num_colors()            // count of distinct colors used
c.color_of(v)             // color assigned to vertex v
```

### Algorithms

```rust
greedy::greedy_coloring(&g)                     // → Coloring
greedy::greedy_coloring_with_order(&g, order)   // → Coloring
welsh_powell::welsh_powell_coloring(&g)          // → Coloring
dsatur::dsatur_coloring(&g)                      // → Coloring
chromatic::chromatic_number_exact(&g)            // → usize
chromatic::chromatic_lower_bound(&g)             // → usize
chromatic::chromatic_upper_bound(&g)             // → usize
```

## When to Use What

```
Need a valid coloring fast?
└── Greedy (always works, might use extra colors)

Need a good coloring?
├── Graph has many high-degree vertices? → Welsh-Powell
└── General case? → DSATUR (best heuristic)

Need the EXACT minimum?
└── Small graph (V ≤ 20)? → chromatic_number_exact
└── Large graph? → Use lower/upper bounds to bracket it
```

## Complexity Guide

| Graph Size | Greedy | Welsh-Powell | DSATUR | Exact |
|-----------|--------|--------------|--------|-------|
| 10 nodes | instant | instant | instant | ~1ms |
| 100 nodes | instant | ~1ms | ~1ms | ⚠️ slow |
| 1,000 nodes | ~1ms | ~10ms | ~100ms | ❌ impractical |
| 10,000 nodes | ~10ms | ~100ms | ~1s | ❌ impractical |

## Ecosystem Role

In SuperInstance, `graph-coloring-rs` solves:
- **Fleet scheduling**: Minimum time slots for agents sharing resources
- **Register allocation**: Compiler backend optimization
- **Frequency assignment**: Avoiding interference between fleet radios
- **Task partitioning**: Distributing work across minimal worker pools

## Comparison with Alternatives

| Feature | graph-coloring-rs | petgraph |
|---------|------------------|----------|
| Zero deps | ✅ | ❌ |
| DSATUR | ✅ | ❌ |
| Welsh-Powell | ✅ | ❌ |
| Exact χ(G) | ✅ | ❌ |
| Chromatic bounds | ✅ | ❌ |
| Graph coloring at all | ✅ | ❌ (no coloring) |

## License

MIT
