use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenDetails {
    pub token: Option<String>,
    pub user_id: Uuid,
    pub expires_in: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub nbf: i64,
}

pub fn generate_jwt_token(
    user_id: uuid::Uuid,
    ttl: i64,
    private_key: String,
) -> Result<TokenDetails, jsonwebtoken::errors::Error> {
    let bytes_private_key =
        general_purpose::STANDARD.decode(private_key).unwrap();
    let decoded_private_key = String::from_utf8(bytes_private_key).unwrap();

    let now = chrono::Utc::now();
    let mut token_details = TokenDetails {
        user_id,
        expires_in: Some((now + chrono::Duration::minutes(ttl)).timestamp()),
        token: None,
    };

    let claims = TokenClaims {
        sub: token_details.user_id.to_string(),
        exp: token_details.expires_in.unwrap(),
        iat: now.timestamp(),
        nbf: now.timestamp(),
    };

    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256);
    let token = jsonwebtoken::encode(
        &header,
        &claims,
        &jsonwebtoken::EncodingKey::from_rsa_pem(
            decoded_private_key.as_bytes(),
        )?,
    )?;
    token_details.token = Some(token);
    Ok(token_details)
}

pub fn verify_jwt_token(
    public_key: String,
    token: &str,
) -> anyhow::Result<TokenDetails> {
    let bytes_public_key =
        general_purpose::STANDARD.decode(public_key).unwrap();
    let decoded_public_key = String::from_utf8(bytes_public_key).unwrap();

    let validation =
        jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);

    let decoded = jsonwebtoken::decode::<TokenClaims>(
        token,
        &jsonwebtoken::DecodingKey::from_rsa_pem(
            decoded_public_key.as_bytes(),
        )?,
        &validation,
    )?;
    Ok(TokenDetails {
        token: None,
        user_id: decoded.claims.sub.parse()?,
        expires_in: None,
    })
}
