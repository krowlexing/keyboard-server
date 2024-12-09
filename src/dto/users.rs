use hmac::{Hmac, Mac};
use jwt::token;
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Serialize, Deserialize)]
pub struct UserCreds {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    pub token: String,
    pub is_admin: bool,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserClaim {
    pub user_id: i32,
}

impl UserClaim {
    pub fn verify(token: impl AsRef<str>) -> Result<Self, jwt::Error> {
        let key = &key();
        token.as_ref().verify_with_key(key)
    }

    pub fn sign(&self) -> String {
        let key = &key();
        self.sign_with_key(key).unwrap()
    }
}

fn key() -> Hmac<Sha256> {
    Hmac::new_from_slice(b"some-secret").unwrap()
}
