use pmc_google::drive;

#[tokio::main]
async fn main() {
    let drive = drive::GDrive::new().await;

    // Get a list of files whose parents include the folder "Scans"
    drive.list("\"Scans\" in parents").await;

    // Get file id as first cli argument.
    let file_id = env::args().skip(1).next().unwrap_or_default().to_string();
    
    // Fetch the file with the specified ID.
    // TODO: differentiate "get", "download", and "export". Return File for "get", and bytes for latter two.
    drive.get(file_id).await;
}
