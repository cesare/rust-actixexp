use rand::{RngCore, SeedableRng};
use rand::rngs::StdRng;

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
