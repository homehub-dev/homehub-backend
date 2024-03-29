use crate::state::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub(crate) struct RegisterUserPayload {
    name: String,
    email: String,
    password: String,
}

// #[debug_handler]
pub(crate) async fn register_user(
    State(data): State<Arc<AppState>>,
    Json(payload): Json<RegisterUserPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    homehub_core::user::register_user(
        &data.db,
        &payload.name,
        &payload.email,
        &payload.password,
    )
    .await
    .map(|user| {
        Json(serde_json::json!({
            "status": "success",
            "user": user,
        }))
    })
    .map_err(|e| match e {
        homehub_core::user::RegisterUserError::UserAlreadyExists { email } => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "status": "error",
                "message": format!("User with email {} already exists", email),
            })),
        ),
        homehub_core::user::RegisterUserError::DbError(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "status": "error",
                "message": "Failed to insert user",
            })),
        ),
        homehub_core::user::RegisterUserError::CouldNotHashError => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "status": "error",
                "message": "Could not hash password",
            })),
        ),
    })
}

#[derive(Deserialize)]
pub(crate) struct LoginUserPayload {
    email: String,
    password: String,
}

pub(crate) async fn login_user(
    State(data): State<Arc<AppState>>,
    Json(payload): Json<LoginUserPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    homehub_core::user::login_user(
        &data.db,
        &payload.email,
        &payload.password,
        &data.config,
    )
    .await
    .map(
        |homehub_core::user::Tokens {
             access_token,
             refresh_token,
         }| {
            Json(serde_json::json!({
            "status": "success",
            "access_token": access_token, 
            "refresh_token": refresh_token}))
        },
    )
    .map_err(translate_login_error)
}

#[derive(Deserialize)]
pub(crate) struct RefreshAccessTokenPayload {
    refresh_token: String,
}

pub(crate) async fn refresh_access_token(
    State(data): State<Arc<AppState>>,
    Json(payload): Json<RefreshAccessTokenPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    homehub_core::user::refresh_access_token(
        &payload.refresh_token,
        &data.config,
    )
    .await
    .map(
        |homehub_core::user::Tokens {
             access_token,
             refresh_token,
         }| {
            Json(serde_json::json!({
                "status": "success",
                "access_token": access_token,
                "refresh_token": refresh_token,
            }))
        },
    )
    .map_err(translate_login_error)
}

fn translate_login_error(
    e: homehub_core::user::LoginUserError,
) -> (StatusCode, Json<serde_json::Value>) {
    match e {
        homehub_core::user::LoginUserError::UserNotFoundError(email) => (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "status": "error",
                "message": format!("User {} not found", email),
            })),
        ),
        homehub_core::user::LoginUserError::InvalidCredentialError => (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "status": "error",
                "message": "Invalid credentials",
            })),
        ),
        homehub_core::user::LoginUserError::DbError(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "status": "error",
            })),
        ),
        homehub_core::user::LoginUserError::CouldNotHashError => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "status": "error",
            })),
        ),
        homehub_core::user::LoginUserError::TokenGenerationError => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "status": "error",
            })),
        ),
    }
}
