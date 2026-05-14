use axum::{
    Json,
    extract::{Path, State},
};
use mongodb::bson::oid::ObjectId;

use crate::{
    core::pricing::{dto::PricingDeleteResponse, payloads::PricingCreatePayload},
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

#[utoipa::path(
    get,
    path = "/pricing",
    tag = "Pricing",
    responses(
        (
            status = 200,
            description = "Liste des tarifs",
            body = ApiResponse<PricingsResponse>
        )
    )
)]
pub async fn get_pricings_handler(State(state): State<AppState>) -> AppResult<PricingsResponse> {
    let pricings = state.mongo.database("umdr-db").collection("pricing");

    let response = services::get_pricings(pricings, state.verbose).await?;

    Ok(ApiResponse::success(response))
}

#[utoipa::path(
    patch,
    path = "/pricing/{id}",
    tag = "Pricing",
    security(
        ("bearer_auth" = [])
    ),
    params(
        (
            "id" = String,
            Path,
            description = "Identifiant MongoDB du tarif"
        )
    ),
    request_body = PricingUpdatePayload,
    responses(
        (
            status = 200,
            description = "Tarif mis à jour",
            body = ApiResponse<PricingResponse>
        ),
        (
            status = 400,
            description = "Identifiant de tarif invalide"
        ),
        (
            status = 404,
            description = "Tarif introuvable"
        )
    )
)]
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

#[utoipa::path(
    post,
    path = "/pricing",
    tag = "Pricing",
    security(
        ("bearer_auth" = [])
    ),
    request_body = PricingCreatePayload,
    responses(
        (
            status = 200,
            description = "Tarif créé",
            body = ApiResponse<PricingResponse>
        ),
        (
            status = 400,
            description = "Requête de création de tarif invalide"
        )
    )
)]
pub async fn create_pricing_handler(
    State(state): State<AppState>,
    Json(payload): Json<PricingCreatePayload>,
) -> AppResult<PricingResponse> {
    let pricings = state.mongo.database("umdr-db").collection("pricing");

    let response = services::create_pricing(pricings, payload, state.verbose).await?;

    Ok(ApiResponse::success(response))
}

#[utoipa::path(
    delete,
    path = "/pricing/{id}",
    tag = "Pricing",
    security(
        ("bearer_auth" = [])
    ),
    params(
        (
            "id" = String,
            Path,
            description = "Identifiant MongoDB du tarif"
        )
    ),
    responses(
        (
            status = 200,
            description = "Tarif supprimé",
            body = ApiResponse<PricingDeleteResponse>
        ),
        (
            status = 400,
            description = "Identifiant de tarif invalide"
        ),
        (
            status = 404,
            description = "Tarif introuvable"
        )
    )
)]
pub async fn delete_pricing_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<PricingDeleteResponse> {
    let object_id =
        ObjectId::parse_str(&id).map_err(|_| AppError::bad_request("Invalid pricing id"))?;

    let pricings = state.mongo.database("umdr-db").collection("pricing");

    let response = services::delete_pricing(pricings, object_id, state.verbose).await?;

    Ok(ApiResponse::success(response))
}
