use std::sync::Mutex;
use tauri::State;

use crate::application::audio::RecordingUseCase;
use crate::domain::audio::entities::recording::{RecordingInfo, RecordingMetadata};
use crate::infrastructure::audio::CpalRecorder;

type RecordingService = RecordingUseCase<CpalRecorder>;

#[tauri::command]
pub async fn start_recording(
    recording_service: State<'_, Mutex<RecordingService>>,
    output_path: String,
) -> Result<RecordingMetadata, String> {
    let mut service = recording_service
        .lock()
        .map_err(|e| format!("Failed to lock recording service: {e}"))?;

    service.start(output_path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn stop_recording(
    recording_service: State<'_, Mutex<RecordingService>>,
) -> Result<RecordingMetadata, String> {
    let mut service = recording_service
        .lock()
        .map_err(|e| format!("Failed to lock recording service: {e}"))?;

    service.stop().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn pause_recording(
    recording_service: State<'_, Mutex<RecordingService>>,
) -> Result<String, String> {
    let mut service = recording_service
        .lock()
        .map_err(|e| format!("Failed to lock recording service: {e}"))?;

    service.pause().map_err(|e| e.to_string())?;
    Ok("Recording paused".to_string())
}

#[tauri::command]
pub async fn resume_recording(
    recording_service: State<'_, Mutex<RecordingService>>,
) -> Result<String, String> {
    let mut service = recording_service
        .lock()
        .map_err(|e| format!("Failed to lock recording service: {e}"))?;

    service.resume().map_err(|e| e.to_string())?;
    Ok("Recording resumed".to_string())
}

#[tauri::command]
pub async fn get_recording_status(
    recording_service: State<'_, Mutex<RecordingService>>,
) -> Result<RecordingInfo, String> {
    let service = recording_service
        .lock()
        .map_err(|e| format!("Failed to lock recording service: {e}"))?;

    Ok(service.get_status())
}
