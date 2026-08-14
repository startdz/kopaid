use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,

    pub seed_superadmin_email: String,
    pub seed_superadmin_username: String,
    pub seed_superadmin_password: String,
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

        let seed_superadmin_email =
            env::var("SEED_SUPERADMIN_EMAIL").expect("SEED_SUPERADMIN_EMAIL must be set");

        let seed_superadmin_username =
            env::var("SEED_SUPERADMIN_USERNAME").expect("SEED_SUPERADMIN_USERNAME must be set");

        let seed_superadmin_password =
            env::var("SEED_SUPERADMIN_PASSWORD").expect("SEED_SUPERADMIN_PASSWORD must be set");

        Ok(Self {
            database_url,
            host,
            port,
            jwt_secret,
            seed_superadmin_email,
            seed_superadmin_username,
            seed_superadmin_password,
        })
    }
}
