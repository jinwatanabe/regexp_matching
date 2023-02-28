use axum::async_trait;

use crate::{domain::entity::user::User, infrastructure::models::user::NewUser};

pub trait UserRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
	fn all(&self) -> anyhow::Result<Vec<User>>;
	fn create(&self, user: NewUser) -> anyhow::Result<User>;
}