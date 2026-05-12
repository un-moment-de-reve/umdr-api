use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

use crate::{
    core::{
        auth::{dto::TokenResponse, payloads::LoginPayload},
        health::dto::HealthCheckResponse,
        pricing::{
            dto::{PricingResponse, PricingsResponse},
            payloads::PricingUpdatePayload,
        },
    },
    utils::response::ApiResponse,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::core::auth::controllers::login_handler,
        crate::core::auth::controllers::refresh_token_handler,
        crate::core::health::controllers::health_check_handler,
        crate::core::pricing::controllers::get_pricings_handler,
        crate::core::pricing::controllers::update_pricing_handler,
    ),
    components(
        schemas(
            LoginPayload,
            TokenResponse,
            HealthCheckResponse,
            PricingUpdatePayload,
            PricingResponse,
            PricingsResponse,
            ApiResponse<TokenResponse>,
            ApiResponse<HealthCheckResponse>,
            ApiResponse<PricingResponse>,
            ApiResponse<PricingsResponse>
        )
    ),
    tags(
        (name = "Auth", description = "Authentification et gestion des tokens"),
        (name = "Health", description = "État de santé de l'API"),
        (name = "Pricing", description = "Gestion des tarifs")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}
