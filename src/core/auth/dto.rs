use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct TokenResponse {
    pub tokens: Tokens,
}

#[derive(Serialize, ToSchema)]
pub struct Tokens {
    pub access: String,
    pub refresh: String,
}

impl TokenResponse {
    pub fn new(access: String, refresh: String) -> Self {
        Self {
            tokens: Tokens { access, refresh },
        }
    }
}
