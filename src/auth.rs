use sheets4::oauth2::{self, authenticator::Authenticator};
use sheets4::{hyper, hyper_rustls};

pub async fn create_authenticator(
    private_key_path: &str,
    client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
) -> Authenticator<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
    let secret: oauth2::ServiceAccountKey = oauth2::read_service_account_key(private_key_path)
        .await
        .expect("Secret not found");

    oauth2::ServiceAccountAuthenticator::with_client(secret, client.clone())
        .build()
        .await
        .expect("Could not create an authenticator")
}
