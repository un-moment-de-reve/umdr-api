use axum::{
    Json,
    extract::{Path, State},
};
use mongodb::bson::oid::ObjectId;

use crate::{
    state::AppState,
    utils::{
        error::AppError,
        response::{ApiResponse, AppResult},
    },
};

use super::{
    dto::{PricingResponse, PricingsResponse},
    payloads::PricingUpdatePayload,
    services,
};

pub async fn get_pricings_handler(State(state): State<AppState>) -> AppResult<PricingsResponse> {
    let pricings = state.mongo.database("umdr-db").collection("pricing");

    let response = services::get_pricings(pricings, state.verbose).await?;

    Ok(ApiResponse::success(response))
}

pub async fn update_pricing_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<PricingUpdatePayload>,
) -> AppResult<PricingResponse> {
    let object_id =
        ObjectId::parse_str(&id).map_err(|_| AppError::bad_request("Invalid pricing id"))?;

    let pricings = state.mongo.database("umdr-db").collection("pricing");

    let response = services::update_pricing(pricings, object_id, payload, state.verbose).await?;

    Ok(ApiResponse::success(response))
}
