use crate::types::users::User;
use jsonwebtoken::{
    decode, encode, errors::ErrorKind, Algorithm, DecodingKey, EncodingKey, Header, TokenData,
    Validation,
};
use serde::{Deserialize, Serialize};
use std::env;

pub struct Authentication;

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub exp: usize,
    pub user: User,
}

pub enum TokenValidationError {
    Expired,
    Invalid,
    Other,
}

impl Authentication {
    pub fn generate_token(user: User) -> String {
        let secret_key = env::var("JWT_SECRET_KEY").expect("Missing `JWT_SECRET_KEY` env variable");
        let secret_key_bytes = secret_key.as_bytes();
        let claims = Claims {
            exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
            user,
        };
        encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(secret_key_bytes),
        )
        .expect("Failed to generate token")
    }

    pub fn validate_token(token: &str) -> Result<TokenData<Claims>, TokenValidationError> {
        let secret_key = env::var("JWT_SECRET_KEY").expect("Missing `JWT_SECRET_KEY` env variable");
        let secret_key_bytes = secret_key.as_bytes();
        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret_key_bytes),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(token_data) => Ok(token_data),
            Err(err) => match *err.kind() {
                ErrorKind::ExpiredSignature => Err(TokenValidationError::Expired),
                ErrorKind::InvalidToken => Err(TokenValidationError::Invalid),
                _ => Err(TokenValidationError::Other),
            },
        }
    }
}
