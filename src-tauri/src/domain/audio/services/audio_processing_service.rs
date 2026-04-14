use crate::domain::audio::entities::AudioFile;
use crate::domain::settings::entities::AudioProcessingConfig;
use crate::domain::shared::DomainResult;

pub trait AudioProcessingService {
    fn process_audio_file(
        &self,
        audio_file: &AudioFile,
        config: &AudioProcessingConfig,
    ) -> DomainResult<()>;

    fn validate_audio_file(&self, input_path: &str) -> DomainResult<()>;
}
