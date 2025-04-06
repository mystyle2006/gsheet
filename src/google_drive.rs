use google_drive3::{DriveHub, hyper_rustls, oauth2, Error, hyper};
use google_drive3::hyper_rustls::HttpsConnector;
use google_drive3::hyper::client::HttpConnector;
use google_drive3::api::File;
use anyhow::{Result};
use crate::debug_println;

pub struct GoogleDrive {
    drive_hub: DriveHub<HttpsConnector<HttpConnector>>,
}

impl GoogleDrive {
    pub fn new(auth: oauth2::authenticator::Authenticator<HttpsConnector<HttpConnector>>) -> Result<Self, Error> {
        let drive_hub = DriveHub::new(
            hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()?
                    .https_or_http()
                    .enable_http1()
                    .build(),
            ),
            auth,
        );

        Ok(GoogleDrive { drive_hub })
    }

    pub async fn list_spreadsheets(&self, name: &str) -> Result<Vec<File>> {
        let mut all_files = Vec::new();
        let mut page_token: Option<String> = None;

        loop {
            let mut query = "mimeType='application/vnd.google-apps.spreadsheet'".to_string();
            if !name.is_empty() {
                query.push_str(&format!(" and name contains '{}'", name));
            }

            let mut req = self.drive_hub
                .files()
                .list()
                .q(&query)
                .include_items_from_all_drives(true)
                .supports_all_drives(true)
                .corpora("allDrives");

            if let Some(token) = page_token {
                req = req.page_token(&token);
            }

            let (response, file_list) = req.doit().await?;

            // 응답 본문 읽기
            debug_println!("Response status: {:?}", response.status());
            debug_println!("File list: {:?}", file_list);

            if let Some(files) = file_list.files {
                all_files.extend(files);
            }

            match file_list.next_page_token {
                Some(token) => page_token = Some(token),
                None => break,
            }
        }

        debug_println!("Total files found: {}", all_files.len());

        Ok(all_files)
    }
}