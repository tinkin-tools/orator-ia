use std::sync::Mutex;
use tauri::State;

use crate::domain::audio::entities::recording::{RecordingConfig, RecordingInfo, RecordingMetadata};
use crate::domain::audio::services::RecordingService;
use crate::infrastructure::audio::CpalRecorder;

#[tauri::command]
pub async fn start_recording(
    recording_service: State<'_, Mutex<CpalRecorder>>,
    output_path: String,
) -> Result<RecordingMetadata, String> {
    let mut service = recording_service
        .lock()
        .map_err(|e| format!("Failed to lock recording service: {e}"))?;

    let config = RecordingConfig::new(output_path).map_err(|e| e.to_string())?;
    service.start_recording(config).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn stop_recording(
    recording_service: State<'_, Mutex<CpalRecorder>>,
) -> Result<RecordingMetadata, String> {
    let mut service = recording_service
        .lock()
        .map_err(|e| format!("Failed to lock recording service: {e}"))?;

    service.stop_recording().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn pause_recording(
    recording_service: State<'_, Mutex<CpalRecorder>>,
) -> Result<String, String> {
    let mut service = recording_service
        .lock()
        .map_err(|e| format!("Failed to lock recording service: {e}"))?;

    service.pause_recording().map_err(|e| e.to_string())?;
    Ok("Recording paused".to_string())
}

#[tauri::command]
pub async fn resume_recording(
    recording_service: State<'_, Mutex<CpalRecorder>>,
) -> Result<String, String> {
    let mut service = recording_service
        .lock()
        .map_err(|e| format!("Failed to lock recording service: {e}"))?;

    service.resume_recording().map_err(|e| e.to_string())?;
    Ok("Recording resumed".to_string())
}

#[tauri::command]
pub async fn get_recording_status(
    recording_service: State<'_, Mutex<CpalRecorder>>,
) -> Result<RecordingInfo, String> {
    let service = recording_service
        .lock()
        .map_err(|e| format!("Failed to lock recording service: {e}"))?;

    Ok(service.get_recording_info())
}
