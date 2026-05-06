use sift_core::{types::LogEntry, engine::LogParser};
use serde_json::Value;

pub struct JsonParser;

impl LogParser for JsonParser {
    fn parse_line(&self, line: &str) -> Option<LogEntry> {
        let v: Value = serde_json::from_str(line).ok()?;
        let obj = v.as_object()?;

        let message = obj.get("message")?.as_str()?.to_string();
        let timestamp = obj.get("timestamp").and_then(|v| v.as_str()).map(str::to_string);
        let level = obj.get("level").and_then(|v| v.as_str()).map(str::to_string);

        let fields = obj
            .iter()
            .filter(|(k, _)| !matches!(k.as_str(), "timestamp" | "level" | "message"))
            .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_owned())))
            .collect();

        Some(LogEntry { timestamp, level, message, fields })
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
    fn extra_fields_are_collected() {
        let line = r#"{"level":"INFO","message":"req","status":"500","path":"/api"}"#;
        let entry = JsonParser.parse_line(line).expect("should parse");
        assert!(entry.fields.iter().any(|(k, v)| k == "status" && v == "500"));
        assert!(entry.fields.iter().any(|(k, v)| k == "path" && v == "/api"));
    }

    #[test]
    fn numeric_fields_are_skipped() {
        // Only string values are collected; numeric JSON values are skipped
        let line = r#"{"message":"req","level":"INFO","count":42}"#;
        let entry = JsonParser.parse_line(line).expect("should parse");
        assert!(entry.fields.iter().all(|(k, _)| k != "count"));
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
