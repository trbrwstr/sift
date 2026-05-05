use logforge_core::aggregate::Aggregator;

pub fn print_summary(agg: &Aggregator) {
    println!("\n=== Log Summary ===");
    println!("Total entries: {}", agg.total);

    if !agg.levels.is_empty() {
        println!("\nLevels:");
        for (level, count) in &agg.levels {
            println!("  {:<10} {}", level, count);
        }
    }

    if !agg.messages.is_empty() {
        println!("\nTop Messages:");
        for (msg, count) in agg.top_messages(5) {
            println!("  {:<50} {}", truncate(msg, 50), count);
        }
    }

    println!();
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}...", &s[..max])
    } else {
        s.to_string()
    }
}