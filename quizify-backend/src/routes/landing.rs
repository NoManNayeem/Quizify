use actix_web::{get, HttpResponse, web::ServiceConfig};
use actix_files::NamedFile;
use actix_web::Either;
use std::path::PathBuf;

#[get("/")]
async fn landing_page() -> Either<NamedFile, HttpResponse> {
    // Path to the static landing page file
    let path: PathBuf = "static/landing.html".into();

    // Try to open the file; return NamedFile if it exists or an HttpResponse if it doesn't
    match NamedFile::open(path) {
        Ok(file) => Either::Left(file),
        Err(_) => Either::Right(HttpResponse::NotFound().body("Page not found")),
    }
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(landing_page);
}
