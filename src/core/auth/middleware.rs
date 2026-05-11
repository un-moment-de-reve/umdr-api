use axum::{extract::State, http::Request, middleware::Next, response::Response};

use crate::{
    core::auth::{token::decode_jwt, utils::jwt::is_refresh_valid},
    state::AppState,
    utils::error::AppError,
};

pub async fn require_access_token(
    State(state): State<AppState>,
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, AppError> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::unauthorized("Missing token"))?;

    let claims = decode_jwt(token, &state.secret_store)
        .map_err(|_| AppError::unauthorized("Invalid token"))?;

    if claims.token_type != "access" {
        return Err(AppError::unauthorized("Expected access token"));
    }

    req.extensions_mut().insert(claims.sub.clone());
    Ok(next.run(req).await)
}

pub async fn require_refresh_token(
    State(state): State<AppState>,
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, AppError> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::unauthorized("Missing token"))?;

    let claims = decode_jwt(token, &state.secret_store)
        .map_err(|_| AppError::unauthorized("Invalid token"))?;

    if claims.token_type != "refresh" {
        return Err(AppError::unauthorized("Expected refresh token"));
    }

    let jti = claims
        .jti
        .as_ref()
        .ok_or_else(|| AppError::unauthorized("Missing jti"))?;

    if !is_refresh_valid(&state.mongo, &claims.sub, jti).await {
        return Err(AppError::unauthorized("Invalid token"));
    }

    req.extensions_mut().insert(claims.sub.clone());
    Ok(next.run(req).await)
}
