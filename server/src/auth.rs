use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthClaims {
    pub exp: i32,
    pub iat: i32,
    pub id: String,
    pub user: i32,
}

pub fn encode(claims: &AuthClaims) -> String {
    let secret = std::env::var("AUTH_SECRET").unwrap_or("TPENV".into());
    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap_or_default()
}

pub fn decode(token: &str) -> Option<AuthClaims> {
    let secret = std::env::var("AUTH_SECRET").unwrap_or("TPENV".into());
    match jsonwebtoken::decode::<AuthClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    ) {
        Ok(token) => Some(token.claims),
        Err(_) => None,
    }
}
