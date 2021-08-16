use std::result::Result;
use std::sync::Arc;

use rand::{RngCore, SeedableRng};
use rand::rngs::StdRng;
use serde::Deserialize;
use thiserror::Error;

use crate::app::config::ApplicationConfig;

pub struct AuthorizationRequest {
    pub state: String,
}

impl AuthorizationRequest {
    pub fn new() -> Self {
        Self {
            state: Self::generate_state(),
        }
    }

    fn generate_state() -> String {
        let mut rng = StdRng::from_entropy();
        let mut rs: [u8; 32] = [0; 32];
        rng.fill_bytes(&mut rs);
        base64::encode_config(rs, base64::URL_SAFE_NO_PAD)
    }
}

#[derive(Deserialize)]
pub struct CallbackParams {
    pub state: String,
    pub code: String,
}

pub struct AuthenticationResult {
    identifier: String,
}

#[derive(Debug, Error)]
pub enum AuthenticationError {
    #[error("No saved state found")]
    StateMissing,

    #[error("Callback state does not match saved one")]
    StateNotMatch,
}

pub struct Authentication {
    config: Arc<ApplicationConfig>,
    params: CallbackParams,
    saved_state: Option<String>,
}

impl Authentication {
    pub fn new(config: Arc<ApplicationConfig>, params: CallbackParams, saved_state: Option<String>) -> Self {
        Self {
            config: config,
            params: params,
            saved_state: saved_state,
        }
    }

    pub async fn execute(self) -> Result<AuthenticationResult, AuthenticationError> {
        let saved_state = self.saved_state.ok_or(AuthenticationError::StateMissing)?;
        if self.params.state != saved_state {
            return Err(AuthenticationError::StateNotMatch)
        }

        todo!()
    }
}
