use std::vec;

use axum::{response::IntoResponse, Json};
use hyper::StatusCode;

use crate::domain::entity::user::User;

pub async fn all_users() -> impl IntoResponse {
	let users = vec![
		User {
			id: 1,
			name: "test".to_string(),
			email: "test@example.com".to_string(),
		},
		User {
			id: 2,
			name: "test2".to_string(),
			email: "test2@example.com".to_string(),
		},
	];

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
				User {
					id: 1,
					name: "test".to_string(),
					email: "test@example.com".to_string(),
				},
				User {
					id: 2,
					name: "test2".to_string(),
					email: "test2@example.com".to_string(),
				},
			]
		)
	}
}