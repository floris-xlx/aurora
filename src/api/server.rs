use actix_cors::Cors;
use actix_web::body::{BoxBody, EitherBody};
use actix_web::dev::Service;
use actix_web::dev::ServiceResponse;
use actix_web::http::header;
use actix_web::{get, post, web, web::Json, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use mime_guess::mime;
use moka::future::Cache;
use reqwest::Client;
use serde_json::{json, Value};
use std::io::Result;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs::File;
use tokio::sync::Mutex;
use uuid::Uuid;
use web::Data;

// config
use crate::config::get_api_port;

// docs
use crate::api::docs::{redirect_to_docs, serve_docs, serve_static_files};

// proxy
use crate::api::proxy::route::download_file;

// parser
use crate::api::proxy::route::FileUrl;
use crate::parser::builder::handle_bytestream;

/// Define a type alias for the shared cache
pub type SharedCache = Arc<Mutex<Cache<String, Value>>>;

#[get("/")]
async fn status() -> impl Responder {
    let status_info: Value = json!({
        "status": "ok",
        "message": "Service is running"
    });
    HttpResponse::Ok().json(status_info)
}

#[post("/")]
async fn process_file(file_url: Json<FileUrl>) -> impl Responder {
    let client: Client = Client::new();
    let url: String = file_url.file_url.clone();

    match client.get(&url).send().await {
        Ok(response) if response.status().is_success() => {
            let content: web::Bytes = match response.bytes().await {
                Ok(bytes) => bytes,
                Err(e) => {
                    return HttpResponse::InternalServerError()
                        .body(format!("Error reading response bytes: {}", e))
                }
            };

            handle_bytestream(&content).await
        }
        Ok(response) => HttpResponse::InternalServerError()
            .body(format!("Error from server: {}", response.status())),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to send request: {}", e))
        }
    }
}

pub async fn api() -> Result<()> {
    dotenv().ok();
    let cache: SharedCache = Arc::new(Mutex::new(
        Cache::builder()
            .time_to_live(Duration::from_secs(60))
            .build(),
    ));

    HttpServer::new(move || {
        let cors: Cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .wrap_fn(|req, srv| {
                let fut = srv.call(req);
                async move {
                    let mut res: ServiceResponse<EitherBody<BoxBody>> = fut.await?;
                    res.headers_mut()
                        .insert(header::SERVER, "XYLEX/0".parse().unwrap());
                    Ok(res)
                }
            })
            // moka cache
            .app_data(Data::new(cache.clone()))
            // status endpoint
            .service(status)
            // rs docs
            .service(redirect_to_docs)
            .service(serve_docs)
            .service(serve_static_files)
            // proxy
            .service(download_file)
            // process files at `/`
            .service(process_file)
    })
    .workers(1)
    .bind(("0.0.0.0", get_api_port()))?
    .run()
    .await
}
