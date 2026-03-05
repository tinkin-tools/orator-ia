use crate::domain::settings::entities::Settings;
use crate::domain::shared::DomainResult;

/// Repository trait for settings management operations
pub trait SettingsRepository {
    /// Load settings from persistent storage
    fn load_settings(&self) -> DomainResult<Settings>;

    /// Save settings to persistent storage
    fn save_settings(&self, settings: &Settings) -> DomainResult<()>;

    /// Get the default settings
    fn get_default_settings(&self) -> Settings;
}
