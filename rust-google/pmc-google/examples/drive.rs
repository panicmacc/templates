use pmc_google::drive;

#[tokio::main]
async fn main() {
    let drive = drive::GDrive::new().await;
    drive.list("\"Scans\" in parents").await;
    // let file_id = "1k9pbDQgUGObsPlue6Zl8LMHJTSagtg6R";
    let file_id = "1zJugwWbzXx1U5MHB5P8vQyPgAXRrF-4k";
    drive.get(file_id).await;
}
