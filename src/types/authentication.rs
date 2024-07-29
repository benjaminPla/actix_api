use crate::types::users::User;
use jsonwebtoken::{
    // decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
    encode,
    Algorithm,
    EncodingKey,
    Header,
};
use serde::Serialize;

const SECRET_KEY: &[u8] = b"your_secret_key";

pub struct Authentication {}

#[derive(Serialize)]
pub struct Claims {
    pub sub: User,
    pub exp: usize,
}

impl Authentication {
    pub fn generate_token(user: User) -> String {
        let claims = Claims {
            sub: user,
            exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
        };
        encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(SECRET_KEY),
        )
        .expect("Failed to generate token")
    }

    // pub fn validate_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    // decode::<Claims>(
    // token,
    // &DecodingKey::from_secret(SECRET_KEY),
    // &Validation::new(Algorithm::HS256),
    // )
    // }
}
