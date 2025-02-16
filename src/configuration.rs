use serde::Deserialize;
use serde_json::Value;
use std::fs;
use tracing::info;

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub socket_addr: String,
}

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
    pub server: Server,
    pub physical: Physical,
    pub authentication: Authentication,
}

pub fn load_configuration() -> Configuration {
    let mut configuration_file = std::env::var("CRUSTPASS_CONFIGURATION_FILE").unwrap_or("".to_string());
    let mut configuration_json = std::env::var("CRUSTPASS_CONFIGURATION_JSON").unwrap_or("".to_string());
    if configuration_file == "" && configuration_json == "" {
        configuration_file = "crustpass.json".to_string();
        info!(
            "CRUSTPASS_CONFIGURATION_FILE and CRUSTPASS_CONFIGURATION_JSON not set, using default file: {}",
            configuration_file
        );
        configuration_json = fs::read_to_string(configuration_file).expect("Unable to read the file");
    } else if configuration_file != "" && configuration_json != "" {
        info!(
            "CRUSTPASS_CONFIGURATION_FILE and CRUSTPASS_CONFIGURATION_JSON both set, using CRUSTPASS_CONFIGURATION_FILE file: {}",
            configuration_file
        );
        configuration_json = fs::read_to_string(configuration_file).expect("Unable to read the file");
    } else if configuration_file != "" && configuration_json == "" {
        info!("CRUSTPASS_CONFIGURATION_FILE set, using file: {}", configuration_file);
        configuration_json = fs::read_to_string(configuration_file).expect("Unable to read the file");
    } else if configuration_json != "" && configuration_file == "" {
        info!("CRUSTPASS_CONFIGURATION_JSON set, using JSON");
    } else {
        panic!("Something went wrong with the settings");
    }

    serde_json::from_str(configuration_json.as_str()).expect("Unable to parse App Settings")
}
