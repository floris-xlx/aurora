use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::fs::File;
use std::io::copy;
use std::path::Path;
use tracing::info;

#[derive(Deserialize)]
struct FileUrl {
    file_url: String,
}

#[post("/proxy/download")]
async fn download_file(file_url: web::Json<FileUrl>) -> impl Responder {
    let url = file_url.file_url.clone();
    let file_name = url.split('/').last().unwrap_or("downloaded_file");
    let file_path = format!("./cache/{}", file_name);

    info!("Starting download for URL: {}", url);

    match download_to_cache(&url, &file_path).await {
        Ok(_) => {
            info!("Successfully downloaded file to {}", file_path);
            HttpResponse::Ok().json(json!({"status": "success", "message": format!("File downloaded to {}", file_path)}))
        }
        Err(e) => {
            info!("Failed to download file from URL: {}. Error: {}", url, e);
            HttpResponse::InternalServerError().json(
                json!({"status": "error", "message": format!("Failed to download file: {}", e)}),
            )
        }
    }
}

async fn download_to_cache(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let mut file = File::create(file_path)?;
    let mut content = response.bytes().await?;
    copy(&mut content.as_ref(), &mut file)?;
    Ok(())
}
