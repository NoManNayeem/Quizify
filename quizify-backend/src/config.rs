use std::env;

pub struct Config {
    pub host: String,
    pub port: u16,
}

pub fn get_config() -> Config {
    // Fetch and parse variables
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    Config { host, port }
}
