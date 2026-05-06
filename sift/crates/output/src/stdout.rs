use sift_core::aggregate::Aggregator;

pub fn print_summary(agg: &Aggregator, top: usize) {
    println!("\n=== Log Summary ===");
    println!("Total entries: {}", agg.total);

    if !agg.levels.is_empty() {
        println!("\nLevels:");
        let mut levels: Vec<_> = agg.levels.iter().collect();
        levels.sort_by_key(|(k, _)| k.as_str());
        for (level, count) in levels {
            println!("  {:<10} {}", level, count);
        }
    }

    if !agg.messages.is_empty() {
        println!("\nTop Messages:");
        for (msg, count) in agg.top_messages(top) {
            println!("  {:<50} {}", truncate(msg, 50), count);
        }
    }

    println!();
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let boundary = s
        .char_indices()
        .nth(max)
        .map(|(i, _)| i)
        .unwrap_or(s.len());
    format!("{}...", &s[..boundary])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_short_string_unchanged() {
        assert_eq!(truncate("hello", 10), "hello");
    }

    #[test]
    fn truncate_exact_length_unchanged() {
        assert_eq!(truncate("hello", 5), "hello");
    }

    #[test]
    fn truncate_long_string_appends_ellipsis() {
        let result = truncate("hello world", 5);
        assert_eq!(result, "hello...");
    }

    #[test]
    fn truncate_multibyte_does_not_panic() {
        // Each char is 3 bytes; slicing by byte index 3 would be valid but by char index 1 is safer
        let result = truncate("héllo", 3);
        assert_eq!(result, "hél...");
        // Crucially, this must not panic even if the char boundary falls mid-byte
        let emoji = "😀😁😂😃😄";
        let _ = truncate(emoji, 3);
    }
}