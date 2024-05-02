use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tokio::net::TcpListener;

use crate::routes::subscribe;

pub async fn run(listener: TcpListener, db_pool: PgPool) {
    let app = Router::new()
        .route("/check_health", get(StatusCode::OK))
        .route("/subscriptions", post(subscribe))
        .with_state(db_pool);

    axum::serve(listener, app).await.unwrap();
}
