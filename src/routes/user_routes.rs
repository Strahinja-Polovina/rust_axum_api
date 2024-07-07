use crate::controllers::user_controller::{
    create_user, delete_user, get_user, get_users, update_user,
};
use crate::middleware::auth_middleware::auth_middleware;
use crate::middleware::permission_middleware::permission_middleware;
use axum::{middleware, routing::get, Router};

pub fn user_routes() -> Router {
    Router::new()
        .route(
            "/users",
            get(get_users)
                .post(create_user)
                .layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/users/:user_id",
            get(get_user)
                .put(update_user)
                .delete(delete_user)
                .layer(middleware::from_fn(permission_middleware))
                .layer(middleware::from_fn(auth_middleware)),
        )
}
