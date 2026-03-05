use std::env;
use std::fs;
use std::path::PathBuf;

use crate::domain::settings::entities::Settings;
use crate::domain::settings::repositories::SettingsRepository;
use crate::domain::shared::{DomainError, DomainResult};

#[derive(Clone)]
pub struct FileSettingsRepository {
    settings_path: PathBuf,
}

impl FileSettingsRepository {
    pub fn new() -> DomainResult<Self> {
        let settings_path =
            Self::get_settings_path().map_err(|e| DomainError::FileSystemError(e.to_string()))?;

        Ok(Self { settings_path })
    }

    fn get_settings_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let home_dir = dirs::home_dir().expect("Could not determine home directory");
        let app_dir = home_dir.join(".oratoria");

        if !app_dir.exists() {
            fs::create_dir_all(&app_dir)?;
        }

        Ok(app_dir.join("settings.json"))
    }

    fn apply_environment_overrides(&self, settings: &mut Settings) {
        if let Ok(gemini_api_key) = env::var("ORATORIA_GEMINI_API_KEY") {
            settings.user.gemini_api_key.value = serde_json::json!(gemini_api_key);
            settings.user.gemini_api_key.is_editable = false;
        }
    }
}

impl SettingsRepository for FileSettingsRepository {
    fn load_settings(&self) -> DomainResult<Settings> {
        if self.settings_path.exists() {
            let content = fs::read_to_string(&self.settings_path)
                .map_err(|e| DomainError::FileSystemError(e.to_string()))?;

            let mut settings: Settings = serde_json::from_str(&content).map_err(|e| {
                DomainError::ConfigurationError(format!("Settings file is corrupted: {e}"))
            })?;

            self.apply_environment_overrides(&mut settings);
            Ok(settings)
        } else {
            // Create default settings
            let mut settings = self.get_default_settings();
            self.apply_environment_overrides(&mut settings);

            self.save_settings(&settings)?;
            Ok(settings)
        }
    }

    fn save_settings(&self, settings: &Settings) -> DomainResult<()> {
        let content = serde_json::to_string_pretty(settings)
            .map_err(|e| DomainError::ConfigurationError(e.to_string()))?;

        fs::write(&self.settings_path, content)
            .map_err(|e| DomainError::FileSystemError(e.to_string()))?;

        Ok(())
    }

    fn get_default_settings(&self) -> Settings {
        Settings::default()
    }
}
