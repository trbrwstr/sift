use logforge_core::{types::LogEntry, engine::LogParser};
use regex::Regex;

pub struct NginxParser {
    re: Regex,
}

impl NginxParser {
    pub fn new() -> Self {
        let re = Regex::new(
            r#"(?P<ip>\S+) - - \[(?P<time>[^\]]+)\] "(?P<method>\S+) (?P<path>\S+) \S+" (?P<status>\d{3})"#,
        ).unwrap();

        Self { re }
    }
}

impl LogParser for NginxParser {
    fn parse_line(&self, line: &str) -> Option<LogEntry> {
        let caps = self.re.captures(line)?;

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