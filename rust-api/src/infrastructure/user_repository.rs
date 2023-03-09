use anyhow::Ok;

use super::{utils::MysqlPool, models::user::{NewUser}};
use crate::{domain::{repository::user_repository::UserRepository, entity::user::User}};
use crate::schema::users::dsl::*;
use diesel::{RunQueryDsl, QueryDsl};
use crate::infrastructure::models::user::UpdateUser;

#[derive(Debug, Clone)]
pub struct UserRepositoryForDb {
	pool: MysqlPool,
}

impl UserRepositoryForDb {
	pub fn new(pool: MysqlPool) -> Self {
		UserRepositoryForDb { pool }
	}
}

impl UserRepository for UserRepositoryForDb {
	fn all(&self) -> anyhow::Result<Vec<User>>{
		let connection = &mut self.pool.get().unwrap();
		let user_vec = users.load::<User>(connection).expect("Error loading users");
		Ok(user_vec)
	}

	fn create(&self, user: NewUser) -> anyhow::Result<User> {
			let connection = &mut self.pool.get().unwrap();
			let result = diesel::insert_into(users)
				.values(&user)
				.execute(connection);
			assert!(result.is_ok());
			
			let user_vec = users.load::<User>(connection).expect("Error loading users");
			Ok(user_vec.last().unwrap().clone())
	}

	fn update(&self, user_id: i32, user: UpdateUser) -> anyhow::Result<User> {
		let connection = &mut self.pool.get().unwrap();
		let result = diesel::update(users.find(user_id))
			.set(&user)
			.execute(connection);
		assert!(result.is_ok());

		let user = users.find(user_id).first::<User>(connection).unwrap();
		Ok(user)
		
	}
}