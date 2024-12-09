mod config;
mod routes;

use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_cors::Cors;
use dotenv::dotenv;
use std::fs;
use std::process;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Ensure the `.env` file exists
    if !fs::metadata(".env").is_ok() {
        eprintln!("Error: .env file not found! Please provide a .env file in the project root.");
        process::exit(1);
    }

    // Load environment variables and initialize logger
    dotenv().ok();
    env_logger::init();

    // Use the `get_config` function for configuration
    let config = config::get_config();

    // Force binding to 0.0.0.0 for external accessibility
    let host = "0.0.0.0";
    println!("Starting server on {}:{}", host, config.port);

    // Start the Actix Web server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                // Allow localhost during development
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec!["Content-Type", "Authorization"])
            .supports_credentials()
            .max_age(3600); // Cache CORS preflight responses for 1 hour

        App::new()
            .wrap(cors) // Apply CORS configuration
            .wrap(Logger::default()) // Enable request logging
            .configure(routes::init_routes) // Initialize routes
    })
    .bind((host, config.port))? // Bind to 0.0.0.0
    .run()
    .await
}
