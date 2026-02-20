use crate::api::dto::{ApiError, Claims};
use axum::{
    extract::Request,
    http::{HeaderMap, header::AUTHORIZATION},
    middleware::Next,
    response::IntoResponse,
};
use jsonwebtoken::{DecodingKey, Validation, decode};

pub async fn auth(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, ApiError> {
    let auth_header = headers
        .get(AUTHORIZATION)
        .ok_or(ApiError::Unauthorized)?
        .to_str()?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(ApiError::Unauthorized)?;

    let key = std::env::var("SECRET_KEY")?;

    let token_msg = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(key.as_bytes()),
        &Validation::default(),
    )?;

    request.extensions_mut().insert(token_msg.claims);

    Ok(next.run(request).await)
}
