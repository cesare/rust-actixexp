use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct AuthConfig {
    client_id: String,
    client_secret: String,
}
