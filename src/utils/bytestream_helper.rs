//! ## `bytestream_helper` 
//! 
//! This module provides a utility function to read a file from a given URL or local path
//! into a byte stream. It supports both HTTP/HTTPS URLs and local file paths.

use anyhow::{Context, Result};
use reqwest::{Client, Response};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

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
    if file_url.starts_with("http://") || file_url.starts_with("https://") {
        // If the file_url is a URL, download the file
        let client: Client = Client::new();
        let response: Response = client
            .get(file_url)
            .send()
            .await
            .context("Failed to send request")?;
        let bytes = response
            .bytes()
            .await
            .context("Failed to read response bytes")?;
        Ok(bytes.to_vec())
    } else {
        // If the file_url is a local path, read the file
        let path: &Path = Path::new(file_url);
        let mut file: File = File::open(path).context("Failed to open file")?;
        let mut buffer: Vec<u8> = Vec::new();
        file.read_to_end(&mut buffer)
            .context("Failed to read file")?;
        Ok(buffer)
    }
}
