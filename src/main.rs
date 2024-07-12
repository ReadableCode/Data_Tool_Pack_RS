extern crate google_sheets4 as sheets4;
use sheets4::Sheets;
use std::collections::HashMap;

mod auth;
mod http_client;
mod sheets;

#[tokio::main]
async fn main() {
    let sheet_id = "1pvmIGeanVd0mjIO4-y53OY-z-ueLIY1AF7e-KZGAMzI";
    let data_range = "rust_test!A2:D";

    let client = http_client::create_http_client();
    let authenticator = auth::create_authenticator(client.clone()).await;
    let hub = Sheets::new(client.clone(), authenticator);

    let result = sheets::read_data(&hub, sheet_id, data_range).await;

    match result {
        Err(e) => println!("{}", e),
        Ok((_, spreadsheet)) => {
            let mut totals = HashMap::<String, i32>::new();

            println!(
                "Success: {:?}",
                spreadsheet
                    .values
                    .unwrap()
                    .into_iter()
                    .fold(&mut totals, |acc, next_row| {
                        let key: String = next_row[0].as_str().unwrap_or("").to_string();
                        let next_value: i32 = next_row[1].as_str().unwrap_or("0").parse().unwrap_or(0);

                        let current_value = acc.get(&key).copied().unwrap_or(0);
                        let new_value = current_value + next_value;

                        acc.insert(key, new_value);

                        acc
                    })
            );
        }
    }
}
