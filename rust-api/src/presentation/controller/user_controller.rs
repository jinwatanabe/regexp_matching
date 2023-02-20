use std::vec;

use axum::{response::IntoResponse, Json};
use hyper::StatusCode;

use crate::domain::entity::user::User;
use crate::infrastructure::user_repository::find_all_users;

pub async fn all_users() -> impl IntoResponse {
	let users = find_all_users();
	(StatusCode::OK, Json(users))
}

#[cfg(test)]
mod test {
	use crate::infrastructure::router::create_app;

use super::*;
	use axum::{
		body::Body,
		http::{Request, StatusCode},
	};
	use tower::ServiceExt;

	#[tokio::test]
	async fn should_return_users() {
		let req = Request::builder().uri("/users").body(Body::empty()).unwrap();
		let res = create_app().oneshot(req).await.unwrap();
		assert_eq!(res.status(), StatusCode::OK);

		let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
		let body: String = String::from_utf8(bytes.to_vec()).unwrap();
		let users: Vec<User> = serde_json::from_str(&body).expect("cannot conver User instance.");
		assert_eq!(
			users,
			vec![
			]
		)
	}
}