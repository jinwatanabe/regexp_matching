use axum::{response::IntoResponse, Json, async_trait, extract::{FromRequest, RequestParts}, BoxError, body::HttpBody};
use hyper::StatusCode;
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use crate::{infrastructure::{user_repository::UserRepositoryForDb, utils::establish_connection}, usecase::{user_use_case::UserUseCase}};
use validator::Validate;

#[async_trait]
impl<T,B> FromRequest<B> for ValidatedJson<T>
where
	T: DeserializeOwned + Validate,
	B: http_body::Body + Send,
	B::Data: Send,
	B::Error: Into<BoxError>,
{
	type Rejection = (StatusCode, String);

	async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
		let Json(value) = Json::<T>::from_request(req).await.map_err(|rejection| {
			let message = format!("Json parse error: [{}]", rejection);
			(StatusCode::BAD_REQUEST, message)
		})?;
		value.validate().map_err(|rejection| {
			let message = format!("Validation error: [{}]", rejection);
			(StatusCode::BAD_REQUEST, message)
		})?;
		Ok(ValidatedJson(value))
	}
}

#[derive(Debug)]
pub struct ValidatedJson<T>(T);

pub async fn all_users() -> impl IntoResponse {
	let pool = establish_connection();
	let usecase = UserUseCase::new(UserRepositoryForDb::new(pool));
	let users = usecase.all().await.unwrap();
	(StatusCode::OK, Json(users))
}

pub async fn create_user(
	ValidatedJson(payload): ValidatedJson<CreateUser>,
) -> impl IntoResponse {
	
	let usecase = UserUseCase::new(UserRepositoryForDb::new(establish_connection()));
	let result = usecase.create(payload).await.unwrap();
	(StatusCode::CREATED, Json(result))
}


#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Validate)]
pub struct CreateUser {
	#[validate(length(min = 1, message="名前は1文字以上で入力してください"))]
	pub name: String,
	#[validate(email(message="メールアドレスの形式が正しくありません"))]
	pub email: String,
	#[validate(length(min = 8, message="パスワードは8文字以上で入力してください"))]
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