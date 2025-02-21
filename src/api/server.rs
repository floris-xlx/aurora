use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::body::BoxBody;
use actix_web::body::EitherBody;
use actix_web::dev::Service;
use actix_web::dev::ServiceResponse;
use actix_web::http::header;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use moka::future::Cache;

use serde_json::json;
use serde_json::Value;
use std::env::var;
use std::io::Result;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

use tracing_subscriber::EnvFilter;
use web::Data;

/// Define a type alias for the shared cache
pub type SharedCache = Arc<Mutex<Cache<String, Value>>>;

#[get("/")]
async fn status() -> impl Responder {
    let status_info = json!({
        "status": "ok",
        "message": "Service is running"
    });
    HttpResponse::Ok().json(status_info)
}

#[get("/docs")]
async fn redirect_to_docs() -> impl Responder {
    HttpResponse::Found()
        .header("Location", "/docs/index.html")
        .finish()
}

#[get("/docs/{filename:.*}")]
async fn serve_docs(path: web::Path<String>) -> impl Responder {
    let filename = path.into_inner();
    let file_path = if filename.is_empty() {
        var("AURORA_DOCS_INDEX_PATH").expect("AURORA_DOCS_INDEX_PATH environment variable not set")
    } else {
        format!(
            "{}/{}",
            var("AURORA_DOCS_TARGET_PATH")
                .expect("AURORA_DOCS_TARGET_PATH environment variable not set"),
            filename
        )
    };
    NamedFile::open_async(file_path).await.unwrap()
}

#[get("/static.files/{filename:.*}")]
async fn serve_static_files(path: web::Path<String>) -> impl Responder {
    let file_path: String = format!(
        "{}/{}",
        var("AURORA_DOCS_STATIC_FILES_PATH")
            .expect("AURORA_DOCS_STATIC_FILES_PATH environment variable not set"),
        path.into_inner()
    );

    NamedFile::open_async(file_path).await.unwrap()
}
pub async fn api() -> Result<()> {
    dotenv().ok();
    let port: u16 = var("API_XYLEX_CLOUD_PORT")
        .unwrap_or("7777".to_string())
        .parse()
        .unwrap_or(7777);

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
            // dashmap cache
            .app_data(Data::new(cache.clone()))
            // status endpoint
            .service(status)
            // rs docs
            .service(redirect_to_docs)
            .service(serve_docs)
            .service(serve_static_files)
    })
    .workers(1)
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
