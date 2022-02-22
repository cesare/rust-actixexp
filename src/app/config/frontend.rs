use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct FrontendConfig {
    pub base_uri: String,
}
