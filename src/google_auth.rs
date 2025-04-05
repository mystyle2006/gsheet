use sheets4::{oauth2, Result, hyper_rustls, hyper};
use std::path::Path;

pub async fn get_auth<P: AsRef<Path>>(client_secret_path: P) -> Result<oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>> {
    let secret = oauth2::read_application_secret(client_secret_path)
        .await
        .expect("fail to read client secret");

    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
        .persist_tokens_to_disk("token_cache.json")
        .build()
        .await?;

    Ok(auth)
}