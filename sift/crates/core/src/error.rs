use thiserror::Error;

#[derive(Error, Debug)]
pub enum LogError {
    #[error("Parse error")]
    ParseError,

    #[error("IO error")]
    Io(#[from] std::io::Error),
}