pub mod health;
pub mod landing;

use actix_web::web::ServiceConfig;

pub fn init_routes(cfg: &mut ServiceConfig) {
    health::init_routes(cfg);
    landing::init_routes(cfg);
}
