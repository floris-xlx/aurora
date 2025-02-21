use actix_cors::Cors;
use actix_web::body::{BoxBody, EitherBody};
use actix_web::dev::Service;
use actix_web::dev::ServiceResponse;
use actix_web::http::header;
use actix_web::{get, post, web, web::Json, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use moka::future::Cache;
use reqwest::{get, Client, Error, Response};
use serde_json::{json, Value};
use std::env::var;
use std::io::Result;
use std::result::Result as stdResult;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

use tracing_subscriber::EnvFilter;
use web::Data;

// config
use crate::config::get_api_port;

// docs
use crate::api::docs::{redirect_to_docs, serve_docs, serve_static_files};

// proxy
use crate::api::proxy::route::download_file;

// parser::csv
use crate::api::proxy::route::FileUrl;
use crate::parser::csv::csv_to_json;

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
    let port: u16 = get_api_port();
    let client: Client = Client::new();
    let url: String = format!("http://localhost:{}/proxy/download", port);

    // Use `tokio::try_join!` to parallelize the HTTP request and JSON parsing
    let response = client.post(&url).json(&file_url).send();
    let result = async {
        let resp = response.await.expect("Failed to send request");
        if resp.status().is_success() {
            let json_data: Value = resp.json().await.expect("Failed to parse JSON");
            if let Some(file_path) = json_data.get("file_path").and_then(|v| v.as_str()) {
                csv_to_json(file_path).await.map_err(|e| {
                    HttpResponse::InternalServerError()
                        .body(format!("Error converting CSV to JSON: {}", e))
                })
            } else {
                Err(HttpResponse::InternalServerError()
                    .body("File path not found in response".to_string()))
            }
        } else {
            Err(HttpResponse::InternalServerError()
                .body(format!("Error from server: {}", resp.status())))
        }
    };

    match result.await {
        Ok(json_result) => HttpResponse::Ok().json(json_result),
        Err(e) => e,
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
