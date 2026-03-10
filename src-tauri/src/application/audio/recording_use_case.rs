use crate::domain::audio::entities::recording::{RecordingConfig, RecordingInfo, RecordingMetadata};
use crate::domain::audio::services::RecordingService;
use crate::domain::shared::DomainResult;

pub struct RecordingUseCase<R: RecordingService> {
    recording_service: R,
}

impl<R: RecordingService> RecordingUseCase<R> {
    pub fn new(recording_service: R) -> Self {
        Self { recording_service }
    }

    pub fn start(&mut self, output_path: String) -> DomainResult<RecordingMetadata> {
        let config = RecordingConfig::new(output_path)?;
        self.recording_service.start_recording(config)
    }

    pub fn stop(&mut self) -> DomainResult<RecordingMetadata> {
        self.recording_service.stop_recording()
    }

    pub fn pause(&mut self) -> DomainResult<()> {
        self.recording_service.pause_recording()
    }

    pub fn resume(&mut self) -> DomainResult<()> {
        self.recording_service.resume_recording()
    }

    pub fn get_status(&self) -> RecordingInfo {
        self.recording_service.get_recording_info()
    }
}
