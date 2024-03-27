use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use anyhow::Result;
use axum::{middleware as axum_middleware, response::IntoResponse, routing};
use axum::{Json, Router};

mod config;
mod middleware;
mod routes;
mod state;
mod util;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    dotenvy::dotenv()?;
    let config = config::Config::from_env();
    let db = homehub_db::get_database(config.database_url.as_str()).await?;
    let app_state = Arc::new(state::AppState { db, config });

    let app = Router::new()
        .route("/health", routing::get(health_check))
        .route("/auth/register", routing::post(routes::auth::register_user))
        .route("/auth/login", routing::post(routes::auth::login_user))
        .route(
            "/auth/refresh",
            routing::post(routes::auth::refresh_access_token),
        )
        .route(
            "/user",
            routing::get(routes::user::get_me).route_layer(axum_middleware::from_fn_with_state(
                app_state.clone(),
                middleware::jwt_auth::auth,
            )),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_check() -> impl IntoResponse {
    const MESSAGE: &str = "I'm alive!";

    Json(serde_json::json!({ "message": MESSAGE }))
}
