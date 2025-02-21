use actix_files::NamedFile;
use actix_web::{get, HttpResponse, Responder, web::Path};
use std::env::var;

#[get("/docs")]
pub async fn redirect_to_docs() -> impl Responder {
    HttpResponse::Found()
        .header("Location", "/docs/index.html")
        .finish()
}

#[get("/docs/{filename:.*}")]
pub async fn serve_docs(path: Path<String>) -> impl Responder {
    let filename: String = path.into_inner();
    let file_path: String = if filename.is_empty() {
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
pub async fn serve_static_files(path: Path<String>) -> impl Responder {
    let file_path: String = format!(
        "{}/{}",
        var("AURORA_DOCS_STATIC_FILES_PATH")
            .expect("AURORA_DOCS_STATIC_FILES_PATH environment variable not set"),
        path.into_inner()
    );

    NamedFile::open_async(file_path).await.unwrap()
}
