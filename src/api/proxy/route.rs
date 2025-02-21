use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use mime_guess::mime;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::File;
use std::io::copy;
use tracing::info;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileUrl {
    pub file_url: String,
}

#[post("/proxy/download")]
async fn download_file(file_url: web::Json<FileUrl>) -> impl Responder {
    let url: String = file_url.file_url.clone();
    let uuid: String = Uuid::new_v4().to_string();

    info!("Starting download for URL: {}", url);

    match download_to_cache(&url, &uuid).await {
        Ok(file_path) => {
            info!("Successfully downloaded file to {}", file_path);
            HttpResponse::Ok().json(json!({
                "status": "success",
                "message": format!("File downloaded to {}", file_path),
                "file_name": uuid,
                "file_path": file_path
            }))
        }
        Err(e) => {
            info!("Failed to download file from URL: {}. Error: {}", url, e);
            HttpResponse::InternalServerError().json(
                json!({"status": "error", "message": format!("Failed to download file: {}", e)}),
            )
        }
    }
}

async fn download_to_cache(url: &str, uuid: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client: Client = Client::new();
    let response: reqwest::Response = client.get(url).send().await?;
    let mime_type: mime::Mime = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse().ok())
        .unwrap_or(mime::APPLICATION_OCTET_STREAM);

    // Check if the URL already has a file extension
    let url_extension: Option<&str> = url.split('.').last().filter(|ext| !ext.contains('/'));

    let ext = if let Some(ext) = url_extension {
        ext
    } else {
        mime_guess::get_mime_extensions_str(mime_type.as_ref())
            .and_then(|exts| exts.first())
            .unwrap_or(&"bin")
    };

    let file_path: String = format!("./cache/{}.{}", uuid, ext);
    let mut file: File = File::create(&file_path)?;
    let mut content: web::Bytes = response.bytes().await?;
    copy(&mut content.as_ref(), &mut file)?;
    Ok(file_path)
}
