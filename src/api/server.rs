use actix_cors::Cors;
use actix_web::body::{BoxBody, EitherBody};
use actix_web::dev::Service;
use actix_web::dev::ServiceResponse;
use actix_web::http::header;
use actix_web::{get, post, web, web::Json, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use mime_guess::mime;
use moka::future::Cache;
use reqwest::{get, Client, Error, Response};
use serde_json::{json, Value};
use std::env::var;
use std::io::Cursor;
use std::io::Result;
use std::result::Result as stdResult;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs::File;
use tokio::sync::Mutex;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;
use web::Data;

// config
use crate::config::get_api_port;

// docs
use crate::api::docs::{redirect_to_docs, serve_docs, serve_static_files};

// proxy
use crate::api::proxy::route::download_file;

// parser::csv
use crate::api::proxy::route::FileUrl;
use crate::parser::csv::{convert_csv_reader_to_json, csv_to_json};
// parser::bytestream_helper
use crate::utils::bytestream_helper::read_file_to_bytestream;

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
    let uuid: String = Uuid::new_v4().to_string();

    match client.get(&url).send().await {
        Ok(response) if response.status().is_success() => {
            let mime_type = response
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok())
                .unwrap_or(mime::APPLICATION_OCTET_STREAM);

            let url_extension: Option<&str> = url.split('.').last().filter(|ext| !ext.contains('/'));

            let ext: &str = if let Some(ext) = url_extension {
                ext
            } else {
                mime_guess::get_mime_extensions_str(mime_type.as_ref())
                    .and_then(|exts| exts.first())
                    .unwrap_or(&"bin")
            };

            let file_path: String = format!("./cache/{}.{}", uuid, ext);
            let mut file: File = match File::create(&file_path).await {
                Ok(f) => f,
                Err(e) => {
                    return HttpResponse::InternalServerError()
                        .body(format!("Error creating file: {}", e))
                }
            };

            let content: web::Bytes = match response.bytes().await {
                Ok(bytes) => bytes,
                Err(e) => {
                    return HttpResponse::InternalServerError()
                        .body(format!("Error reading response bytes: {}", e))
                }
            };

            if let Err(e) = tokio::io::copy(&mut content.as_ref(), &mut file).await {
                return HttpResponse::InternalServerError()
                    .body(format!("Error writing to file: {}", e));
            }

            match read_file_to_bytestream(&file_path).await {
                Ok(bytestream) => {
                    let reader = Cursor::new(bytestream);
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
