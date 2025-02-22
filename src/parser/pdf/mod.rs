use pdf_extract::{extract_text, OutputError};
use tracing::info;
use std::path::Path;

/// Parses a PDF document from a file path and extracts the text of all pages.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the PDF file to be parsed.
///
/// # Returns
///
/// A `Result` containing a vector of strings, each representing the text of a page, or an `OutputError` if an error occurs.
pub async fn output_doc(file_path: &str) -> Result<Vec<String>, OutputError> {
    info!("Document opened successfully");

    // Extract text from the document
    let pages_text = extract_text(file_path).map_err(OutputError::from)?;

    info!("Extracted text from all pages");
    Ok(pages_text.lines().map(String::from).collect())
}
