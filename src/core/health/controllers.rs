use axum::extract::State;

use crate::{
    core::health::{dto::HealthCheckResponse, services},
    state::AppState,
    utils::response::{ApiResponse, AppResult},
};

#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (
            status = 200,
            description = "API opérationnelle",
            body = ApiResponse<HealthCheckResponse>
        )
    )
)]
pub async fn health_check_handler(State(state): State<AppState>) -> AppResult<HealthCheckResponse> {
    let response = services::health_check(state.started_at)?;
    Ok(ApiResponse::success(response))
}
