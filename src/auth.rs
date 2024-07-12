use google_sheets4::oauth2::{self, authenticator::Authenticator, ServiceAccountKey};
use google_sheets4::{hyper, hyper_rustls};
use serde_json;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

fn load_env() -> HashMap<String, String> {
    let env_path = Path::new(".env");
    let mut variables = HashMap::new();

    if env_path.exists() {
        let content = fs::read_to_string(env_path).expect("Failed to read .env file");
        for line in content.lines() {
            if let Some((key, value)) = line.split_once('=') {
                variables.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
    }

    variables
}

pub async fn create_authenticator(
    client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
) -> Authenticator<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
    // Manually load the .env file for complex variable handling
    let env_vars = load_env();

    // Debug output to verify environment variable loading
    if let Some(google_service_account_string) = env_vars.get("GOOGLE_SERVICE_ACCOUNT") {
        // Deserialize the JSON string to ensure it is valid
        let secret: ServiceAccountKey = serde_json::from_str(&google_service_account_string)
            .expect("Invalid JSON in GOOGLE_SERVICE_ACCOUNT");

        oauth2::ServiceAccountAuthenticator::with_client(secret, client.clone())
            .build()
            .await
            .expect("Could not create an authenticator")

    } else {
        println!("GOOGLE_SERVICE_ACCOUNT not found in .env");
        panic!("GOOGLE_SERVICE_ACCOUNT is required");
    }
}
