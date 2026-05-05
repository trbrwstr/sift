use logforge_core::aggregate::Aggregator;

pub fn print_summary(agg: &Aggregator) {
    println!("Total entries: {}", agg.total);
    println!("Levels:");

    for (level, count) in &agg.levels {
        println!("  {}: {}", level, count);
    }
}