use std::time::Instant;

use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::state::AppState;

pub async fn request_logger(
    State(state): State<AppState>,
    req: Request<axum::body::Body>,
    next: Next,
) -> Response {
    if !state.verbose {
        return next.run(req).await;
    }

    let method = req.method().clone();
    let uri = req.uri().clone();
    let started_at = Instant::now();

    println!(
        "{} {:<7} {}",
        cyan("[REQ]"),
        method.to_string(),
        uri
    );

    let response = next.run(req).await;

    let status = response.status();
    let duration = started_at.elapsed();

    println!(
        "{} {:<7} {:<35} {:<15} {}",
        status_label(status),
        method.to_string(),
        uri.to_string(),
        status.to_string(),
        format_duration(duration)
    );

    response
}

fn status_label(status: StatusCode) -> String {
    if status.is_success() {
        green("[DONE]")
    } else if status.is_client_error() {
        yellow("[FAIL]")
    } else if status.is_server_error() {
        red("[ERR]")
    } else {
        blue("[INFO]")
    }
}

fn format_duration(duration: std::time::Duration) -> String {
    let millis = duration.as_secs_f64() * 1000.0;

    if millis < 10.0 {
        green(&format!("{:.3}ms", millis))
    } else if millis < 100.0 {
        blue(&format!("{:.3}ms", millis))
    } else if millis < 500.0 {
        yellow(&format!("{:.3}ms", millis))
    } else {
        red(&format!("{:.3}ms", millis))
    }
}

fn green(value: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", value)
}

fn yellow(value: &str) -> String {
    format!("\x1b[33m{}\x1b[0m", value)
}

fn red(value: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", value)
}

fn blue(value: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", value)
}

fn cyan(value: &str) -> String {
    format!("\x1b[36m{}\x1b[0m", value)
}