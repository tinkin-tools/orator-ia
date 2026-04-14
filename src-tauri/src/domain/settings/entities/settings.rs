use crate::domain::settings::entities::{AudioProcessingConfig, ConfigItem, ConfigType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SystemConfig {
    pub audio_processing: AudioProcessingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    pub gemini_api_key: ConfigItem,
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            gemini_api_key: ConfigItem::new(
                serde_json::json!(""),
                ConfigType::String,
                "API key for Google Gemini service",
                true,
                true,
            ),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    pub system: SystemConfig,
    pub user: UserConfig,
}
