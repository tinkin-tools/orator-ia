// Layered architecture modules
pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

// Re-export main domain entities for backwards compatibility
pub use domain::audio::entities::AudioFile;
pub use domain::settings::entities::{AudioProcessingConfig, Settings};

use std::sync::Mutex;

// Import the new modular components
use application::audio::ProcessAudioUseCase;
use application::settings::{GetSettingsUseCase, ResetSettingsUseCase, UpdateSettingsUseCase};
use domain::settings::repositories::SettingsRepository;
use infrastructure::audio::SymphoniaAudioProcessor;
use infrastructure::settings::FileSettingsRepository;
use presentation::commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize dependencies with dependency injection

    // Infrastructure layer
    let audio_service = SymphoniaAudioProcessor::new();
    let settings_repository = match FileSettingsRepository::new() {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Failed to initialize settings repository: {e}");
            std::process::exit(1);
        }
    };

    // Load initial settings into memory
    let initial_settings = match settings_repository.load_settings() {
        Ok(settings) => {
            println!("Settings loaded successfully at startup");
            settings
        }
        Err(e) => {
            eprintln!("Failed to load initial settings: {e}");
            std::process::exit(1);
        }
    };

    // Extract audio processing config for use case
    let audio_config = initial_settings.system.audio_processing.clone();

    // Application layer (Use Cases)
    let process_audio_use_case = Mutex::new(ProcessAudioUseCase::new(audio_service, audio_config));
    let get_settings_use_case = Mutex::new(GetSettingsUseCase::new(settings_repository.clone()));
    let update_settings_use_case =
        Mutex::new(UpdateSettingsUseCase::new(settings_repository.clone()));
    let reset_settings_use_case = Mutex::new(ResetSettingsUseCase::new(settings_repository));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(process_audio_use_case)
        .manage(get_settings_use_case)
        .manage(update_settings_use_case)
        .manage(reset_settings_use_case)
        .invoke_handler(tauri::generate_handler![
            process_audio_file,
            get_all_settings,
            update_setting,
            save_settings,
            reset_settings_to_defaults,
            reset_section_to_defaults,
            get_config_value
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
