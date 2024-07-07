use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json as Json};
use serde_json::json;

use crate::messages::controller_constants::{
    INTERNAL_SERVER_ERROR, USER_CREATED_MESSAGE, USER_CREATE_BAD_REQUEST_ERROR,
    USER_DELETED_SUCCESSFULLY_MESSAGE, USER_EMAIL_EXISTS_ERROR, USER_DELETE_ERROR,
    USER_UPDATED_ERROR_MESSAGE, USER_UPDATED_SUCCESSFULLY_MESSAGE, USER_FETCH_SUCCESSFUL_MESSAGE, USER_NOT_FOUND_ERROR
};
use crate::models::user_model::{CreateUserDTO, UpdateUserDTO};
use crate::{
    config::db::get_connection,
    repositories::user_repository::UserRepository,
};
use crate::messages::controller_constants::{DATA, MESSAGE, STATUS};

pub async fn get_users() -> impl IntoResponse {
    match get_connection() {
        Ok(mut conn) => match UserRepository::find_all(&mut conn) {
            Ok(users) => (
                StatusCode::OK,
                Json(json!({
                    STATUS: StatusCode::OK.as_u16(),
                    MESSAGE: USER_FETCH_SUCCESSFUL_MESSAGE,
                    DATA: users,
                })),
            )
                .into_response(),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    STATUS: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    MESSAGE: INTERNAL_SERVER_ERROR,
                    DATA: null,
                })),
            )
                .into_response(),
        },
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

pub async fn get_user(Path(user_id): Path<i32>) -> impl IntoResponse {
    match get_connection() {
        Ok(mut conn) => match UserRepository::find_one(&mut conn, user_id) {
            Ok(user) => (
                StatusCode::OK,
                Json(json!({
                    STATUS: StatusCode::OK.as_u16(),
                    MESSAGE: USER_FETCH_SUCCESSFUL_MESSAGE,
                    DATA: user,
                })),
            )
                .into_response(),
            Err(_) => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    STATUS: StatusCode::NOT_FOUND.as_u16(),
                    MESSAGE: USER_NOT_FOUND_ERROR,
                    DATA: null,
                })),
            )
                .into_response(),
        },
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

pub async fn create_user(Json(new_user): Json<CreateUserDTO>) -> impl IntoResponse {
    match get_connection() {
        Ok(mut conn) => {
            match UserRepository::find_by_email(&mut conn, &new_user.email.to_lowercase()) {
                Ok(_) => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        STATUS: StatusCode::BAD_REQUEST.as_u16(),
                        MESSAGE: USER_EMAIL_EXISTS_ERROR,
                        DATA: null,
                    })),
                )
                    .into_response(),
                Err(_) => match UserRepository::create_user(&mut conn, new_user) {
                    Ok(created_user) => (
                        StatusCode::CREATED,
                        Json(json!({
                            STATUS: StatusCode::CREATED.as_u16(),
                            MESSAGE: USER_CREATED_MESSAGE,
                            DATA: created_user,
                        })),
                    )
                        .into_response(),
                    Err(_) => (
                        StatusCode::BAD_REQUEST,
                        Json(json!({
                            STATUS: StatusCode::BAD_REQUEST.as_u16(),
                            MESSAGE: USER_CREATE_BAD_REQUEST_ERROR,
                            DATA: null,
                        })),
                    )
                        .into_response(),
                },
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

pub async fn delete_user(Path(user_id): Path<i32>) -> impl IntoResponse {
    match get_connection() {
        Ok(mut conn) => {
            let user_deleted = UserRepository::find_one(&mut conn, user_id);
            match user_deleted {
                Ok(_) => match UserRepository::delete_user(&mut conn, user_id) {
                    Ok(_) => (
                        StatusCode::OK,
                        Json(json!({
                            STATUS: StatusCode::OK.as_u16(),
                            MESSAGE: USER_DELETED_SUCCESSFULLY_MESSAGE,
                            DATA: user_deleted.unwrap(),
                        })),
                    )
                        .into_response(),
                    Err(_) => (
                        StatusCode::BAD_REQUEST,
                        Json(json!({
                            STATUS: StatusCode::BAD_REQUEST.as_u16(),
                            MESSAGE: USER_DELETE_ERROR,
                            DATA: null,
                        })),
                    )
                        .into_response(),
                },
                Err(_) => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        STATUS: StatusCode::BAD_REQUEST.as_u16(),
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

pub async fn update_user(
    Path(user_id): Path<i32>,
    Json(updated_user): Json<UpdateUserDTO>,
) -> impl IntoResponse {
    match get_connection() {
        Ok(mut conn) => match UserRepository::find_one(&mut conn, user_id) {
            Ok(_) => match UserRepository::find_by_email(&mut conn, &updated_user.email) {
                Ok(_) => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        STATUS: StatusCode::BAD_REQUEST.as_u16(),
                        MESSAGE: USER_EMAIL_EXISTS_ERROR,
                        DATA: null,
                    })),
                )
                    .into_response(),
                Err(_) => {
                    let updated_user =
                        UserRepository::update_user(&mut conn, user_id, updated_user);
                    match updated_user {
                        Ok(user) => (
                            StatusCode::OK,
                            Json(json!({
                                STATUS: StatusCode::OK.as_u16(),
                                MESSAGE: USER_UPDATED_SUCCESSFULLY_MESSAGE,
                                DATA: user,
                            })),
                        )
                            .into_response(),
                        Err(_) => (
                            StatusCode::BAD_REQUEST,
                            Json(json!({
                                STATUS: StatusCode::BAD_REQUEST.as_u16(),
                                MESSAGE: USER_UPDATED_ERROR_MESSAGE,
                                DATA: null,
                            })),
                        )
                            .into_response(),
                    }
                }
            },
            Err(_) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    STATUS: StatusCode::BAD_REQUEST.as_u16(),
                    MESSAGE: USER_NOT_FOUND_ERROR,
                    DATA: null,
                })),
            )
                .into_response(),
        },
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
