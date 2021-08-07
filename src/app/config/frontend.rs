use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct FrontendConfig {
    base_uri: String,
}
