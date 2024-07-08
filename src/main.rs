use axum::Router;
use axumapi::routes::auth_routes::auth_routes;
use axumapi::routes::user_routes::user_routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect(".env doesnt exist");

    let app = Router::new()
        .nest("/api", auth_routes())
        .nest("/api", user_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
