use std::result::Result;
use std::sync::Arc;
use std::time::SystemTime;

use deadpool_postgres::Pool;
use hmac::{Hmac, NewMac};
use jwt::SignWithKey;
use jwt::claims::RegisteredClaims;
use rand::{RngCore, SeedableRng};
use rand::rngs::StdRng;
use serde::{Deserialize, Serialize};
use sha2::Sha384;
use thiserror::Error;

use crate::app::config::ApplicationConfig;
use crate::app::db::identity_repository::IdentityRepository;
use crate::app::models::Identity;

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

#[derive(Serialize)]
pub struct AuthenticationResult {
    pub identity: Identity,
    pub identifier: String,
    pub username: String,
    pub name: String,
    pub token: String,
}

#[derive(Debug, Error)]
pub enum AuthenticationError {
    #[error("No saved state found")]
    StateMissing,

    #[error("Callback state does not match saved one")]
    StateNotMatch,

    #[error("Token request failed")]
    TokenRequestFailed,

    #[error("Parsing token response failed")]
    InvalidTokenResponse,

    #[error("User request failed")]
    UserRequestFailed,

    #[error("Parsing user response failed")]
    InvalidUserResponse,

    #[error("Failed to save state to session")]
    StateSavingFailed,

    #[error("Failed to load state from session")]
    StateLoadingFailed,

    #[error("Database connection failed")]
    DatabaseConnectionFailed,

    #[error("Failed to find/register identity")]
    IdentityRegistrationFailed,

    #[error("Failed to create token signing key")]
    InvalidTokenSigninKey,

    #[error("Failed to sign token")]
    TokenSigningFailed,

    #[error("Failed to get current time")]
    SystemTimeFailed,
}

pub struct Authentication {
    config: Arc<ApplicationConfig>,
    pool: Arc<Pool>,
    params: CallbackParams,
    saved_state: Option<String>,
}

impl Authentication {
    pub fn new(config: Arc<ApplicationConfig>, pool: Arc<Pool>, params: CallbackParams, saved_state: Option<String>) -> Self {
        Self {
            config: config,
            pool: pool,
            params: params,
            saved_state: saved_state,
        }
    }

    pub async fn execute(self) -> Result<AuthenticationResult, AuthenticationError> {
        let saved_state = self.saved_state.ok_or(AuthenticationError::StateMissing)?;
        if self.params.state != saved_state {
            return Err(AuthenticationError::StateNotMatch)
        }

        let token_response = TokenRequest::new(&self.config, self.params.code, self.params.state)
            .execute()
            .await?;

        let user_response = UserRequest::new(token_response.access_token).execute().await?;
        let client = self.pool.get().await.or(Err(AuthenticationError::DatabaseConnectionFailed))?;
        let identity = IdentityRepository::new(client).find_or_create(&user_response.id.to_string()).await.or(Err(AuthenticationError::IdentityRegistrationFailed))?;

        let token = TokenGenerator::new(&self.config).generate(&identity.id)?;
        let result = AuthenticationResult {
            identity: identity,
            identifier: user_response.id.to_string(),
            username: user_response.login,
            name: user_response.name,
            token: token,
        };
        Ok(result)
    }
}

struct TokenRequest {
    config: Arc<ApplicationConfig>,
    code: String,
    state: String,
}

impl TokenRequest {
    fn new(config: &Arc<ApplicationConfig>, code: String, state: String) -> Self {
        Self {
            config: config.clone(),
            code: code,
            state: state,
        }
    }

    async fn execute(&self) -> Result<TokenResponse, AuthenticationError> {
        let client = reqwest::Client::new();
        let parameters = [
            ("client_id", &self.config.auth.client_id),
            ("client_secret", &self.config.auth.client_secret),
            ("code", &self.code),
            ("state", &self.state),
        ];
        let result = client.post("https://github.com/login/oauth/access_token")
            .header("Accept", "application/json")
            .form(&parameters)
            .send()
            .await
            .or(Err(AuthenticationError::TokenRequestFailed))?
            .json::<TokenResponse>()
            .await
            .or(Err(AuthenticationError::InvalidTokenResponse))?;

        Ok(result)
    }
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

struct UserRequest {
    access_token: String,
}

impl UserRequest {
    fn new(access_token: String) -> Self {
        Self {
            access_token: access_token,
        }
    }

    async fn execute(&self) -> Result<UserResponse, AuthenticationError> {
        let client = reqwest::Client::new();
        let response = client.get("https://api.github.com/user")
            .header("Accept", "application/vnd.github.v3+json")
            .header("Authorization", format!("token {}", self.access_token))
            .header("User-Agent", "Webauthexp")
            .send()
            .await
            .or(Err(AuthenticationError::UserRequestFailed))?;

        let result = response.json::<UserResponse>()
            .await
            .or(Err(AuthenticationError::InvalidUserResponse))?;
        Ok(result)
    }
}

#[derive(Deserialize)]
struct UserResponse {
    id: u64,
    login: String,
    name: String,
}

struct TokenGenerator {
    config: Arc<ApplicationConfig>,
}

impl TokenGenerator {
    fn new(config: &Arc<ApplicationConfig>) -> Self {
        Self { config: config.clone() }
    }

    fn generate(&self, identifier: &str) -> Result<String, AuthenticationError> {
        let raw_key = &self.config.auth.token_signing_key;
        let key: Hmac<Sha384> = Hmac::new_from_slice(raw_key)
            .or(Err(AuthenticationError::InvalidTokenSigninKey))?;

        let claims = self.build_claims(&identifier)?;
        claims.sign_with_key(&key)
            .or(Err(AuthenticationError::TokenSigningFailed))
    }

    fn build_claims(&self, identifier: &str) -> Result<RegisteredClaims, AuthenticationError> {
        let mut claims = RegisteredClaims::default();

        claims.subject = Some(identifier.to_owned());

        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
            .or(Err(AuthenticationError::SystemTimeFailed))?;
        let now_in_seconds = now.as_secs();
        claims.issued_at = Some(now_in_seconds);
        claims.expiration = Some(now_in_seconds + 3600);

        Ok(claims)
    }
}
