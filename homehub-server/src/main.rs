use axum::routing;
use axum::Router;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let app = Router::new().route("/", routing::get(root));
    homehub_models::connect_db().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
