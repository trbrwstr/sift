use sift_core::{types::LogEntry, engine::LogParser};
use regex::Regex;
use std::sync::LazyLock;

static RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?P<ip>\S+) - - \[(?P<time>[^\]]+)\] "(?P<method>\S+) (?P<path>\S+) \S+" (?P<status>\d{3})"#,
    )
    .expect("nginx log regex is invalid")
});

pub struct NginxParser;

impl NginxParser {
    pub fn new() -> Self {
        Self
    }
}

impl LogParser for NginxParser {
    fn parse_line(&self, line: &str) -> Option<LogEntry> {
        let caps = RE.captures(line)?;

        Some(LogEntry {
            timestamp: Some(caps["time"].to_string()),
            level: None,
            message: caps["path"].to_string(),
            fields: vec![
                ("status".into(), caps["status"].into()),
                ("method".into(), caps["method"].into()),
            ],
        })
    }
}
