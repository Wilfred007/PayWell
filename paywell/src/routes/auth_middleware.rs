use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;
use crate::config::Config;

#[derive(Debug, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct AuthUser {
    pub user_id: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (axum::http::StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or((axum::http::StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;

        if !auth_header.starts_with("Bearer ") {
            return Err((axum::http::StatusCode::UNAUTHORIZED, "Invalid token".to_string()));
        }

        let token = &auth_header[7..];
        let config = Config::from_env();

        let decoded = decode::<Claims>(
            token,
            &DecodingKey::from_secret(config.jwt_secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| (axum::http::StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

        Ok(AuthUser { user_id: decoded.claims.sub })
    }
}
