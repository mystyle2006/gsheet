use sheets4::{oauth2, hyper_rustls, hyper};
use std::path::Path;
use anyhow::{Result};

pub async fn get_auth<P: AsRef<Path>>(client_secret_path: P) -> Result<oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>> {
    let secret = oauth2::read_application_secret(client_secret_path)
        .await
        .expect("fail to read client secret");

    // 필요한 스코프 정의
    let scopes = &[
        // "https://www.googleapis.com/auth/drive",
        "https://www.googleapis.com/auth/drive.readonly",
        "https://www.googleapis.com/auth/drive.meet.readonly",
        // "https://www.googleapis.com/auth/spreadsheets",
        "https://www.googleapis.com/auth/spreadsheets.readonly",
    ];

    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
        .persist_tokens_to_disk("token_cache.json")
        .build()
        .await?;

    auth.token(scopes).await?;

    Ok(auth)
}