use axum_htmx_maud::routers::app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let socketadder = listener.local_addr();
    let port = socketadder.unwrap().port();
    println!("application is running on port: {}", &port);
    axum::serve(listener, app()).await.unwrap();
}
