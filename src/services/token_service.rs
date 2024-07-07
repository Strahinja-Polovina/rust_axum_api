use crate::constants::services_constants::{JSON_SECRET, JWT_SECRET_ENV_ERROR};
use crate::models::user_model::GetUsersDTO;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: i32,
    pub email: String,
    pub exp: usize,
}

pub fn generate_jwt(user: GetUsersDTO) -> Result<String, jsonwebtoken::errors::Error> {
    let expire_time = (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize;
    let secret_key = env::var(JSON_SECRET).expect(JWT_SECRET_ENV_ERROR);
    let my_claims = Claims {
        sub: user.id,
        email: user.email,
        exp: expire_time,
    };

    let key = EncodingKey::from_secret(secret_key.as_bytes());
    encode(&Header::default(), &my_claims, &key)
}

pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret_key = env::var(JSON_SECRET).expect(JWT_SECRET_ENV_ERROR);
    let key = DecodingKey::from_secret(secret_key.as_bytes());
    let validation = Validation::default();
    decode::<Claims>(token, &key, &validation).map(|data| data.claims)
}
