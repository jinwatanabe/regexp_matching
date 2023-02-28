use anyhow::Ok;

use crate::{domain::{repository::user_repository::UserRepository, entity::user::User}, infrastructure::models::user::NewUser};

pub struct UserUseCase<T: UserRepository> {
		user_repository: T,
}

impl<T: UserRepository> UserUseCase<T> {
		pub fn new(user_repository: T) -> Self {
				UserUseCase { user_repository }
		}

		pub async fn all(&self) -> anyhow::Result<Vec<User>> {
				self.user_repository.all()
		}

		pub async fn create(&self, payload: User) -> anyhow::Result<User> {

			let new_user = NewUser{
				name: &payload.name,
				email: &payload.email,
				password: &payload.password,
			};

			let result = self.user_repository.create(new_user);
			Ok(result.unwrap())
		}
}