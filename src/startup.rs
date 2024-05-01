use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};

use crate::routes::subscribe;

pub fn app() -> Router {
    Router::new()
        .route("/check_health", get(StatusCode::OK))
        .route("/subscriptions", post(subscribe))
}
