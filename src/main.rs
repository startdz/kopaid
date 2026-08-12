mod app;
mod config;
mod errors;
mod health;
mod infrastructure;
mod middleware;
mod modules;

use config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let config = Config::from_env().expect("Failed to load application configuration");

    println!("Database: {}", config.database_url);
    println!("Host: {}", config.host);
    println!("Port: {}", config.port);

    app::run(config).await
}
