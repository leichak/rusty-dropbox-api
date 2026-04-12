/// Enum describing set of errors that can occur
/// Thiserror macro to derive std::error::Error trait
#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[allow(dead_code)]
    #[error("Unknown")] // display trait
    Unknown,
    #[error("Reqwest error: {0}")] // display trait
    Request(anyhow::Error),
    #[error("Parsing error: {0}")] // display trait
    Parsing(anyhow::Error),
    #[error("Dropbox error: {0}")] // display trait
    DropBox(anyhow::Error),
}
