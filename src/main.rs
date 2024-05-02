use axum_htmx_maud::{configuration::get_configuration, startup::run};
use sqlx::PgPool;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read the Configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("failed to connecto  pg");
    let address = format!("127.0.0.1:{}", &configuration.application_port);
    let listener = TcpListener::bind(address).await.unwrap();
    run(listener, connection_pool).await
}
