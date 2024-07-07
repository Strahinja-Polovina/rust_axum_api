use axum::body::Body;
use crate::constants::middleware_constants::{
    BEARER_WITH_SPACE, INVALID_HEADER_VALUE_ERROR, INVALID_TOKEN_ERROR, INVALID_TOKEN_FORMAT_ERROR,
    MISSING_AUTHORIZATION_HEADER_ERROR,
};
use crate::services::token_service::validate_token;
use axum::http::header::AUTHORIZATION;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;

pub async fn middleware(req: Request<Body>, next: Next) -> Response {
    if let Some(header_value) = req.headers().get(AUTHORIZATION) {
        if let Ok(header_str) = header_value.to_str() {
            if header_str.starts_with(BEARER_WITH_SPACE) {
                let token = &header_str[7..];

                let is_valid_token = validate_token(token);

                match is_valid_token {
                    Ok(_) => next.run(req).await,
                    Err(_) => Response::new(INVALID_TOKEN_ERROR.to_string().into()),
                }
            } else {
                Response::new(INVALID_TOKEN_FORMAT_ERROR.to_string().into())
            }
        } else {
            Response::new(INVALID_HEADER_VALUE_ERROR.to_string().into())
        }
    } else {
        Response::new(MISSING_AUTHORIZATION_HEADER_ERROR.to_string().into())
    }
}
