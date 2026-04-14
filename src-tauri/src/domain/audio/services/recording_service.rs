use crate::domain::audio::entities::recording::{RecordingConfig, RecordingInfo, RecordingMetadata};
use crate::domain::shared::DomainResult;

pub trait RecordingService: Send + Sync {
    fn start_recording(&mut self, config: RecordingConfig) -> DomainResult<RecordingMetadata>;
    fn stop_recording(&mut self) -> DomainResult<RecordingMetadata>;
    fn pause_recording(&mut self) -> DomainResult<()>;
    fn resume_recording(&mut self) -> DomainResult<()>;
    fn get_recording_info(&self) -> RecordingInfo;
}
