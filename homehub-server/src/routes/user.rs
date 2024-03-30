use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use homehub_db::queries::app_user::FilteredAppUserModel;

use crate::middleware::jwt_auth::JWTAuthMiddleware;

pub async fn get_me(
    Extension(jwt): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user: FilteredAppUserModel = jwt.user.into();
    Ok(Json(serde_json::json!({
        "status": "success",
        "user": user,
    })))
}
