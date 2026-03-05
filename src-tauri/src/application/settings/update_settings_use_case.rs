use crate::domain::settings::entities::{AudioProcessingConfig, Settings};
use crate::domain::settings::repositories::SettingsRepository;
use crate::domain::shared::{DomainError, DomainResult, Duration, Threshold};

pub struct UpdateSettingsUseCase<R: SettingsRepository> {
    repository: R,
}

impl<R: SettingsRepository> UpdateSettingsUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn update_config(
        &self,
        section: String,
        subsection: Option<String>,
        key: String,
        value: serde_json::Value,
    ) -> DomainResult<String> {
        let mut settings = self.repository.load_settings()?;

        self.update_config_value(&mut settings, &section, subsection.as_deref(), &key, value)?;

        self.repository.save_settings(&settings)?;

        Ok("Setting updated successfully".to_string())
    }

    pub fn save_settings(&self) -> DomainResult<String> {
        let settings = self.repository.load_settings()?;
        self.repository.save_settings(&settings)?;
        Ok("Settings saved successfully".to_string())
    }

    pub fn get_config_value(
        &self,
        section: String,
        subsection: Option<String>,
        key: String,
    ) -> DomainResult<Option<serde_json::Value>> {
        let settings = self.repository.load_settings()?;
        let value =
            self.get_config_value_from_settings(&settings, &section, subsection.as_deref(), &key);
        Ok(value.cloned())
    }

    fn update_config_value(
        &self,
        settings: &mut Settings,
        section: &str,
        subsection: Option<&str>,
        key: &str,
        value: serde_json::Value,
    ) -> DomainResult<()> {
        match section {
            "system" => {
                if let Some(subsection) = subsection {
                    match subsection {
                        "audio_processing" => {
                            self.update_audio_processing_config(
                                &mut settings.system.audio_processing,
                                key,
                                value,
                            )?;
                        }
                        _ => {
                            return Err(DomainError::ConfigurationError(format!(
                                "Unknown system subsection: {subsection}"
                            )));
                        }
                    }
                } else {
                    return Err(DomainError::ConfigurationError(
                        "System section requires a subsection".to_string(),
                    ));
                }
            }
            "user" => match key {
                "gemini_api_key" => {
                    settings
                        .user
                        .gemini_api_key
                        .update_value(value)
                        .map_err(DomainError::ConfigurationError)?;
                }
                _ => {
                    return Err(DomainError::ConfigurationError(format!(
                        "Unknown user config: {key}"
                    )));
                }
            },
            _ => {
                return Err(DomainError::ConfigurationError(format!(
                    "Unknown section: {section}"
                )));
            }
        }

        Ok(())
    }

    fn get_config_value_from_settings<'a>(
        &'a self,
        settings: &'a Settings,
        section: &str,
        subsection: Option<&str>,
        key: &str,
    ) -> Option<&'a serde_json::Value> {
        match section {
            "system" => {
                if let Some(subsection) = subsection {
                    match subsection {
                        "audio_processing" => {
                            self.get_audio_processing_value(&settings.system.audio_processing, key)
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            }
            "user" => match key {
                "gemini_api_key" => Some(&settings.user.gemini_api_key.value),
                _ => None,
            },
            _ => None,
        }
    }

    fn update_audio_processing_config(
        &self,
        config: &mut AudioProcessingConfig,
        key: &str,
        value: serde_json::Value,
    ) -> DomainResult<()> {
        // Validate the value first by creating value objects
        match key {
            "silence_threshold" => {
                let threshold_value = value.as_f64().ok_or_else(|| {
                    DomainError::ValidationError("silence_threshold must be a number".to_string())
                })? as f32;
                // Validate using value object
                Threshold::new(threshold_value)?;
                // Update ConfigItem
                config
                    .silence_threshold
                    .update_value(value)
                    .map_err(DomainError::ConfigurationError)?;
            }
            "min_silence_duration" => {
                let duration_value = value.as_f64().ok_or_else(|| {
                    DomainError::ValidationError(
                        "min_silence_duration must be a number".to_string(),
                    )
                })? as f32;
                // Validate using value object
                Duration::new(duration_value)?;
                // Update ConfigItem
                config
                    .min_silence_duration
                    .update_value(value)
                    .map_err(DomainError::ConfigurationError)?;
            }
            "min_audio_duration" => {
                let duration_value = value.as_f64().ok_or_else(|| {
                    DomainError::ValidationError("min_audio_duration must be a number".to_string())
                })? as f32;
                // Validate using value object
                Duration::new(duration_value)?;
                // Update ConfigItem
                config
                    .min_audio_duration
                    .update_value(value)
                    .map_err(DomainError::ConfigurationError)?;
            }
            _ => {
                return Err(DomainError::ConfigurationError(format!(
                    "Unknown audio processing config: {key}"
                )));
            }
        }
        Ok(())
    }

    fn get_audio_processing_value<'a>(
        &self,
        config: &'a AudioProcessingConfig,
        key: &str,
    ) -> Option<&'a serde_json::Value> {
        match key {
            "silence_threshold" => Some(&config.silence_threshold.value),
            "min_silence_duration" => Some(&config.min_silence_duration.value),
            "min_audio_duration" => Some(&config.min_audio_duration.value),
            _ => None,
        }
    }
}
