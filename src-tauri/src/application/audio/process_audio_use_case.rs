use crate::domain::audio::entities::AudioFile;
use crate::domain::audio::services::AudioProcessingService;
use crate::domain::settings::entities::AudioProcessingConfig;
use crate::domain::shared::DomainResult;

pub struct ProcessAudioUseCase<A: AudioProcessingService> {
    audio_service: A,
    audio_config: AudioProcessingConfig,
}

impl<A: AudioProcessingService> ProcessAudioUseCase<A> {
    pub fn new(audio_service: A, audio_config: AudioProcessingConfig) -> Self {
        Self {
            audio_service,
            audio_config,
        }
    }

    pub fn execute(&self, input_path: String, output_path: String) -> DomainResult<String> {
        let audio_file = AudioFile::new(input_path, output_path)?;

        self.audio_service
            .validate_audio_file(audio_file.input_path_str())?;

        self.audio_service
            .process_audio_file(&audio_file, &self.audio_config)?;

        Ok("Audio processing completed successfully".to_string())
    }
}
