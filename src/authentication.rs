use serde_json::Value;

#[derive(Clone, Debug)]
pub struct Authentication {
    authentication_type: String,
    authentication_details: Value,
}

impl Authentication {
    pub fn new(authentication: crate::configuration::Authentication) -> Authentication {
        match authentication.authentication_type.as_str() {
            "admin_api_key" => Authentication {
                authentication_type: "admin_api_key".to_string(),
                authentication_details: authentication.authentication_details,
            },
            _ => panic!("Unsupported storage type"),
        }
    }

    pub fn is_authorized(&self, auth_token: Option<String>, _: String, resource: String) -> bool {
        if resource == "/health" {
            return true;
        }
        let mut is_authorized = false;
        if self.authentication_type == "admin_api_key" {
            let admin_api_key = self.authentication_details["api_key"].as_str().unwrap();
            if let Some(auth_token) = auth_token {
                if auth_token == admin_api_key {
                    is_authorized = true;
                }
            }
        }
        is_authorized
    }
}
