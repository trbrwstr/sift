use sift_core::types::LogEntry;

/// Utility: build a LogEntry quickly
pub fn make_entry(
    message: &str,
    level: Option<&str>,
    fields: Vec<(String, String)>,
) -> LogEntry {
    LogEntry {
        timestamp: None,
        level: level.map(|s| s.to_string()),
        message: message.to_string(),
        fields,
    }
}

/// Safe field extractor from JSON-like maps later
pub fn get_field<'a>(fields: &'a [(String, String)], key: &str) -> Option<&'a str> {
    fields.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
}