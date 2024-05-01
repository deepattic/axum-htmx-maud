use axum::{
    extract::rejection::FormRejection,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Form, Router,
};

pub fn app() -> Router {
    Router::new()
        // .route("/", get(return_index))
        .route("/check_health", get(StatusCode::OK))
        .route("/subscriptions", post(subscribe))
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(Form(_form): Form<FormData>) -> StatusCode {
    StatusCode::OK
}
