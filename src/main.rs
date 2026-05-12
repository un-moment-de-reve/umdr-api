// use calendly to schedule meetings with users
//stitch
// delete redis and use mongodb for refresh-tokens
use axum::Router;
use axum::middleware;
use dotenv::dotenv;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use umdr_api::middleware::request_logger::request_logger;
use umdr_api::route::health::health_routes;
use umdr_api::utils::openapi::ApiDoc;
use umdr_api::{
    route::{auth::auth_routes, pricing::pricing_routes},
    state::{AppState, SecretStore},
    utils::config::CliConfig,
    utils::seed::{seed_default_pricing, seed_default_user},
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = CliConfig::from_args();

    let mongo_uri = std::env::var("MONGO_URI").expect("missing MONGO_URI");
    let dev_mode = std::env::var("DEV_MODE").unwrap_or_else(|_| "false".to_string()) == "true";
    let mongo = mongodb::Client::with_uri_str(&mongo_uri).await.unwrap();
    let db = mongo.database("umdr-db");
    if cli.seed {
        seed_default_user(&db)
            .await
            .expect("Failed to seed default user");
        seed_default_pricing(&db)
            .await
            .expect("Failed to seed default pricing");
    }
    let secret_store = SecretStore;

    let app_state = AppState {
        mongo,
        secret_store,
        started_at: std::time::Instant::now(),
        verbose: cli.verbose,
    };

    let auth_routes = auth_routes(app_state.clone());
    let pricing_routes = pricing_routes(app_state.clone());
    let health_routes = health_routes();
    let mut app = Router::new()
        .merge(auth_routes)
        .merge(pricing_routes)
        .merge(health_routes)
        .with_state(app_state.clone())
        .route_layer(middleware::from_fn_with_state(app_state, request_logger));
    if dev_mode {
        app = app
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
    }

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
