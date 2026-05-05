use logforge_core::{types::LogEntry, engine::LogParser};
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