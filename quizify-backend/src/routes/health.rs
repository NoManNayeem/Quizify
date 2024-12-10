use actix_web::{get, HttpResponse, Responder, web::ServiceConfig};
use serde_json::json;

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({ "status": "Healthy" }))
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(health_check);
}
