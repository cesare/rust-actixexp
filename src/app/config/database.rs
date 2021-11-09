use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u32,
    pub database: String,
    pub user: String,
    pub password: String,
}
