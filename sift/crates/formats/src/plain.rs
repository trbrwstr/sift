use logforge_core::{types::LogEntry, engine::LogParser};

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