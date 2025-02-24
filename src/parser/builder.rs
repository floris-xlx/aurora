//! Here we build a parser that can handle both bytestream format and file_path format.
//! The parser will determine the correct parsing engine based on EXIF and MIME types.
//!

use actix_web::{web::Bytes, HttpResponse};
use serde_json::Value;
use std::io::Cursor;
use tracing::info;

// crate imports
use crate::parser::caster::caster_registry::cast_transactions;
use crate::parser::csv::convert_csv_reader_to_json;
use crate::parser::schema::determine_document_provider;
use crate::parser::schema::RevolutPersonalSchema;
use crate::utils::bytestream_helper::read_file_to_bytestream;

// pdf 
use crate::parser::pdf::output_doc;


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
pub async fn process_json_value(
    json_value: &mut Value,
    schemas: &[RevolutPersonalSchema],
) -> Value {
    info!("Processing JSON value: {:#?}", json_value);
    // First, determine the document provider
    let mut processed_value: Value = determine_document_provider(json_value, schemas);

    info!("processed_values; {:#?}", processed_value);
    // here we wanna enter the pipeline for the casting registry

    let casted_data: Result<Value, String> = cast_transactions(&mut processed_value).await;

    // handle
    let final_value: Value = match casted_data {
        Ok(value) => value,
        Err(e) => {
            // Log the error and return the original processed value
            info!("Error during casting transactions: {}", e);
            processed_value
        }
    };

    // Here you can add additional processing logic if needed
    // For now, we simply return the processed value
    final_value
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

use std::io::Read; // Import the Read trait for read_exact method

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
    let mut reader = Cursor::new(content.clone());

    // Determine if the content is a CSV by checking the first few bytes for a CSV header
    // let is_csv: bool = {
    //     let mut buf: [u8; 4] = [0; 4];
    //     reader.read_exact(&mut buf).is_ok() && buf.starts_with(b"\"") // Simple check for CSV
    // };

    let is_csv: bool = true; // For testing purposes, we assume it's a CSV

    if is_csv {
        match convert_csv_reader_to_json(reader).await {
            Ok(mut json_result) => {
                // Assuming `schemas` is available in the context or passed as an argument
                let schemas: Vec<RevolutPersonalSchema> = vec![]; // Replace with actual schemas
                let processed_value: Value = process_json_value(&mut json_result, &schemas).await;

                HttpResponse::Ok().json(processed_value)
            }
            Err(e) => {
                HttpResponse::InternalServerError().body(format!("Error converting CSV to JSON: {}", e))
            }
        }
    } else {
        match output_doc("./cache/Financieel_Overzicht_2024.pdf").await {
            Ok(pages_text) => HttpResponse::Ok().json(pages_text),
            Err(e) => HttpResponse::InternalServerError().body(format!("Error processing PDF: {}", e)),
        }
    }
}
