use logforge_core::aggregate::Aggregator;

pub fn print_table(agg: &Aggregator) {
    println!("\n+----------------------+--------+");
    println!("| Metric               | Value  |");
    println!("+----------------------+--------+");

    println!("| Total Entries        | {:>6} |", agg.total);

    println!("+----------------------+--------+");

    if !agg.levels.is_empty() {
        for (level, count) in &agg.levels {
            println!("| Level: {:<13} | {:>6} |", level, count);
        }
        println!("+----------------------+--------+");
    }

    println!();
}