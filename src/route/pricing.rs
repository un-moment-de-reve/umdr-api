use axum::{
    Router, middleware,
    routing::{delete, get, patch, post},
};

use crate::{
    core::{
        auth::middleware as auth_middleware,
        pricing::controllers::{
            create_pricing_handler, delete_pricing_handler, get_pricings_handler,
            update_pricing_handler,
        },
    },
    state::AppState,
};

pub fn pricing_routes(app_state: AppState) -> Router<AppState> {
    let protected_by_access_routes = Router::new()
        .route("/{id}", patch(update_pricing_handler))
        .route("/{id}", delete(delete_pricing_handler))
        .route("/", post(create_pricing_handler))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware::require_access_token,
        ));

    let public_routes = Router::new().route("/", get(get_pricings_handler));

    Router::new()
        .nest("/pricing", public_routes)
        .nest("/pricing", protected_by_access_routes)
}
