use jsonwebtoken::{
    // decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
    encode,
    Algorithm,
    EncodingKey,
    Header,
};
use serde::{Deserialize, Serialize};

const SECRET_KEY: &[u8] = b"your_secret_key";

pub struct Authentication {}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

impl Authentication {
    pub fn generate_token(email: &str) -> String {
        let claims = Claims {
            sub: email.to_owned(),
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