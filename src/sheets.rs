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

    pub async fn read_data(&self, sheet_id: &str, data_range: &str) -> Result<(Vec<String>, Vec<Vec<i32>>), Error> {
        let hub = self.hub.lock().await;
        let (_, spreadsheet) = hub.spreadsheets()
            .values_get(sheet_id, data_range)
            .doit()
            .await?;

        println!("Spreadsheet values: {:?}", spreadsheet.values);

        let mut rows = Vec::new();
        let headers: Vec<String>;

        if let Some(values) = spreadsheet.values {
            if !values.is_empty() {
                headers = values[0].iter()
                    .map(|v| v.as_str().unwrap_or("").to_string())
                    .collect();

                println!("Headers: {:?}", headers);

                for row in values.iter().skip(1) {
                    let row_vec: Vec<i32> = row.iter()
                        .map(|cell| cell.as_str().unwrap_or("0").parse::<i32>().unwrap_or(0))
                        .collect();

                    println!("Row vec: {:?}", row_vec);
                    rows.push(row_vec);
                }

                Ok((headers, rows))
            } else {
                Ok((vec![], vec![]))
            }
        } else {
            Ok((vec![], vec![]))
        }
    }

    pub fn sum_row(row: &Vec<i32>) -> i32 {
        row.iter().sum()
    }

    pub fn sum_column(data: &Vec<Vec<i32>>, col_index: usize) -> i32 {
        data.iter().map(|row| row.get(col_index).copied().unwrap_or(0)).sum()
    }
}
