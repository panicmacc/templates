use google_sheets4 as sheets4;
// use pmc_google::hello;
pub use sheets4::api::{AppendValuesResponse, UpdateValuesResponse, ValueRange};
use sheets4::{
    hyper::{self, client::HttpConnector, Body, Response},
    hyper_rustls::{self, HttpsConnector},
    oauth2, Error, Result, Sheets,
};
use std::env;

pub type SheetRows = Vec<Vec<String>>;

pub struct Sheet {
    // sa: oauth2::authenticator::Authenticator<HttpsConnector<HttpConnector>>,
    hub: Sheets<HttpsConnector<HttpConnector>>,
    sheet_id: String,
}

impl Sheet {
    pub async fn new(sheet_id: String) -> Self {
        let sa_creds_filename = env::var("GOOGLE_SA_PATH").unwrap_or_default();
        println!("Got SA creds path: {}", sa_creds_filename);

        let secret: oauth2::ServiceAccountKey = oauth2::read_service_account_key(sa_creds_filename)
            .await
            .expect("client secret could not be read");

        let sa = oauth2::ServiceAccountAuthenticator::builder(secret)
            .build()
            .await
            .unwrap();

        let hub = Sheets::new(
            hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_or_http()
                    .enable_http1()
                    .enable_http2()
                    .build(),
            ),
            sa,
        );
        Self { hub, sheet_id }
    }

    pub async fn get_rows(&self, range: &str) -> Option<SheetRows> {
        let result = self
            .hub
            .spreadsheets()
            .values_get(&self.sheet_id[..], range)
            .major_dimension("ROWS")
            .doit()
            .await;

        match result {
            Err(e) => match e {
                // The Error enum provides details about what exactly happened.
                // You can also just use its `Debug`, `Display` or `Error` traits
                Error::HttpError(_)
                | Error::Io(_)
                | Error::MissingAPIKey
                | Error::MissingToken(_)
                | Error::Cancelled
                | Error::UploadSizeLimitExceeded(_, _)
                | Error::Failure(_)
                | Error::BadRequest(_)
                | Error::FieldClash(_)
                | Error::JsonDecodeError(_, _) => {
                    println!("{}", e);
                    None
                }
            },
            Ok(res) => {
                let rows = res.1.values;
                rows
            }
        }
    }

    pub async fn append(
        &self,
        range: &str,
        values: Vec<Vec<String>>,
    ) -> Result<(Response<Body>, AppendValuesResponse)> {
        let major_dimension = Some(String::from("ROWS"));
        let values = ValueRange {
            range: Some(String::from(range)),
            major_dimension,
            values: Some(values),
        };

        let result = self
            .hub
            .spreadsheets()
            .values_append(values, &self.sheet_id[..], range)
            .value_input_option("RAW")
            .include_values_in_response(true)
            .doit()
            .await;
        result
    }

    pub async fn update(
        &self,
        range: &str,
        values: Vec<Vec<String>>,
    ) -> Result<(Response<Body>, UpdateValuesResponse)> {
        let major_dimension = Some(String::from("ROWS"));
        let values = ValueRange {
            range: Some(String::from(range)),
            major_dimension,
            values: Some(values),
        };

        let result = self
            .hub
            .spreadsheets()
            .values_update(values, &self.sheet_id[..], range)
            .value_input_option("RAW")
            .include_values_in_response(true)
            .doit()
            .await;
        result
    }
}
