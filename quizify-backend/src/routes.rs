use actix_web::{get, HttpResponse, Responder, web::ServiceConfig};

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Health check OK")
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(health_check);
}
