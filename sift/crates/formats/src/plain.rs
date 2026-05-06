use sift_core::{types::LogEntry, engine::LogParser};

pub struct PlainParser;

impl LogParser for PlainParser {
    fn parse_line(&self, line: &str) -> Option<LogEntry> {
        Some(LogEntry {
            timestamp: None,
            level: None,
            message: line.to_string(),
            fields: vec![],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sift_core::engine::LogParser;

    #[test]
    fn wraps_any_line_as_message() {
        let entry = PlainParser.parse_line("hello world").unwrap();
        assert_eq!(entry.message, "hello world");
        assert!(entry.level.is_none());
        assert!(entry.timestamp.is_none());
    }

    #[test]
    fn handles_empty_line() {
        let entry = PlainParser.parse_line("").unwrap();
        assert_eq!(entry.message, "");
    }
}