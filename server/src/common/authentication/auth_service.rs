use chrono::{Utc, Duration};
use jsonwebtoken::{DecodingKey, EncodingKey};
use ring::signature::{Ed25519KeyPair, KeyPair};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{ Validation, encode, decode, Algorithm };
use crate::common::repository::user::models::DeveloperOrEmployer;

pub const STANDARD_REFRESH_TOKEN_EXPIRATION: i64 = 60 * 60 * 24 * 30;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub role: DeveloperOrEmployer
}

pub struct AuthKeys {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey
}

pub async fn init_auth_keys() -> AuthKeys {
    let doc = Ed25519KeyPair::generate_pkcs8(&ring::rand::SystemRandom::new()).unwrap();
    let encoding_key = EncodingKey::from_ed_der(doc.as_ref());

    let pair = Ed25519KeyPair::from_pkcs8(doc.as_ref()).unwrap();
    let decoding_key = DecodingKey::from_ed_der(pair.public_key().as_ref());

    AuthKeys { encoding_key, decoding_key }
}

pub fn get_token(user_name: String, dev_or_emp: DeveloperOrEmployer, encoding_key: &EncodingKey, exp_duration_seconds: Option<i64>) -> String {
    let duration = if let None = exp_duration_seconds {
        STANDARD_REFRESH_TOKEN_EXPIRATION
    } else {
        exp_duration_seconds.unwrap()
    };
    let claims = Claims { sub: user_name, exp: (Utc::now() + Duration::seconds(duration)).timestamp() as usize, role: dev_or_emp };
    let token = encode(&jsonwebtoken::Header::new(jsonwebtoken::Algorithm::EdDSA), &claims, encoding_key).unwrap();

    token
}

pub fn decode_token(token: &str, decoding_key: &DecodingKey) -> Claims {
    let validation = Validation::new(Algorithm::EdDSA);
    let token_data = decode::<Claims>(token, decoding_key, &validation).unwrap();

    token_data.claims
}