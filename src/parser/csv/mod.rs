pub mod headers;

use anyhow::{Context, Result};
use csv::{Reader, ReaderBuilder, StringRecord};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use tracing::{error, info};

// crate imports
use crate::parser::csv::headers::normalize_headers;

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

    // Delegate to the conversion function
    convert_csv_reader_to_json(reader).await
}

pub async fn convert_csv_reader_to_json<R: Read>(reader: R) -> Result<Value> {
    // Create a CSV reader
    let mut csv_reader: Reader<R> = ReaderBuilder::new().from_reader(reader);
    info!("CSV reader created");

    // Convert CSV to JSON
    let headers: csv::StringRecord = csv_reader
        .headers()
        .context("Failed to read headers")?
        .clone();

    let normalized_headers: csv::StringRecord = normalize_headers(&headers);
    let mut records: Vec<Value> = vec![];
    for result in csv_reader.records() {
        match result {
            Ok(record) => {
                let mut json_record: HashMap<String, String> = HashMap::new();
                for (header, field) in normalized_headers.iter().zip(record.iter()) {
                    json_record.insert(header.to_string(), field.to_string());
                }
                records.push(json!(json_record));
            }
            Err(e) => {
                error!("Error reading record: {}", e);
                return Err(e.into());
            }
        }
    }

    info!("Successfully converted CSV to JSON");
    Ok(json!(records))
}
