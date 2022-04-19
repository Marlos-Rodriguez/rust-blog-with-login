pub mod posts;
pub mod users;

use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct SecretKey {
    pub enc_key: EncodingKey,
    pub dec_key: DecodingKey,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub jwt: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub description: String,
}
