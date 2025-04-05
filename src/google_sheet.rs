use sheets4::{Sheets, Error, hyper, hyper_rustls, oauth2};
use sheets4::api::ValueRange;
use sheets4::hyper::client::HttpConnector;
use sheets4::hyper::body::Body;
use sheets4::hyper::Response;
use sheets4::hyper_rustls::HttpsConnector;

pub struct GoogleSheet {
    hub: Sheets<HttpsConnector<HttpConnector>>,
}

impl GoogleSheet {
    pub fn new(auth: oauth2::authenticator::Authenticator<HttpsConnector<HttpConnector>>) -> Result<Self, Error> {
        let hub = Sheets::new(
            hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()?
                    .https_or_http()
                    .enable_http1()
                    .build(),
            ),
            auth,
        );

        Ok(GoogleSheet { hub })
    }

    pub async fn get_sheet_id(&self, spreadsheet_id: &str, sheet_name: &str) -> Result<Option<i32>, Error> {
        let sheet_list = self.hub.spreadsheets().get(spreadsheet_id).doit().await?;

        for sheet in sheet_list.1.sheets.unwrap_or_default() {
            if let Some(properties) = sheet.properties {
                if properties.title.as_deref() == Some(sheet_name) {
                    return Ok(properties.sheet_id);
                }
            }
        }

        Ok(None)
    }

    pub async fn get_values(&self, spreadsheet_id: &str, range: &str) -> Result<(Response<Body>, ValueRange), Error> {
        let result = self.hub
            .spreadsheets()
            .values_get(spreadsheet_id, range)
            .doit()
            .await?;

        Ok(result)
    }
}