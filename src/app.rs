use crate::{
    config::Config,
    health::health,
    infrastructure::database::{create_pool, run_migrations},
    modules::users::handler::{create_user, list_users},
};
use actix_web::{App, HttpServer, web};

pub async fn run(config: Config) -> std::io::Result<()> {
    let pool = create_pool(&config.database_url)
        .await
        .expect("Failed to connect postgresql");

    println!("Server running at http://{}:{}", config.host, config.port);

    run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(health))
            .route("/api/v1/users", web::get().to(list_users))
            .route("/api/v1/users", web::post().to(create_user))
    })
    .bind((config.host.clone(), config.port))?
    .run()
    .await
}
