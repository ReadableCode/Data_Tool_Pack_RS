use sheets4::{hyper, hyper_rustls, Error, Sheets};
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

    pub async fn read_data(&self, sheet_id: &str, data_range: &str) -> Result<Vec<Vec<(String, String)>>, Error> {
        let hub = self.hub.lock().await;
        let (_, spreadsheet) = hub.spreadsheets()
            .values_get(sheet_id, data_range)
            .doit()
            .await?;

        println!("Spreadsheet values: {:?}", spreadsheet.values);

        let mut rows = Vec::new();

        if let Some(values) = spreadsheet.values {
            if !values.is_empty() {
                let headers: Vec<String> = values[0].iter()
                    .map(|v| v.as_str().unwrap_or("").to_string())
                    .collect();

                println!("Headers: {:?}", headers);

                for row in values.iter().skip(1) {
                    let mut row_vec = Vec::new();
                    for (i, cell) in row.iter().enumerate() {
                        let key = headers.get(i).cloned().unwrap_or_else(|| format!("Column{}", i + 1));
                        let value = cell.as_str().unwrap_or("").to_string();
                        row_vec.push((key, value));
                    }
                    println!("Row vec: {:?}", row_vec);
                    rows.push(row_vec);
                }
            }
        }

        Ok(rows)
    }
}
