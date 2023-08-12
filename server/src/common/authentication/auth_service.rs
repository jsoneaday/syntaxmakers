use jsonwebtoken::{DecodingKey, EncodingKey};
use ring::signature::{Ed25519KeyPair, KeyPair};
use serde::{Deserialize, Serialize};
use derive_more::{Display, Error};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Error, Display, Debug)]
pub enum AuthError {
    #[display(fmt = "Unable to create authentication keys")]
    UnableToCreateKeys
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
