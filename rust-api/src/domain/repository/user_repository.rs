use crate::{domain::entity::user::User, infrastructure::models::user::{NewUser, UpdateUser,}};

pub trait UserRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
	fn all(&self) -> anyhow::Result<Vec<User>>;
	fn create(&self, user: NewUser) -> anyhow::Result<User>;
	fn update(&self, id: i32 ,user: UpdateUser) -> anyhow::Result<User>;
}