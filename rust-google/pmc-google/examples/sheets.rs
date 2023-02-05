use pmc_google::sheets::Sheet;
use std::env;

#[tokio::main]
async fn main() {
    // Get sheet id as first cli argument.
    let sheet_id = env::args().skip(1).next().unwrap_or_default().to_string();

    // The GOOGLE_SA_PATH env var must have the path to a Google credentials
    //  file for a Service Account with access to the sheet.
    let sheet = Sheet::new(sheet_id).await;

    // Read a specified range from a sheet
    let range = "Categories!A1:B40";
    let rows = sheet.get_rows(range).await.unwrap();
    for r in rows {
        println!("{:?}", r);
    }

    // Append to a sheet
    let range = "Transactions!A30:B31";
    let values = vec![
        vec!["A1".to_string(), "B1".to_string()],
        vec!["A2".to_string(), "B2".to_string()],
    ];
    let res = sheet.append(range, values).await;
    println!("{:#?}", res);

    // Update a sheet
    let range = "Transactions!A30:B31";
    let values = vec![
        vec!["X1".to_string(), "B1".to_string()],
        vec!["A2".to_string(), "Y1".to_string()],
    ];
    let res = sheet.update(range, values).await;
    println!("{:#?}", res);
}
