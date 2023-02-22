use axum::async_trait;

use crate::domain::entity::user::User;

#[async_trait]
pub trait UserRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
	async fn all(&self) -> anyhow::Result<Vec<User>>;
}