use thiserror::Error;

#[derive(Error, Debug)]
pub enum TbaError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Reqwest header parsing error: {0}")]
    ReqwestHeader(#[from] reqwest::header::InvalidHeaderValue),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("CSV write error: {0}")]
    CsvWrite(#[from] csv::Error),
    #[error("Serde JSON reader error: {0}")]
    JsonRead(#[from] serde_json::Error),
}
