use crate::services::token_service::validate_token;
use axum::body::Body;
use axum::http::header::AUTHORIZATION;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;

pub async fn auth_middleware(req: Request<Body>, next: Next) -> Response {
    if let Some(header_value) = req.headers().get(AUTHORIZATION) {
        if let Ok(header_str) = header_value.to_str() {
            if header_str.len() >= 7 {
                let token = &header_str[7..];

                let is_valid_token = validate_token(token);

                match is_valid_token {
                    Ok(_) => next.run(req).await,
                    Err(_) => Response::new("Invalid token".to_string().into()),
                }
            } else {
                Response::new("Invalid token format".to_string().into())
            }
        } else {
            Response::new("Invalid header value".to_string().into())
        }
    } else {
        Response::new("Missing authorization header".to_string().into())
    }
}
