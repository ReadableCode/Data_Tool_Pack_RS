use sheets4::{Error, Sheets};
use std::collections::HashMap;

use crate::auth;
use crate::http_client;

pub async fn read_data(sheet_id: &str, data_range: &str) -> Result<HashMap<String, i32>, Error> {
    let client = http_client::create_http_client();
    let authenticator = auth::create_authenticator(client.clone()).await;
    let hub = Sheets::new(client.clone(), authenticator);

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
