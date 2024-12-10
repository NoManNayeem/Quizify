mod config;
mod routes;

use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_cors::Cors;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = config::get_config();

    println!("Starting server on {}:{}", config.host, config.port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _| {
                origin.as_bytes().starts_with(b"http://localhost")
                    || origin.as_bytes().starts_with(b"http://127.0.0.1")
                    || origin.as_bytes().starts_with(b"http://quizify-frontend")
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec!["Content-Type", "Authorization"])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .configure(routes::init_routes)
    })
    .bind((config.host.as_str(), config.port))?
    .run()
    .await
}
