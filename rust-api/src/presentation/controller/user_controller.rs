use axum::{response::IntoResponse, Json};
use hyper::StatusCode;

use crate::{infrastructure::{user_repository::UserRepositoryForDb, utils::establish_connection}, usecase::{user_use_case::UserUseCase}};

pub async fn all_users() -> impl IntoResponse {
	let pool = establish_connection();
	let usecase = UserUseCase::new(UserRepositoryForDb::new(pool));
	let users = usecase.all().await.unwrap();
	(StatusCode::OK, Json(users))
}

pub async fn create_user(
	Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
	
	let usecase = UserUseCase::new(UserRepositoryForDb::new(establish_connection()));
	let result = usecase.create(payload).await.unwrap();
	(StatusCode::CREATED, Json(result))
}


#[derive(serde::Deserialize)]
pub struct CreateUser {
	pub name: String,
	pub email: String,
	pub password: String,
}

#[cfg(test)]
mod test {
	use crate::{infrastructure::{router::create_app, models::user::NewUser}, schema::users, domain::entity::user::User};

use super::*;
	use axum::{
		body::Body,
		http::{Request},
	};
	use diesel::RunQueryDsl;
use hyper::{header::{self}, Method};
use tower::ServiceExt;

	async fn setup() {
		let connection = establish_connection();
		let user = NewUser {
			name: "test",
			email: "test@example.com",
			password: "password",
		};
		let _ = diesel::insert_into(users::table)
			.values(&user)
			.execute(&mut connection.get().unwrap());
	}

	async fn teardown() {
		print!("teardown");
		let pool = establish_connection();
		let result = diesel::delete(users::table).execute(&mut pool.get().unwrap());
		assert!(result.is_ok());
	}

	#[tokio::test]
	async fn should_return_users() {

		setup().await;

		let req = Request::builder().uri("/users").body(Body::empty()).unwrap();
		let res = create_app().oneshot(req).await.unwrap();
		let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
		let body: String = String::from_utf8(bytes.to_vec()).unwrap();
		let users: Vec<User> = serde_json::from_str(&body).unwrap();

		let result = std::panic::catch_unwind(|| {
			assert_eq!(
				users[0],
				User {
					id: users[0].id,
					name: "test".to_string(),
					email: "test@example.com".to_string(),
					password: "password".to_string(),
				},
			);
		});

		teardown().await;

		if let Err(err) = result {
        std::panic::resume_unwind(err);
    };
	}

	#[tokio::test]
	async fn should_create_user() {
		let req = Request::builder()
			.uri("/users")
			.method(Method::POST)
			.header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
			.body(Body::from(
				r#"
				{
					"name": "test2",
					"email": "test2@example.com",
					"password": "password2"
				}
				"#,
			))
			.unwrap();


		let res = create_app().oneshot(req).await.unwrap();
		let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
		let body: String = String::from_utf8(bytes.to_vec()).unwrap();
		let user: User = serde_json::from_str(&body).expect("Error parsing json");

		assert_eq!(
			user,
			User {
				id: user.id,
				name: "test2".to_string(),
				email: "test2@example.com".to_string(),
				password: "password2".to_string(),
			}
		);
		teardown().await;
	}
}