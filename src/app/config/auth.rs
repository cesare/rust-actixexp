use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct AuthConfig {
    pub client_id: String,
    pub client_secret: String,
}
