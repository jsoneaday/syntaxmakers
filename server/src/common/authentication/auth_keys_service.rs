use chrono::{Utc, Duration};
use jsonwebtoken::{DecodingKey, EncodingKey};
use ring::signature::{Ed25519KeyPair, KeyPair};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{ Validation, encode, decode, Algorithm };
use crate::common::repository::user::models::RepoDeveloperOrEmployer;
use async_trait::async_trait;
use derive_more::{Error, Display};
use log::info;

pub const STANDARD_REFRESH_TOKEN_EXPIRATION: i64 = 60 * 60 * 24 * 30;
pub const STANDARD_ACCESS_TOKEN_EXPIRATION: i64 = 60 * 10; // todo: switch to 2 min once testing complete
pub const REFRESH_TOKEN_LABEL: &str = "refresh_token";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub role: RepoDeveloperOrEmployer
}

#[derive(Error, Display, Debug)]
pub enum AuthenticationError {
    #[display(fmt = "Authentication Failure")]
    AuthenticationFailure,
    #[display(fmt = "Authentication Failure from Database")]
    AuthenticationDbFailure
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

pub fn get_token(user_name: String, dev_or_emp: RepoDeveloperOrEmployer, encoding_key: &EncodingKey, exp_duration_seconds: Option<i64>) -> String {
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

pub struct AuthService;

#[async_trait]
pub trait Authenticator {
    /// Checks headers for Authorization and Bearer token
    /// @headers is a tuple: 0 is header name and 1 is header value
    async fn is_authenticated(&self, user_name: String, headers: Vec<(&str, &str)>, decoding_key: &DecodingKey) -> Result<bool, AuthenticationError>;
}

/// Check that user has already logged in and received their access token
#[async_trait]
impl Authenticator for AuthService {    
    async fn is_authenticated(&self, user_name: String, headers: Vec<(&str, &str)>, decoding_key: &DecodingKey) -> Result<bool, AuthenticationError> {
        let mut result: Result<bool, AuthenticationError> = Err(AuthenticationError::AuthenticationFailure);

        _ = headers.iter().for_each(|header| {
            let header_name = header.0;
            let header_val = header.1;
            
            if header_name.to_lowercase() == "authorization" {
                let bearer_items: Vec<&str> = header_val.split(' ').collect();
                let claims = decode_token(bearer_items.get(1).unwrap(), decoding_key);
                info!("claims {:?}", claims);
                info!("checking against user_name {}", user_name);
                if claims.sub == user_name {
                    if claims.exp >= (Utc::now().timestamp() as usize) {
                        result = Ok(true);
                    }
                }    
            }
        });

        result
    }
}
