use crate::domain::shared::errors::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilePath(String);

impl FilePath {
    pub fn new(path: String) -> DomainResult<Self> {
        if path.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "File path cannot be empty".to_string(),
            ));
        }
        Ok(FilePath(path))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<FilePath> for String {
    fn from(path: FilePath) -> Self {
        path.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Threshold(f32);

impl Threshold {
    pub fn new(value: f32) -> DomainResult<Self> {
        if !(0.0..=1.0).contains(&value) {
            return Err(DomainError::ValidationError(
                "Threshold must be between 0.0 and 1.0".to_string(),
            ));
        }
        Ok(Threshold(value))
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Duration(f32);

impl Duration {
    pub fn new(seconds: f32) -> DomainResult<Self> {
        if seconds <= 0.0 {
            return Err(DomainError::ValidationError(
                "Duration must be positive".to_string(),
            ));
        }
        Ok(Duration(seconds))
    }

    pub fn seconds(&self) -> f32 {
        self.0
    }
}
