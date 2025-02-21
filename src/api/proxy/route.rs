use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use mime_guess::from_ext;
use mime_guess::mime;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::fs::File;
use std::io::copy;
use tracing::info;
use uuid::Uuid;

#[derive(Deserialize)]
struct FileUrl {
    file_url: String,
}

#[post("/proxy/download")]
async fn download_file(file_url: web::Json<FileUrl>) -> impl Responder {
    let url = file_url.file_url.clone();
    let uuid = Uuid::new_v4().to_string();

    info!("Starting download for URL: {}", url);

    match download_to_cache(&url, &uuid).await {
        Ok(file_path) => {
            info!("Successfully downloaded file to {}", file_path);
            HttpResponse::Ok().json(json!({
                "status": "success",
                "message": format!("File downloaded to {}", file_path),
                "file_name": uuid
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
    let client = Client::new();
    let response = client.get(url).send().await?;
    let mime_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse().ok())
        .unwrap_or(mime::APPLICATION_OCTET_STREAM);

    let ext = mime_guess::get_mime_extensions_str(mime_type.as_ref())
        .and_then(|exts| exts.first())
        .unwrap_or(&"bin");
  

    let file_path = format!("./cache/{}.{}", uuid, ext);
    let mut file = File::create(&file_path)?;
    let mut content = response.bytes().await?;
    copy(&mut content.as_ref(), &mut file)?;
    Ok(file_path)
}
