use axum::{Json, extract::State, http::HeaderMap};

use crate::{
    core::auth::{dto::TokenResponse, payloads::LoginPayload, services},
    state::AppState,
    utils::{
        error::AppError,
        response::{ApiResponse, AppResult},
    },
};

#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "Auth",
    request_body = LoginPayload,
    responses(
        (
            status = 200,
            description = "Connexion réussie",
            body = ApiResponse<TokenResponse>
        ),
        (
            status = 401,
            description = "Identifiants invalides"
        )
    )
)]
pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> AppResult<TokenResponse> {
    let response = services::login(
        state.get_user_collection(),
        state.secret_store.clone(),
        payload.username,
        payload.password,
        state.verbose,
    )
    .await?;

    Ok(ApiResponse::success(response))
}

#[utoipa::path(
    get,
    path = "/auth/refresh",
    tag = "Auth",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (
            status = 200,
            description = "Token rafraîchi",
            body = ApiResponse<TokenResponse>
        ),
        (
            status = 401,
            description = "Refresh token manquant ou invalide"
        )
    )
)]
pub async fn refresh_token_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> AppResult<TokenResponse> {
    let old_token = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::unauthorized("Missing refresh token"))?;

    let response = services::refresh_token(
        state.get_user_collection(),
        state.secret_store.clone(),
        old_token,
        state.verbose,
    )
    .await?;

    Ok(ApiResponse::success(response))
}
