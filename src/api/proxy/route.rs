use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use reqwest::Client;
use std::fs::File;
use std::io::copy;
use std::path::Path;

#[post("/download")]
async fn download_file(file_url: web::Json<String>) -> impl Responder {
    let url = file_url.into_inner();
    let file_name = url.split('/').last().unwrap_or("downloaded_file");
    let file_path = format!("./cache/{}", file_name);

    match download_to_cache(&url, &file_path).await {
        Ok(_) => HttpResponse::Ok().body(format!("File downloaded to {}", file_path)),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to download file: {}", e))
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(download_file))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
