use serde::{Deserialize, Serialize};

/// Represents the type of a configuration value
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ConfigType {
    String,
    Number,
    Boolean,
}

/// Represents a single configuration item with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigItem {
    pub value: serde_json::Value,
    pub default_value: serde_json::Value,
    pub config_type: ConfigType,
    pub description: String,
    pub is_editable: bool,
    pub is_sensible: bool,
}

impl ConfigItem {
    pub fn new(
        value: serde_json::Value,
        config_type: ConfigType,
        description: &str,
        is_editable: bool,
        is_sensible: bool,
    ) -> Self {
        Self {
            default_value: value.clone(),
            value,
            config_type,
            description: description.to_string(),
            is_editable,
            is_sensible,
        }
    }

    pub fn reset_to_default(&mut self) {
        self.value = self.default_value.clone();
    }

    pub fn is_modified(&self) -> bool {
        self.value != self.default_value
    }

    pub fn update_value(&mut self, new_value: serde_json::Value) -> Result<(), String> {
        if !self.is_editable {
            return Err("Configuration is not editable".to_string());
        }
        self.value = new_value;
        Ok(())
    }
}
