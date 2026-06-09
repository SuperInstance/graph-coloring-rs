//! Advanced: Frequency assignment and exam scheduling
//!
//! Two classic graph coloring applications:
//! 1. Radio frequency assignment (avoiding interference)
//! 2. Exam scheduling (no student has two exams at the same time)

use graph_coloring_rs::graph::Graph;
use graph_coloring_rs::dsatur::dsatur_coloring;
use graph_coloring_rs::chromatic::chromatic_number_exact;

fn main() {
    // === Frequency Assignment ===
    println!("=== Radio Frequency Assignment ===\n");

    // 7 radio towers. If two towers are close, they interfere.
    // We need to assign frequencies so no two close towers share a frequency.
    let mut towers = Graph::new(7);
    // Tower proximity map (edges = interference)
    towers.add_edge(0, 1); // Tower 0 and 1 are close
    towers.add_edge(0, 2);
    towers.add_edge(1, 3);
    towers.add_edge(2, 3);
    towers.add_edge(2, 4);
    towers.add_edge(3, 5);
    towers.add_edge(4, 5);
    towers.add_edge(4, 6);
    towers.add_edge(5, 6);
    towers.add_edge(1, 4); // Cross-interference

    let freq_assignment = dsatur_coloring(&towers);
    let min_freqs = chromatic_number_exact(&towers);

    println!("Tower interference graph: 7 towers, 10 interference pairs");
    println!("Minimum frequencies needed: {} (exact: {})", 
        freq_assignment.num_colors(), min_freqs);
    println!();
    for tower in 0..7 {
        println!("  Tower {} → Frequency band {}", tower, freq_assignment.color_of(tower));
    }
    println!();

    // Verify: no two adjacent towers share a frequency
    assert!(freq_assignment.is_valid(&towers));
    println!("✅ Assignment verified: no interference\n");

    // === Exam Scheduling ===
    println!("=== University Exam Scheduling ===\n");

    // 12 exams, 8 students. Build conflict graph from student enrollments.
    // Student enrollments: which exams each student takes
    let students = [
        vec![0, 1, 3],     // Student A: Math, Physics, Chemistry
        vec![0, 2, 4],     // Student B: Math, English, History
        vec![1, 2, 5],     // Student C: Physics, English, Bio
        vec![3, 4, 6],     // Student D: Chemistry, History, CS
        vec![5, 6, 7],     // Student E: Bio, CS, Art
        vec![0, 7, 8],     // Student F: Math, Art, Music
        vec![1, 8, 9],     // Student G: Physics, Music, Econ
        vec![9, 10, 11],   // Student H: Econ, Phil, Lit
    ];

    let mut exams = Graph::new(12);
    for student_exams in &students {
        // Every pair of exams a student takes must be in different slots
        for i in 0..student_exams.len() {
            for j in (i + 1)..student_exams.len() {
                exams.add_edge(student_exams[i], student_exams[j]);
            }
        }
    }

    let schedule = dsatur_coloring(&exams);
    let exam_names = ["Math", "Physics", "English", "Chemistry", "History",
                      "Bio", "CS", "Art", "Music", "Econ", "Philosophy", "Literature"];

    println!("Exam conflict graph: 12 exams, {} students", students.len());
    println!("Minimum exam periods needed: {}\n", schedule.num_colors());

    for period in 0..schedule.num_colors() {
        let period_exams: Vec<&str> = (0..12)
            .filter(|&e| schedule.color_of(e) == period)
            .map(|e| exam_names[e])
            .collect();
        println!("  Period {}: {}", period, period_exams.join(", "));
    }

    // Verify no student has a conflict
    assert!(schedule.is_valid(&exams));
    println!("\n✅ Schedule verified: no student has overlapping exams");
}
