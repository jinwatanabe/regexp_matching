use crate::presentation::controller::hello_controller::{root};
use crate::presentation::controller::user_controller::{all_users, create_user, update_user};
use axum::routing::patch;
use axum::{routing::get, Router};


pub fn create_app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", get(all_users).post(create_user))
        .route("/users/:id", patch(update_user))
}