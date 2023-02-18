use crate::presentation::controller::hello_controller::{root};
use crate::presentation::controller::user_controller::{all_users};
use axum::{routing::get, Router};


pub fn create_app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", get(all_users))
}