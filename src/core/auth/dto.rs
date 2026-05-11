use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub tokens: Tokens,
}

#[derive(Debug, Serialize)]
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
