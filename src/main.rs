use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let server_address = "0.0.0.0:3000";

    let app = Router::new()
                .route("/", get(hello_world));
    
    let listener = tokio::net::TcpListener::bind(&server_address).await.unwrap();

    println!("Server running at {}", server_address);

    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> &'static str {
    "Hello world!"
}