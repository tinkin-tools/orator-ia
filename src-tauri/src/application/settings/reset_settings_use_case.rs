use crate::domain::settings::entities::{SystemConfig, UserConfig};
use crate::domain::settings::repositories::SettingsRepository;
use crate::domain::shared::{DomainError, DomainResult};

pub struct ResetSettingsUseCase<R: SettingsRepository> {
    repository: R,
}

impl<R: SettingsRepository> ResetSettingsUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn reset_all_to_defaults(&self) -> DomainResult<String> {
        let default_settings = self.repository.get_default_settings();
        self.repository.save_settings(&default_settings)?;
        Ok("Settings reset to defaults successfully".to_string())
    }

    pub fn reset_section_to_defaults(&self, section: String) -> DomainResult<String> {
        let mut settings = self.repository.load_settings()?;

        match section.as_str() {
            "system" => {
                settings.system = SystemConfig::default();
            }
            "user" => {
                settings.user = UserConfig::default();
            }
            _ => {
                return Err(DomainError::ConfigurationError(format!(
                    "Unknown section: {section}"
                )));
            }
        }

        self.repository.save_settings(&settings)?;

        Ok(format!(
            "Section '{section}' reset to defaults successfully"
        ))
    }
}
