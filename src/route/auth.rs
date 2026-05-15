use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::{
    core::auth::{
        controllers::{login_handler, refresh_token_handler, register_handler},
        middleware::{require_access_token, require_refresh_token},
    },
    state::AppState,
};

pub fn auth_routes(app_state: AppState) -> Router<AppState> {
    let protected_by_refresh_routes = Router::new()
        .route("/refresh", get(refresh_token_handler))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            require_refresh_token,
        ));
    let protected_by_access_routes = Router::new()
        .route("/register", post(register_handler))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            require_access_token,
        ));
    let public = Router::new().route("/login", post(login_handler));

    Router::new()
        .nest("/auth", public)
        .nest("/auth", protected_by_access_routes)
        .nest("/auth", protected_by_refresh_routes)
}
