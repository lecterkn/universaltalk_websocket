use std::env;

use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Claims {
    pub sub: String,
    pub name: String,
    pub iat: usize,
    pub exp: usize,
}

fn get_secret_key() -> Vec<u8> {
    let secret_key = env::var("JWT_SECRET_KEY").expect("failed to get jwt secret key");
    return secret_key.into_bytes();
}

pub fn authorize_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(&get_secret_key()),
        &Validation::new(Algorithm::HS256),
    ).map(|token_data| token_data.claims)
}