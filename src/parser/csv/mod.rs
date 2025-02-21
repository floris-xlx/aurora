use anyhow::{Context, Result};
use csv::Reader;
use csv::ReaderBuilder;
use serde_json::json;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tracing::{error, info};

pub async fn csv_to_json(filepath: &str) -> Result<Value> {
    let path: &Path = Path::new(filepath);
    info!("Starting conversion of CSV to JSON for file: {}", filepath);

    // Verify the file is a CSV
    if path.extension().and_then(|s| s.to_str()) != Some("csv") {
        error!("File is not a CSV: {}", filepath);
        anyhow::bail!("File is not a CSV");
    }

    // Open the file
    let file: File = File::open(path).context("Failed to open file")?;
    let reader: BufReader<File> = BufReader::new(file);
    info!("File opened successfully: {}", filepath);

    // Create a CSV reader
    let mut csv_reader: Reader<BufReader<File>> = ReaderBuilder::new().from_reader(reader);
    info!("CSV reader created for file: {}", filepath);

    // Convert CSV to JSON
    let mut records: Vec<Value> = vec![];
    for result in csv_reader.records() {
        match result {
            Ok(record) => {
                let json_record: Value = json!(record.iter().collect::<Vec<&str>>());
                records.push(json_record);
            }
            Err(e) => {
                error!("Error reading record: {}", e);
                return Err(e.into());
            }
        }
    }

    info!("Successfully converted CSV to JSON for file: {}", filepath);
    Ok(json!(records))
}
