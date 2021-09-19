pub mod config;
pub mod db;
pub mod errors;
pub mod handlers;
pub mod middlewares;
pub mod models;

pub type Result<T, S = errors::ActixexpError> = std::result::Result<T, S>;
