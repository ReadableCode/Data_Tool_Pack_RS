extern crate google_sheets4 as sheets4;
use sheets4::Sheets;
use std::collections::HashMap;

mod auth;
mod config;
mod http_client;
mod sheets;

#[tokio::main]
async fn main() {
    let config = config::Config::new();
    let client = http_client::http_client();
    let auth = auth::auth(&config, client.clone()).await;
    let hub = Sheets::new(client.clone(), auth);

    let result = sheets::read(&hub, &config).await;

    match result {
        Err(e) => println!("{}", e),
        Ok((_, spreadsheet)) => {
            let totals = HashMap::<String, i32>::new();

            println!(
                "Success: {:?}",
                spreadsheet
                    .values
                    .unwrap()
                    .into_iter()
                    .fold(totals, |mut acc, next_row| {
                        let key: String = match next_row[0].as_str() {
                            Some(s) => s.to_string(),
                            None => String::new(),
                        };
                        let next_value: i32 = match next_row[1].as_str() {
                            Some(s) => s.parse::<i32>().unwrap(),
                            None => 0,
                        };

                        let current_value = acc.get(&key).copied().unwrap_or(0);
                        let new_value = current_value + next_value;

                        acc.insert(key, new_value);

                        acc
                    })
            );
        }
    }
}
