use serde::Deserialize;
use serde_json::Value;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Physical {
    pub physical_type: String,
    pub physical_details: Value,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Authentication {
    pub authentication_type: String,
    pub authentication_details: Value,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Configuration {
    pub socket_addr: String,
    pub physical: Physical,
    pub authentication: Authentication,
}

pub fn load_configuration() -> Configuration {
    let mut app_settings_file = std::env::var("SQ_SETTINGS_FILE").unwrap_or("".to_string());
    let mut app_settings_json = std::env::var("SQ_SETTINGS_JSON").unwrap_or("".to_string());
    if app_settings_file == "" && app_settings_json == "" {
        app_settings_file = "app_settings.json".to_string();
        println!("SQ_SETTINGS_FILE and SQ_SETTINGS_JSON not set, using default file: {}", app_settings_file);
        app_settings_json = fs::read_to_string(app_settings_file).expect("Unable to read the file");
    } else if app_settings_file != "" && app_settings_json != "" {
        println!("SQ_SETTINGS_FILE and SQ_SETTINGS_JSON both set, using SQ_SETTINGS_FILE file: {}", app_settings_file);
        app_settings_json = fs::read_to_string(app_settings_file).expect("Unable to read the file");
    } else if app_settings_file != "" && app_settings_json == "" {
        println!("SQ_SETTINGS_FILE set, using file: {}", app_settings_file);
        app_settings_json = fs::read_to_string(app_settings_file).expect("Unable to read the file");
    } else if app_settings_json != "" && app_settings_file == "" {
        println!("SQ_SETTINGS_JSON set, using JSON");
    } else {
        panic!("Something went wrong with the settings");
    }

    serde_json::from_str(app_settings_json.as_str()).expect("Unable to parse App Settings")
}
