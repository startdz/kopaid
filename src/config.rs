use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        let database_url = env::var("DATABASE_URL")?;

        let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

        let port = env::var("APP_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .unwrap_or(8080);

        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        Ok(Self {
            database_url,
            host,
            port,
            jwt_secret,
        })
    }
}
