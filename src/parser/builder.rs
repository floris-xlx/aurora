//! Here we build a parser that can handle both bytestream format and file_path format.
//! The parser will determine the correct parsing engine based on EXIF and MIME types.
//!
//!
//!
//!

use actix_web::{web::Bytes, HttpResponse};
use serde_json::Value;
use std::io::Cursor;
use tracing::info;

// crate imports
use crate::parser::csv::convert_csv_reader_to_json;
use crate::parser::schema::determine_document_provider;
use crate::parser::schema::RevolutPersonalSchema;
use crate::utils::bytestream_helper::read_file_to_bytestream;

/// Processes a JSON value by first determining its document provider and then passing it to the appropriate handler.
///
/// # Arguments
///
/// * `json_value` - A mutable reference to a `serde_json::Value` which is expected to be a JSON object.
/// * `schemas` - A slice of `RevolutPersonalSchema` representing the available struct schemas to match against.
///
/// # Returns
///
/// A `Value` representing the processed JSON object.
pub fn process_json_value(json_value: &mut Value, schemas: &[RevolutPersonalSchema]) -> Value {
    info!("Processing JSON value: {:#?}", json_value);
    // First, determine the document provider
    let processed_value: Value = determine_document_provider(json_value, schemas);

    info!("processed_values; {:#?}", processed_value);

    // Here you can add additional processing logic if needed
    // For now, we simply return the processed value
    processed_value
}

/// Handles the processing of a file given its path.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the file.
///
/// # Returns
///
/// An `HttpResponse` indicating the result of the operation.
pub async fn handle_file_path(file_path: &str) -> HttpResponse {
    match read_file_to_bytestream(file_path).await {
        Ok(bytestream) => {
            let reader: Cursor<Vec<u8>> = Cursor::new(bytestream);
            match convert_csv_reader_to_json(reader).await {
                Ok(json_result) => HttpResponse::Ok().json(json_result),
                Err(e) => HttpResponse::InternalServerError()
                    .body(format!("Error converting CSV to JSON: {}", e)),
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Error reading file to bytestream: {}", e)),
    }
}

/// Handles the processing of a bytestream.
///
/// # Arguments
///
/// * `content` - A reference to the bytes of the content.
///
/// # Returns
///
/// An `HttpResponse` indicating the result of the operation.
pub async fn handle_bytestream(content: &Bytes) -> HttpResponse {
    let reader: Cursor<Bytes> = Cursor::new(content.clone());
    // after this function we still are in a Pred-accessor to the `Old` schema that we then cast intot he `Target` schema

    match convert_csv_reader_to_json(reader).await {
        Ok(mut json_result) => {
            // Assuming `schemas` is available in the context or passed as an argument
            let schemas: Vec<RevolutPersonalSchema> = vec![]; // Replace with actual schemas
            let processed_value: Value = process_json_value(&mut json_result, &schemas);

            HttpResponse::Ok().json(processed_value)
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error converting CSV to JSON: {}", e))
        }
    }
}
