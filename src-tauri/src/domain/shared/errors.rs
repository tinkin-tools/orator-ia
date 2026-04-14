use std::fmt;

#[derive(Debug, Clone)]
pub enum DomainError {
    ValidationError(String),
    ProcessingError(String),
    ConfigurationError(String),
    FileSystemError(String),
    InvalidInput(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::ValidationError(msg) => write!(f, "Validation error: {msg}"),
            DomainError::ProcessingError(msg) => write!(f, "Processing error: {msg}"),
            DomainError::ConfigurationError(msg) => write!(f, "Configuration error: {msg}"),
            DomainError::FileSystemError(msg) => write!(f, "File system error: {msg}"),
            DomainError::InvalidInput(msg) => write!(f, "Invalid input: {msg}"),
        }
    }
}

impl std::error::Error for DomainError {}

pub type DomainResult<T> = Result<T, DomainError>;
