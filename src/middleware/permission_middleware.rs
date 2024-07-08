use crate::services::token_service::validate_token;
use axum::body::Body;
use axum::http::header::AUTHORIZATION;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use std::sync::Arc;

pub async fn permission_middleware(req: Request<Body>, next: Next) -> Response {
    if let Some(header_value) = req.headers().get(AUTHORIZATION) {
        if let Ok(header_str) = header_value.to_str() {
            if header_str.starts_with("Bearer ") {
                let token = &header_str[7..];

                let is_valid_token = validate_token(token);
                let user_id = req.uri().path().split('/').nth(2);

                match is_valid_token {
                    Ok(claims) => match user_id {
                        Some(id) => match id.parse::<i32>() {
                            Ok(id) => {
                                if id == claims.sub || claims.roles == "admin".to_string() {
                                    let mut req = req;
                                    req.extensions_mut().insert(Arc::new(claims));
                                    next.run(req).await
                                } else {
                                    Response::new("Don't have permission".to_string().into())
                                }
                            }
                            Err(_) => Response::new("Invalid user id param".to_string().into()),
                        },
                        None => next.run(req).await,
                    },
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
