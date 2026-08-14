use crate::{
    config::Config,
    health::health,
    infrastructure::database::{create_pool, run_migrations, run_seeds},
    modules::{
        auth::handler::login,
        permissions::handler::{create_permission, list_permissions},
        roles::handler::{assign_permission, create_role, list_roles},
        users::handler::{assign_role, create_user, list_users},
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
    run_seeds(&pool, &config)
        .await
        .expect("Failed to run seeders");

    let host = config.host.clone();
    let port = config.port;
    let pool = web::Data::new(pool);
    let config = web::Data::new(config);

    println!("Server running at http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
            .app_data(pool.clone())
            .service(
                web::scope("/api/v1")
                    // PUBLIC
                    .service(web::scope("/auth").route("/login", web::post().to(login)))
                    // PROTECTED
                    .service(
                        web::scope("")
                            .wrap(crate::middleware::auth::AuthMiddleware)
                            .service(
                                web::scope("/users")
                                    .route("", web::get().to(list_users))
                                    .route("", web::post().to(create_user))
                                    .route("/{id}/role", web::patch().to(assign_role)),
                            )
                            .service(
                                web::scope("/roles")
                                    .route("", web::get().to(list_roles))
                                    .route("", web::post().to(create_role))
                                    .route("/{id}/permission", web::post().to(assign_permission)),
                            )
                            .service(
                                web::scope("/permissions")
                                    .route("", web::get().to(list_permissions))
                                    .route("", web::post().to(create_permission)),
                            )
                            // HEALTY CHECK
                            .service(
                                web::scope("/monitor").route("/health", web::get().to(health)),
                            ),
                    ),
            )
    })
    .bind((host, port))?
    .run()
    .await
}
