use crate::config::db::get_connection;
use crate::constants::controller_constants::{
    DATA, GENERATING_JWT_ERROR, INTERNAL_SERVER_ERROR, LOGIN_SUCCESSFULLY_MESSAGE, MESSAGE, STATUS,
    USER_NOT_FOUND_ERROR, WRONG_PASSWORD_ERROR,
};
use crate::models::user_model::{GetUsersDTO, LoginDTO, LoginResponseDTO};
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
                        let user_to_token = GetUsersDTO {
                            id: user_exist.id,
                            email: user_exist.email,
                        };
                        let token = generate_jwt(user_to_token);
                        match token {
                            Ok(token) => {
                                let response = LoginResponseDTO { token };
                                (
                                    StatusCode::OK,
                                    Json(json!({
                                        STATUS: StatusCode::OK.as_u16(),
                                        MESSAGE: LOGIN_SUCCESSFULLY_MESSAGE,
                                        DATA: response
                                    })),
                                )
                                    .into_response()
                            }
                            Err(_) => (
                                StatusCode::BAD_REQUEST,
                                Json(json!({
                                    STATUS: StatusCode::BAD_REQUEST.as_u16(),
                                    MESSAGE: GENERATING_JWT_ERROR,
                                    DATA: null,
                                })),
                            )
                                .into_response(),
                        }
                    } else {
                        (
                            StatusCode::BAD_REQUEST,
                            Json(json!({
                                STATUS: StatusCode::BAD_REQUEST.as_u16(),
                                MESSAGE: WRONG_PASSWORD_ERROR,
                                DATA: null,
                            })),
                        )
                            .into_response()
                    }
                }
                Err(_) => (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        STATUS: StatusCode::NOT_FOUND.as_u16(),
                        MESSAGE: USER_NOT_FOUND_ERROR,
                        DATA: null,
                    })),
                )
                    .into_response(),
            }
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                STATUS: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                MESSAGE: INTERNAL_SERVER_ERROR,
                DATA: null,
            })),
        )
            .into_response(),
    }
}
