use drive3::{
    api::DriveHub,
    hyper::{self, client::HttpConnector, Body, Response},
    hyper_rustls::{self, HttpsConnector},
    oauth2, Error, Result,
};
use google_drive3 as drive3;
use hyper::body::HttpBody;
use std::env;

pub struct GDrive {
    hub: DriveHub<HttpsConnector<HttpConnector>>,
}

impl GDrive {
    pub async fn new() -> Self {
        let sa_creds_filename = env::var("GOOGLE_SA_PATH").unwrap_or_default();
        println!("Got SA creds path: {}", sa_creds_filename);

        let secret: oauth2::ServiceAccountKey = oauth2::read_service_account_key(sa_creds_filename)
            .await
            .expect("client secret could not be read");
        let sa = oauth2::ServiceAccountAuthenticator::builder(secret)
            .build()
            .await
            .unwrap();

        let hub = DriveHub::new(
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

        Self { hub }
    }

    pub async fn list(&self, query: &str) -> () {
        let result = self
            .hub
            .files()
            .list()
            // .corpora("user")
            // .q(query)
            // .team_drive_id("eos")
            // .supports_team_drives(false)
            // .supports_all_drives(true)
            // .spaces("duo")
            // .page_token("no")
            // .page_size(15)
            // .order_by("kasd")
            // .include_team_drive_items(true)
            // .include_permissions_for_view("et")
            // .include_items_from_all_drives(true)
            // .drive_id("vero")
            // .corpus("erat")
            // .corpora("sed")
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
                | Error::JsonDecodeError(_, _) => println!("{}", e),
            },
            Ok(res) => {
                let files = res.1.files.unwrap();
                for f in files {
                    let name = f.name.unwrap_or_default();
                    let id = f.id.unwrap_or_default();
                    println!("Name(Id): {}({})", name, id);
                }
            }
        }
    }
    // https://drive.google.com/file/d/1tJzLYRQuOD8X66_yEQjsqm1FXI-T_YUw/view?usp=share_link
    pub async fn get(&self, file_id: &str) -> () {
        let result = self
            .hub
            .files()
            .get(file_id)
            .param("alt", "media")
            // .acknowledge_abuse(true)
            .supports_all_drives(true)
            .add_scope("https://www.googleapis.com/auth/drive")
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
                | Error::JsonDecodeError(_, _) => println!("{}", e),
            },
            Ok(res) => {
                let res = res.0;
                let body = res.into_body().size_hint();
                println!("{:#?}", body);
            }
        }
    }
}
