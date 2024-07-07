pub mod config {
    pub mod db;
}

pub mod messages {
    pub mod config_constants;
    pub mod controller_constants;
    pub mod global_constants;
    pub mod middleware_constants;
    pub mod repositories_constants;
    pub mod services_constants;
}

pub mod models {
    pub mod user_model;
}

pub mod repositories {
    pub mod user_repository;
}

pub mod controllers {
    pub mod auth_controller;
    pub mod user_controller;
}

pub mod schema;

pub mod services {
    pub mod auth_service;
    pub mod password_service;
    pub mod token_service;
}
pub mod helpers {
    pub mod response_structs;
}

pub mod middleware {
    pub mod auth_middleware;
}

pub mod routes {
    pub mod auth_routes;
    pub mod user_routes;
}
