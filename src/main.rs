use axum_htmx_maud::{configuration::get_configuration, startup::app};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read the Configuration");
    let address = format!("127.0.0.1:{}", &configuration.application_port);
    let listener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}
