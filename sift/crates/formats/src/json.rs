use sift_core::{types::LogEntry, engine::LogParser};
use serde_json::Value;

pub struct JsonParser;

impl LogParser for JsonParser {
    fn parse_line(&self, line: &str) -> Option<LogEntry> {
        let v: Value = serde_json::from_str(line).ok()?;

        Some(LogEntry {
            timestamp: v.get("timestamp")?.as_str().map(|s| s.to_string()),
            level: v.get("level")?.as_str().map(|s| s.to_string()),
            message: v.get("message")?.as_str()?.to_string(),
            fields: vec![],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sift_core::engine::LogParser;

    #[test]
    fn parses_valid_json_log() {
        let line = r#"{"timestamp":"2024-01-01","level":"ERROR","message":"oops"}"#;
        let entry = JsonParser.parse_line(line).expect("should parse");
        assert_eq!(entry.message, "oops");
        assert_eq!(entry.level.as_deref(), Some("ERROR"));
        assert_eq!(entry.timestamp.as_deref(), Some("2024-01-01"));
    }

    #[test]
    fn returns_none_for_missing_message() {
        let line = r#"{"timestamp":"2024-01-01","level":"ERROR"}"#;
        assert!(JsonParser.parse_line(line).is_none());
    }

    #[test]
    fn returns_none_for_invalid_json() {
        assert!(JsonParser.parse_line("not json").is_none());
    }

    #[test]
    fn returns_none_for_empty_input() {
        assert!(JsonParser.parse_line("").is_none());
    }
}