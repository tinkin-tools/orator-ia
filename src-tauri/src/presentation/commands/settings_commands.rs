use std::sync::Mutex;
use tauri::State;

use crate::application::settings::{
    GetSettingsUseCase, ResetSettingsUseCase, UpdateSettingsUseCase,
};
use crate::domain::settings::entities::Settings;
use crate::infrastructure::settings::FileSettingsRepository;

// Type aliases for cleaner code
type GetSettingsService = GetSettingsUseCase<FileSettingsRepository>;
type UpdateSettingsService = UpdateSettingsUseCase<FileSettingsRepository>;
type ResetSettingsService = ResetSettingsUseCase<FileSettingsRepository>;

#[tauri::command]
pub async fn get_all_settings(
    get_settings_service: State<'_, Mutex<GetSettingsService>>,
) -> Result<Settings, String> {
    let service = get_settings_service
        .lock()
        .map_err(|e| format!("Failed to lock settings service: {e}"))?;

    service.execute().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_setting(
    update_settings_service: State<'_, Mutex<UpdateSettingsService>>,
    section: String,
    subsection: Option<String>,
    key: String,
    value: serde_json::Value,
) -> Result<String, String> {
    let service = update_settings_service
        .lock()
        .map_err(|e| format!("Failed to lock settings service: {e}"))?;

    service
        .update_config(section, subsection, key, value)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_settings(
    update_settings_service: State<'_, Mutex<UpdateSettingsService>>,
) -> Result<String, String> {
    let service = update_settings_service
        .lock()
        .map_err(|e| format!("Failed to lock settings service: {e}"))?;

    service.save_settings().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn reset_settings_to_defaults(
    reset_settings_service: State<'_, Mutex<ResetSettingsService>>,
) -> Result<String, String> {
    let service = reset_settings_service
        .lock()
        .map_err(|e| format!("Failed to lock settings service: {e}"))?;

    service.reset_all_to_defaults().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn reset_section_to_defaults(
    reset_settings_service: State<'_, Mutex<ResetSettingsService>>,
    section: String,
) -> Result<String, String> {
    let service = reset_settings_service
        .lock()
        .map_err(|e| format!("Failed to lock settings service: {e}"))?;

    service
        .reset_section_to_defaults(section)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_config_value(
    update_settings_service: State<'_, Mutex<UpdateSettingsService>>,
    section: String,
    subsection: Option<String>,
    key: String,
) -> Result<Option<serde_json::Value>, String> {
    let service = update_settings_service
        .lock()
        .map_err(|e| format!("Failed to lock settings service: {e}"))?;

    service
        .get_config_value(section, subsection, key)
        .map_err(|e| e.to_string())
}
