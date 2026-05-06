pub mod common;
pub mod json;
pub mod nginx;
pub mod plain;

use sift_core::engine::LogParser;

pub fn make_parser(format: &str) -> Box<dyn LogParser> {
    match format {
        "json" => Box::new(json::JsonParser),
        "nginx" => Box::new(nginx::NginxParser::new()),
        _ => Box::new(plain::PlainParser),
    }
}
