use serde::{Deserialize, Deserializer};
use serde::de::Error;

#[derive(Clone, Debug, Deserialize)]
pub struct AuthConfig {
    pub client_id: String,
    pub client_secret: String,

    #[serde(deserialize_with="deserialize_base64")]
    pub token_signing_key: Vec<u8>,
}

fn deserialize_base64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where D: Deserializer<'de>
{
    String::deserialize(deserializer)
        .and_then(|s|
            base64::decode(&s)
                .map_err(|err| Error::custom(err.to_string())
        ))
}
