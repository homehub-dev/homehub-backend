use homehub_db::{app_user::FilteredAppUserModel, DatabaseConnection};

use argon2::{
    password_hash::SaltString, Argon2, PasswordHash, PasswordHasher,
    PasswordVerifier,
};
use rand_core::OsRng;
use thiserror::Error;

use crate::config;

#[derive(Debug, Error)]
pub enum RegisterUserError {
    #[error("User with email already exists")]
    UserAlreadyExists { email: String },
    #[error("Failed to insert user")]
    DbError(anyhow::Error),
    #[error("Could not hash password")]
    CouldNotHashError,
}
pub async fn register_user(
    db: &DatabaseConnection,
    name: &str,
    email: &str,
    password: &str,
) -> Result<FilteredAppUserModel, RegisterUserError> {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), salt.as_salt())
        .map(|hash| hash.to_string())
        .map_err(|_| RegisterUserError::CouldNotHashError)?;

    if let Ok(Some(_)) =
        homehub_db::app_user::find_user_by_email(email, db).await
    {
        return Err(RegisterUserError::UserAlreadyExists {
            email: email.to_string(),
        });
    };

    let user = homehub_db::app_user::create_user(
        name,
        email,
        &password_hash,
        None,
        db,
    )
    .await
    .map_err(RegisterUserError::DbError)?;
    Ok(user.into())
}

#[derive(Debug, Error)]
pub enum LoginUserError {
    #[error("User not found")]
    UserNotFoundError(String),
    #[error("Invalid credentials")]
    InvalidCredentialError,
    #[error("Failed to query database")]
    DbError(anyhow::Error),
    #[error("Could not hash password")]
    CouldNotHashError,
    #[error("Token generation failed")]
    TokenGenerationError,
}

pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}
pub async fn login_user(
    db: &DatabaseConnection,
    email: &str,
    password: &str,
    config: &config::Config,
) -> Result<Tokens, LoginUserError> {
    let user = homehub_db::app_user::find_user_by_email(email, db)
        .await
        .map_err(LoginUserError::DbError)?
        .ok_or_else(|| LoginUserError::UserNotFoundError(email.to_string()))?;

    let matches = match PasswordHash::new(&user.password_hash) {
        Ok(hash) => {
            Argon2::default().verify_password(password.as_bytes(), &hash)
        }
        Err(_) => return Err(LoginUserError::CouldNotHashError),
    };

    match matches {
        Ok(_) => generate_tokens(user.id, config),
        Err(_) => Err(LoginUserError::InvalidCredentialError),
    }
}

pub async fn refresh_access_token(
    refresh_token: &str,
    config: &config::Config,
) -> Result<Tokens, LoginUserError> {
    crate::token::verify_jwt_token(
        config.refresh_token_public_key.clone(),
        refresh_token,
    )
    .map_err(|_| LoginUserError::InvalidCredentialError)
    .map(|token_detail| generate_tokens(token_detail.user_id, config))?
}

fn generate_tokens(
    user_id: i32,
    config: &config::Config,
) -> Result<Tokens, LoginUserError> {
    let access_token = crate::token::generate_jwt_token(
        user_id,
        config.access_token_max_age,
        config.access_token_private_key.clone(),
    );
    let refresh_token = crate::token::generate_jwt_token(
        user_id,
        config.refresh_token_max_age,
        config.refresh_token_private_key.clone(),
    );
    let tokens = vec![access_token, refresh_token]
        .into_iter()
        .map(|token_result| {
            token_result.map(|token_detail| {
                token_detail.token.unwrap_or("".to_string())
            })
        })
        .collect::<Result<Vec<String>, _>>()
        .map_err(|_| LoginUserError::TokenGenerationError);

    match tokens {
        Ok(tokens) => Ok(Tokens {
            access_token: tokens[0].clone(),
            refresh_token: tokens[1].clone(),
        }),
        Err(e) => Err(e),
    }
}
