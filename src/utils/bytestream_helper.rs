//! ## `bytestream_helper`
//!
//! This module provides a utility function to read a file from a given URL or local path
//! into a byte stream. It supports both HTTP/HTTPS URLs and local file paths.

use anyhow::{Context, Result};
use reqwest::{Client, Response};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use tracing::{error, info};

/// Reads a file from a given URL or local path into a byte stream.
///
/// # Arguments
///
/// * `file_url` - A string slice that holds the URL or local path of the file.
///
/// # Returns
///
/// A `Result<Vec<u8>>` containing the file's byte stream or an error if the operation fails.
pub async fn read_file_to_bytestream(file_url: &str) -> Result<Vec<u8>> {
    info!("Starting to read file from URL or local path: {}", file_url);

    if file_url.starts_with("http://") || file_url.starts_with("https://") {
        // If the file_url is a URL, download the file
        info!("Detected URL, attempting to download: {}", file_url);
        let client: Client = Client::new();
        let response: Response = match client.get(file_url).send().await {
            Ok(resp) => {
                info!("Request sent successfully for URL: {}", file_url);
                resp
            }
            Err(e) => {
                error!(
                    "Failed to send request for URL: {}. Error: {:?}",
                    file_url, e
                );
                return Err(e.into());
            }
        };

        let bytes: actix_web::web::Bytes = match response.bytes().await {
            Ok(b) => {
                info!("Successfully read response bytes for URL: {}", file_url);
                b
            }
            Err(e) => {
                error!(
                    "Failed to read response bytes for URL: {}. Error: {:?}",
                    file_url, e
                );
                return Err(e.into());
            }
        };

        Ok(bytes.to_vec())
    } else {
        // If the file_url is a local path, read the file
        info!("Detected local path, attempting to read file: {}", file_url);
        let path: &Path = Path::new(file_url);
        let mut file: File = match File::open(path) {
            Ok(f) => {
                info!("File opened successfully: {}", file_url);
                f
            }
            Err(e) => {
                error!("Failed to open file: {}. Error: {:?}", file_url, e);
                return Err(e.into());
            }
        };

        let mut buffer: Vec<u8> = Vec::new();
        if let Err(e) = file.read_to_end(&mut buffer) {
            error!("Failed to read file: {}. Error: {:?}", file_url, e);
            return Err(e.into());
        }

        info!("File read successfully: {}", file_url);
        Ok(buffer)
    }
}
