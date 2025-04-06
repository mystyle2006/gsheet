use sheets4::{oauth2, hyper_rustls, hyper};
use std::path::Path;
use anyhow::{Result, Context};

pub async fn get_auth<P: AsRef<Path>>(client_secret_path: P) -> Result<oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>> {
    let client_secret_path = client_secret_path.as_ref();
    let secret = oauth2::read_application_secret(client_secret_path)
        .await
        .with_context(|| format!("Failed to read client secret from '{:?}'. Please ensure the file exists and is accessible.", client_secret_path))?;

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
        .await
        .context("Failed to build authentication flow. Please check your internet connection and try again.")?;

    auth.token(scopes)
        .await
        .context("Failed to obtain authentication token. Please ensure you have the correct permissions and try again.")?;

    Ok(auth)
}