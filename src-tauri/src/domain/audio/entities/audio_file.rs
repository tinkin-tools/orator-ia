use crate::domain::shared::{DomainResult, FilePath};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioFile {
    pub input_path: FilePath,
    pub output_path: FilePath,
}

impl AudioFile {
    pub fn new(input_path: String, output_path: String) -> DomainResult<Self> {
        Ok(Self {
            input_path: FilePath::new(input_path)?,
            output_path: FilePath::new(output_path)?,
        })
    }

    pub fn input_path_str(&self) -> &str {
        self.input_path.as_str()
    }

    pub fn output_path_str(&self) -> &str {
        self.output_path.as_str()
    }
}
