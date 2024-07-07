use crate::controllers::user_controller::{
    create_user, delete_user, get_user, get_users, update_user,
};
use crate::middleware::auth_middleware::middleware;
use axum::{middleware, routing::get, Router};

pub fn user_routes() -> Router {
    Router::new()
        .route("/users", get(get_users).post(create_user))
        .route(
            "/users/:user_id",
            get(get_user).put(update_user).delete(delete_user),
        )
        .layer(middleware::from_fn(middleware))
}
