use crate::{
    config::Config,
    health::health,
    infrastructure::database::{create_pool, run_migrations},
    modules::{
        auth::handler::login,
        users::handler::{create_user, list_users},
    },
};
use actix_web::{App, HttpServer, web};

pub async fn run(config: Config) -> std::io::Result<()> {
    let pool = create_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");

    run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    let host = config.host.clone();
    let port = config.port;
    let pool = web::Data::new(pool);
    let config = web::Data::new(config);

    println!("Server running at http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
            .app_data(pool.clone())
            .route("/health", web::get().to(health))
            .service(
                web::scope("/api/v1")
                    .service(web::scope("/auth").route("/login", web::post().to(login)))
                    .service(
                        web::scope("/users")
                            .wrap(crate::middleware::auth::AuthMiddleware)
                            .route("", web::get().to(list_users))
                            .route("", web::post().to(create_user)),
                    ),
            )
    })
    .bind((host, port))?
    .run()
    .await
}
