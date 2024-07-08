use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use crate::models::user_model::{CreateUserDTO, UpdateUserDTO};
use crate::{config::db::get_connection, repositories::user_repository::UserRepository};

pub async fn get_users() -> impl IntoResponse {
    match get_connection() {
        Ok(mut conn) => match UserRepository::find_all(&mut conn) {
            Ok(users) => (
                StatusCode::OK,
                Json(json!({
                    "status": StatusCode::OK.as_u16(),
                    "message": "Success fetch users",
                    "data": users,
                })),
            )
                .into_response(),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    "message": "Internal server error",
                    "data": null,
                })),
            )
                .into_response(),
        },
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

pub async fn get_user(Path(user_id): Path<i32>) -> impl IntoResponse {
    match get_connection() {
        Ok(mut conn) => match UserRepository::find_one(&mut conn, user_id) {
            Ok(user) => (
                StatusCode::OK,
                Json(json!({
                    "status": StatusCode::OK.as_u16(),
                    "message": "Success fetch user",
                    "data": user,
                })),
            )
                .into_response(),
            Err(_) => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "status": StatusCode::NOT_FOUND.as_u16(),
                    "message": "User not found",
                    "data": null,
                })),
            )
                .into_response(),
        },
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

pub async fn create_user(Json(new_user): Json<CreateUserDTO>) -> impl IntoResponse {
    match get_connection() {
        Ok(mut conn) => {
            match UserRepository::find_by_email(&mut conn, &new_user.email.to_lowercase()) {
                Ok(_) => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "status": StatusCode::BAD_REQUEST.as_u16(),
                        "message": "E-mail already exist",
                        "data": null,
                    })),
                )
                    .into_response(),
                Err(_) => match UserRepository::create_user(&mut conn, new_user) {
                    Ok(created_user) => (
                        StatusCode::CREATED,
                        Json(json!({
                            "status": StatusCode::CREATED.as_u16(),
                            "message": "User created successfully",
                            "data": created_user,
                        })),
                    )
                        .into_response(),
                    Err(_) => (
                        StatusCode::BAD_REQUEST,
                        Json(json!({
                            "status": StatusCode::BAD_REQUEST.as_u16(),
                            "message": "Bad request",
                            "data": null,
                        })),
                    )
                        .into_response(),
                },
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

pub async fn delete_user(Path(user_id): Path<i32>) -> impl IntoResponse {
    match get_connection() {
        Ok(mut conn) => {
            let user_deleted = UserRepository::find_one(&mut conn, user_id);
            match user_deleted {
                Ok(_) => match UserRepository::delete_user(&mut conn, user_id) {
                    Ok(_) => (
                        StatusCode::OK,
                        Json(json!({
                            "status": StatusCode::OK.as_u16(),
                            "message": "User deleted successfully",
                            "data": user_deleted.unwrap(),
                        })),
                    )
                        .into_response(),
                    Err(_) => (
                        StatusCode::BAD_REQUEST,
                        Json(json!({
                            "status": StatusCode::BAD_REQUEST.as_u16(),
                            "message": "Cannot delete user",
                            "data": null,
                        })),
                    )
                        .into_response(),
                },
                Err(_) => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "status": StatusCode::BAD_REQUEST.as_u16(),
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
                        "status": StatusCode::BAD_REQUEST.as_u16(),
                        "message": "User already exist",
                        "data": null,
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
                                "status": StatusCode::OK.as_u16(),
                                "message": "Update successfully",
                                "data": user,
                            })),
                        )
                            .into_response(),
                        Err(_) => (
                            StatusCode::BAD_REQUEST,
                            Json(json!({
                                "status": StatusCode::BAD_REQUEST.as_u16(),
                                "message": "Cannot update user",
                                "data": null,
                            })),
                        )
                            .into_response(),
                    }
                }
            },
            Err(_) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": StatusCode::BAD_REQUEST.as_u16(),
                    "message": "User not found",
                    "data": null,
                })),
            )
                .into_response(),
        },
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
