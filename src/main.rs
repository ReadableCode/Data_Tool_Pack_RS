extern crate google_sheets4 as sheets4;

mod auth;
mod http_client;
mod sheets;

#[tokio::main]
async fn main() {
    let sheet_id = "1cKaxHkIwrH0jEevJt7W5_r_xyw3uwjETerajwbZ8Kjk";
    let data_range = "TestApp!A1:D";

    let sheets_client = sheets::SheetsClient::new().await.unwrap();

    let (headers, data) = sheets_client.read_data(sheet_id, data_range).await.unwrap();

    println!("Headers: {:?}", headers);
    println!("Data: {:?}", data);

    for (i, row) in data.iter().enumerate() {
        let row_sum = sheets::SheetsClient::sum_row(row);
        println!("Sum of row {}: {}", i + 1, row_sum);
    }

    for col_index in 0..headers.len() {
        let col_sum = sheets::SheetsClient::sum_column(&data, col_index);
        println!("Sum of column {}: {}", headers[col_index], col_sum);
    }
}
