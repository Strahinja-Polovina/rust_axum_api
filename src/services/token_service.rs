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
    pub roles: String
}

trait ClaimsConstructor {
    fn new(sub: i32, email: String, exp: usize, roles: String) -> Self;
}

impl ClaimsConstructor for Claims {
    fn new(sub: i32, email: String, exp: usize, roles: String) -> Self {
        Claims {
            sub,
            email,
            exp,
            roles
        }
    }
}

pub fn generate_jwt(user: GetUsersDTO) -> Result<String, jsonwebtoken::errors::Error> {
    let expire_time = (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize;
    let secret_key = env::var("JSON_SECRET").expect("JWT secret doesnt exist");
    let my_claims = Claims::new(user.id, user.email, expire_time, user.roles);

    let key = EncodingKey::from_secret(secret_key.as_bytes());
    encode(&Header::default(), &my_claims, &key)
}

pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret_key = env::var("JSON_SECRET").expect("JWT secret doesnt exist");
    let key = DecodingKey::from_secret(secret_key.as_bytes());
    let validation = Validation::default();
    decode::<Claims>(token, &key, &validation).map(|data| data.claims)
}
