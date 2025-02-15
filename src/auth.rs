use serde_json::Value;

#[derive(Clone)]
#[derive(Debug)]
pub struct Authentication {
    authentication_type: String,
    authentication_details: Value,
}

impl Authentication {
    pub fn new(authentication: crate::app_settings::Authentication) -> Authentication {
        match authentication.authentication_type.as_str() {
            "api_key" => Authentication {
                authentication_type: "api_key".to_string(),
                authentication_details: authentication.authentication_details,
            },
            _ => panic!("Unsupported storage type"),
        }
    }

    pub fn is_authenticate(&self, auth_token: &str) -> bool {
        if self.authentication_type == "api_key" {
            let admin_api_key = self.authentication_details["admin_api_key"].as_str().unwrap();
            if auth_token == admin_api_key {
                return true;
            }
        }
        return false;
    }
}
