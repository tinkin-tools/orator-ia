use crate::domain::shared::DomainError;

pub struct ErrorHandler;

impl ErrorHandler {
    pub fn handle_domain_error(error: DomainError) -> String {
        match error {
            DomainError::ValidationError(msg) => format!("Validation Error: {msg}"),
            DomainError::ProcessingError(msg) => format!("Processing Error: {msg}"),
            DomainError::ConfigurationError(msg) => format!("Configuration Error: {msg}"),
            DomainError::FileSystemError(msg) => format!("File System Error: {msg}"),
            DomainError::InvalidInput(msg) => format!("Invalid Input: {msg}"),
        }
    }
}
