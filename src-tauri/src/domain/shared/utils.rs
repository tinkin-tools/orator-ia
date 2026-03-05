use std::{fs, path::PathBuf};

pub fn get_app_path() -> PathBuf {
    let home_dir = dirs::home_dir().expect("Could not determine home directory");
    let app_dir = home_dir.join(".oratoria");

    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).expect("Error creating app directory");
    }

    app_dir
}


pub fn get_settings_path() -> PathBuf {
    get_app_path().join("settings.json")
}
