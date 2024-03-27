use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
// use axum::debug_handler;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use rand_core::OsRng;
use serde::Deserialize;
use std::sync::Arc;

use crate::{state::AppState, util::token};

#[derive(Deserialize)]
pub struct RegisterUserPayload {
    name: String,
    email: String,
    password: String,
}

// #[debug_handler]
pub async fn register_user(
    State(data): State<Arc<AppState>>,
    Json(payload): Json<RegisterUserPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Error while hashing password: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })
        .map(|hash| hash.to_string())?;

    let user = homehub_db::app_user::create_user(
        &payload.name,
        &payload.email,
        &password_hash,
        None,
        &data.db,
    )
    .await
    .map_err(|e| {
        tracing::error!("Failed to insert user: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "status": "error",
                "message": "Failed to insert user",
            })),
        )
    })?;

    let user: homehub_db::app_user::FilteredAppUserModel = user.into();

    Ok(Json(serde_json::json!({
        "status": "success",
        "user": user,
    })))
}

pub async fn login_user(
    State(data): State<Arc<AppState>>,
    Json(payload): Json<RegisterUserPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user = homehub_db::app_user::find_user_by_email(&payload.email, &data.db)
        .await
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "status": "error",
                    "message": "Failed to find user",
                })),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "status": "error",
                    "message": "User not found",
                })),
            )
        })?;

    let matches = match PasswordHash::new(&user.password_hash) {
        Ok(hash) => Argon2::default()
            .verify_password(payload.password.as_bytes(), &hash)
            .is_ok(),
        Err(_) => false,
    };

    if !matches {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "status": "error",
                "message": "Invalid password",
            })),
        ));
    }

    let access_token = generate_token(
        user.id,
        data.config.access_token_max_age.to_owned(),
        data.config.access_token_private_key.to_owned(),
    )?;
    let refresh_token = generate_token(
        user.id,
        data.config.refresh_token_max_age.to_owned(),
        data.config.refresh_token_private_key.to_owned(),
    )?;

    Ok(Json(serde_json::json!({
        "status": "success",
        "access_token": access_token,
        "refresh_token": refresh_token,
    })))
}

pub async fn refresh_access_token(
    State(data): State<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let refresh_token = payload
        .get("refresh_token")
        .and_then(|t| t.as_str())
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "status": "error",
                    "message": "refresh_token is required",
                })),
            )
        })?;

    let token_details = token::verify_jwt_token(
        data.config.refresh_token_public_key.to_owned(),
        refresh_token,
    )
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("error verifying token: {}", e),
        });
        (StatusCode::UNAUTHORIZED, Json(error_response))
    })?;

    let access_token = generate_token(
        token_details.user_id,
        data.config.access_token_max_age.to_owned(),
        data.config.access_token_private_key.to_owned(),
    )?;

    let refresh_token = generate_token(
        token_details.user_id,
        data.config.refresh_token_max_age.to_owned(),
        data.config.refresh_token_private_key.to_owned(),
    )?;

    Ok(Json(serde_json::json!({
        "status": "success",
        "access_token": access_token,
        "refresh_token": refresh_token,
    })))
}

fn generate_token(
    user_id: i32,
    max_age: i64,
    private_key: String,
) -> Result<token::TokenDetails, (StatusCode, Json<serde_json::Value>)> {
    token::generate_jwt_token(user_id, max_age, private_key).map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("error generating token: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })
}
