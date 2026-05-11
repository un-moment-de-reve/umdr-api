use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::state::SecretStore;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub jti: Option<String>,
    pub exp: usize,
    pub token_type: String,
}

pub fn create_tokens(user_id: &str, secret_store: &SecretStore) -> (String, String, String) {
    let access_token = create_access_token(user_id, secret_store);
    let (refresh_token, jti) = create_refresh_token(user_id, secret_store);
    (access_token, refresh_token, jti)
}

pub fn create_access_token(user_id: &str, secret_store: &SecretStore) -> String {
    create_jwt(user_id, secret_store, 15 * 60, "access", None)
}

pub fn create_refresh_token(user_id: &str, secret_store: &SecretStore) -> (String, String) {
    let jti = Uuid::new_v4().to_string();
    let token = create_jwt(
        user_id,
        secret_store,
        7 * 24 * 60 * 60,
        "refresh",
        Some(jti.clone()),
    );
    (token, jti)
}

fn create_jwt(
    user_id: &str,
    secret_store: &SecretStore,
    expiration_secs: i64,
    token_type: &str,
    jti: Option<String>,
) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(expiration_secs))
        .unwrap()
        .timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_owned(),
        jti,
        exp: expiration,
        token_type: token_type.to_string(),
    };

    let secret = secret_store.get("JWT_SECRET").expect("missing jwt_secret");
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .expect("JWT creation failed")
}

pub fn decode_jwt(
    token: &str,
    secret_store: &SecretStore,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = secret_store
        .get("JWT_SECRET")
        .expect("JWT_SECRET not found in Secrets.toml");
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(token_data.claims)
}
