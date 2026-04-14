use crate::domain::shared::{DomainResult, FilePath};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordingStatus {
    Idle,
    Recording,
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingMetadata {
    pub output_path: FilePath,
    pub sample_rate: u32,
    pub channels: u16,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
}

impl RecordingMetadata {
    pub fn new(
        output_path: FilePath,
        sample_rate: u32,
        channels: u16,
        started_at: DateTime<Utc>,
    ) -> Self {
        Self {
            output_path,
            sample_rate,
            channels,
            started_at,
            finished_at: None,
        }
    }

    pub fn finalize(&mut self, finished_at: DateTime<Utc>) {
        self.finished_at = Some(finished_at);
    }

    pub fn output_path_str(&self) -> &str {
        self.output_path.as_str()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingInfo {
    pub status: RecordingStatus,
    pub metadata: Option<RecordingMetadata>,
}

impl RecordingInfo {
    pub fn idle() -> Self {
        Self {
            status: RecordingStatus::Idle,
            metadata: None,
        }
    }

    pub fn recording(metadata: RecordingMetadata) -> Self {
        Self {
            status: RecordingStatus::Recording,
            metadata: Some(metadata),
        }
    }

    pub fn paused(metadata: RecordingMetadata) -> Self {
        Self {
            status: RecordingStatus::Paused,
            metadata: Some(metadata),
        }
    }
}

pub struct RecordingConfig {
    pub output_path: FilePath,
}

impl RecordingConfig {
    pub fn new(output_path: String) -> DomainResult<Self> {
        Ok(Self {
            output_path: FilePath::new(output_path)?,
        })
    }
}
