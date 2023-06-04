use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum S3syncError {
    #[error("a object references a parent directory.")]
    DirectoryTraversalError,
    #[error("cancelled")]
    Cancelled,
}
