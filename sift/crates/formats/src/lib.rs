pub mod common;
pub mod json;
pub mod nginx;
pub mod plain;

use sift_core::{engine::LogParser, types::LogEntry};

#[derive(Clone, Default, clap::ValueEnum)]
pub enum LogFormat {
    #[default]
    Plain,
    Json,
    Nginx,
}

/// Enum-based parser: static dispatch, no heap allocation or vtable per line.
pub enum AnyParser {
    Json(json::JsonParser),
    Nginx(nginx::NginxParser),
    Plain(plain::PlainParser),
}

impl LogParser for AnyParser {
    fn parse_line(&self, line: &str) -> Option<LogEntry> {
        match self {
            AnyParser::Json(p) => p.parse_line(line),
            AnyParser::Nginx(p) => p.parse_line(line),
            AnyParser::Plain(p) => p.parse_line(line),
        }
    }
}

pub fn make_parser(format: &LogFormat) -> AnyParser {
    match format {
        LogFormat::Json  => AnyParser::Json(json::JsonParser),
        LogFormat::Nginx => AnyParser::Nginx(nginx::NginxParser::new()),
        LogFormat::Plain => AnyParser::Plain(plain::PlainParser),
    }
}
