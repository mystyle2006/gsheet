use sheets4::{oauth2, Result, hyper_rustls, hyper};

pub async fn get_auth() -> Result<oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>> {
    let secret = oauth2::read_application_secret("client_secret.json")
        .await
        .expect("client_secret.json 을 읽지 못했습니다");

    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
        .persist_tokens_to_disk("token_cache.json")
        .build()
        .await?;

    Ok(auth)
}