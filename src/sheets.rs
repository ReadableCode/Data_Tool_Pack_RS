use sheets4::{hyper, hyper_rustls, Error, Sheets};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::auth;
use crate::http_client;

pub struct SheetsClient {
    hub: Arc<Mutex<Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>>>,
}

impl SheetsClient {
    pub async fn new() -> Result<Self, Error> {
        let client = http_client::create_http_client();
        let authenticator = auth::create_authenticator(client.clone()).await;
        let hub = Sheets::new(client.clone(), authenticator);
        Ok(SheetsClient {
            hub: Arc::new(Mutex::new(hub)),
        })
    }

    pub async fn read_data(&self, sheet_id: &str, data_range: &str) -> Result<HashMap<String, i32>, Error> {
        let hub = self.hub.lock().await;
        let (_, spreadsheet) = hub.spreadsheets()
            .values_get(sheet_id, data_range)
            .doit()
            .await?;

        let mut totals = HashMap::<String, i32>::new();

        if let Some(values) = spreadsheet.values {
            for next_row in values.into_iter() {
                let key: String = next_row.get(0)
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_else(|| "".to_string());

                let next_value: i32 = next_row.get(1)
                    .and_then(|v| v.as_str().map(|s| s.parse().ok()))
                    .flatten()
                    .unwrap_or(0);

                let current_value = totals.get(&key).copied().unwrap_or(0);
                let new_value = current_value + next_value;

                totals.insert(key, new_value);
            }
        }

        Ok(totals)
    }
}
