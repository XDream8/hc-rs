use thiserror::Error;

#[derive(Error, Debug)]
pub enum HcError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to fetch from '{url}': {source}")]
    Fetch {
        url: String,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    #[error("Thread error: {0}")]
    Thread(String),
    #[error("Failed to create output file '{filename}': {source}")]
    FileCreation {
        filename: String,
        source: std::io::Error,
    },

    #[error("Failed to write to file '{filename}': {source}")]
    FileWrite {
        filename: String,
        source: std::io::Error,
    },
}
