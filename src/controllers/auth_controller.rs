use crate::config::db::get_connection;

use crate::models::user_model::{GetUserDtoConstructor, GetUsersDTO, LoginDTO, LoginResponseDTO};
use crate::repositories::user_repository::UserRepository;
use crate::services::password_service::is_valid;
use crate::services::token_service::generate_jwt;
use axum::http::StatusCode;
use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn login(user: Json<LoginDTO>) -> impl IntoResponse {
    let user_login_data = user;
    match get_connection() {
        Ok(mut conn) => {
            let user =
                UserRepository::find_by_email(&mut conn, &user_login_data.email.to_lowercase());
            match user {
                Ok(user_exist) => {
                    let verify_password = is_valid(&user_login_data.password, &user_exist.password);
                    if verify_password {
                        let user_to_token =
                            GetUsersDTO::new(user_exist.id, user_exist.email, user_exist.roles);
                        let token = generate_jwt(user_to_token);
                        match token {
                            Ok(token) => {
                                let response = LoginResponseDTO { token };
                                (
                                    StatusCode::OK,
                                    Json(json!({
                                        "status": StatusCode::OK.as_u16(),
                                        "message": "Login successfully",
                                        "data": response
                                    })),
                                )
                                    .into_response()
                            }
                            Err(_) => (
                                StatusCode::BAD_REQUEST,
                                Json(json!({
                                    "status": StatusCode::BAD_REQUEST.as_u16(),
                                    "message": "Generating JWT token error",
                                    "data": null,
                                })),
                            )
                                .into_response(),
                        }
                    } else {
                        (
                            StatusCode::BAD_REQUEST,
                            Json(json!({
                                "status": StatusCode::BAD_REQUEST.as_u16(),
                                "message": "Wrong password",
                                "data": null,
                            })),
                        )
                            .into_response()
                    }
                }
                Err(_) => (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "status": StatusCode::NOT_FOUND.as_u16(),
                        "message": "User not found",
                        "data": null,
                    })),
                )
                    .into_response(),
            }
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "message": "Internal server error",
                "data": null,
            })),
        )
            .into_response(),
    }
}
