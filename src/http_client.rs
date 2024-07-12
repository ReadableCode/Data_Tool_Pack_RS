use sheets4::{hyper, hyper_rustls};

pub fn create_http_client() -> hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
    let https_connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .expect("Failed to load native roots")
        .https_only()
        .enable_http1()
        .enable_http2()
        .build();

    hyper::Client::builder().build(https_connector)
}
