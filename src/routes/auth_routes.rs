use crate::controllers::auth_controller::login;
use axum::routing::post;
use axum::Router;

pub fn auth_routes() -> Router {
    Router::new().nest("/auth", Router::new().route("/login", post(login)))
}
