use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWTAuthMiddleware {
    pub user: homehub_db::app_user::Model,
}

pub async fn auth(
    State(data): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let access_token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| auth_value.strip_prefix("Bearer "));

    let access_token = access_token.ok_or_else(|| {
        let error_response = ErrorResponse {
            status: "error",
            message: "No valid token found".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(error_response))
    })?;

    let access_token = match homehub_core::token::verify_jwt_token(
        data.config.access_token_public_key.to_owned(),
        access_token,
    ) {
        Ok(token) => token,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    status: "error",
                    message: "Unauthorized".to_owned(),
                }),
            ))
        }
    };

    let user_id = access_token.user_id;

    let user = match homehub_db::app_user::find_by_id(user_id, &data.db).await {
        Ok(user) => match user {
            Some(user) => user,
            None => {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(ErrorResponse {
                        status: "error",
                        message: "Unauthorized".to_owned(),
                    }),
                ))
            }
        },
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    status: "error",
                    message: "Unauthorized".to_owned(),
                }),
            ))
        }
    };

    req.extensions_mut().insert(JWTAuthMiddleware { user });

    Ok(next.run(req).await)
}
