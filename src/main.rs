extern crate google_sheets4 as sheets4;

mod auth;
mod http_client;
mod sheets;

#[tokio::main]
async fn main() {
    let sheet_id = "1pvmIGeanVd0mjIO4-y53OY-z-ueLIY1AF7e-KZGAMzI";
    let data_range = "rust_test!A1:D";

    let sheets_client = sheets::SheetsClient::new().await.unwrap();

    let result = sheets_client.read_data(sheet_id, data_range).await;

    match result {
        Err(e) => println!("{}", e),
        Ok(totals) => {
            println!("Success: {:?}", totals);
        }
    }
}
