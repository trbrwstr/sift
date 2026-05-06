use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Config {
    pub default_format: String,
    pub max_threads: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_format: "plain".into(),
            max_threads: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1),
        }
    }
}

pub fn load_config(path: Option<&str>) -> Config {
    if let Some(p) = path {
        if Path::new(p).exists() {
            if let Ok(content) = fs::read_to_string(p) {
                if content.contains("format=json") {
                    return Config {
                        default_format: "json".into(),
                        max_threads: std::thread::available_parallelism()
                            .map(|n| n.get())
                            .unwrap_or(1),
                    };
                }
            }
        }
    }

    Config::default()
}
