use crate::controllers::user_controller::{
    create_user, delete_user, get_user, get_users, update_user,
};
use crate::middleware::auth_middleware::middleware;
use axum::{middleware, routing::get, Router};

pub fn user_routes() -> Router {
    // Apply middleware to the individual routes that need it, after path extraction
    Router::new()
        .route(
            "/users",
            get(get_users).post(create_user).layer(middleware::from_fn(middleware)),
        )
        .route(
            "/users/:user_id",
            get(get_user)
                .put(update_user)
                .delete(delete_user)
                .layer(middleware::from_fn(middleware)), // <-- Layer applied here
        )
}
