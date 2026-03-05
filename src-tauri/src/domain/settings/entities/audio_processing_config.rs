use crate::domain::settings::entities::{ConfigItem, ConfigType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioProcessingConfig {
    pub silence_threshold: ConfigItem,
    pub min_silence_duration: ConfigItem,
    pub min_audio_duration: ConfigItem,
}

impl Default for AudioProcessingConfig {
    fn default() -> Self {
        Self {
            silence_threshold: ConfigItem::new(
                serde_json::json!(0.01),
                ConfigType::Number,
                "Threshold for silence detection (0.0 to 1.0)",
                true,
                false,
            ),
            min_silence_duration: ConfigItem::new(
                serde_json::json!(0.5),
                ConfigType::Number,
                "Minimum duration of silence to detect (in seconds)",
                true,
                false,
            ),
            min_audio_duration: ConfigItem::new(
                serde_json::json!(0.2),
                ConfigType::Number,
                "Minimum duration of audio segments to keep (in seconds)",
                true,
                false,
            ),
        }
    }
}
