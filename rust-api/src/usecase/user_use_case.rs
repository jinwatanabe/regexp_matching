use crate::domain::{repository::user_repository::UserRepository, entity::user::User};

pub struct UserUseCase<T: UserRepository> {
		user_repository: T,
}

impl<T: UserRepository> UserUseCase<T> {
		pub fn new(user_repository: T) -> Self {
				UserUseCase { user_repository }
		}

		pub async fn all(&self) -> anyhow::Result<Vec<User>> {
				self.user_repository.all().await
		}
}