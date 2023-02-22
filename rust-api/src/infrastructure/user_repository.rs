use anyhow::Ok;
use axum::async_trait;

use super::utils::MysqlPool;
use crate::{domain::{repository::user_repository::UserRepository, entity::user::User}};
use crate::{schema::users};
use crate::schema::users::dsl::*;
use diesel::{RunQueryDsl};

#[derive(Debug, Clone)]
pub struct UserRepositoryForDb {
	pool: MysqlPool,
}

impl UserRepositoryForDb {
	pub fn new(pool: MysqlPool) -> Self {
		UserRepositoryForDb { pool }
	}
}

#[async_trait]
impl UserRepository for UserRepositoryForDb {
	async fn all(&self) -> anyhow::Result<Vec<User>>{
		let connection = &mut self.pool.get().unwrap();
		let user_vec = users.load::<User>(connection).expect("Error loading users");
		Ok(user_vec)
	}
}