use std::sync::Mutex;
use tauri::State;

use crate::application::audio::ProcessAudioUseCase;
use crate::infrastructure::audio::SymphoniaAudioProcessor;

type AudioUseCase = ProcessAudioUseCase<SymphoniaAudioProcessor>;

#[tauri::command]
pub async fn process_audio_file(
    audio_processor: State<'_, Mutex<AudioUseCase>>,
    input_path: String,
    output_path: String,
) -> Result<String, String> {
    let processor = audio_processor
        .lock()
        .map_err(|e| format!("Failed to lock audio processor: {e}"))?;

    processor
        .execute(input_path, output_path)
        .map_err(|e| e.to_string())
}
