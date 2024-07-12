use sheets4::{api::ValueRange, hyper, hyper_rustls, Error, Sheets};

pub async fn read_data(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    sheet_id: &str,
    data_range: &str,
) -> Result<(hyper::Response<hyper::Body>, ValueRange), Error> {
    hub.spreadsheets()
        .values_get(sheet_id, data_range)
        .doit()
        .await
}
