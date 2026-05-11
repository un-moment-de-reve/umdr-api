use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}
