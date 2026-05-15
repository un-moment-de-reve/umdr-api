use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, ToSchema)]
pub struct RegisterPayload {
    pub username: String,
    pub password: String,
}
