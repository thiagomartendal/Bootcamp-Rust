use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // Conteúdo do token
    exp: usize,  // Tempo de expiração do token
}

pub fn generate_token(adm_id: u32) -> String {
    let expiration_time = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("Timestamp inválido")
        .timestamp();

    let claims = Claims {
        sub: adm_id.to_string(),
        exp: expiration_time as usize,
    };

    let secret = env::var("SECRET_JWT").unwrap_or_else(|_| String::from("chave_secreta"));
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

pub fn verify_token(token: &str) -> bool {
    let secret = env::var("SECRET_JWT").unwrap_or_else(|_| String::from("chave_secreta"));
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .is_ok()
}
