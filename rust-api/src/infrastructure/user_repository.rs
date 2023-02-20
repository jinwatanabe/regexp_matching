use diesel::{Insertable, RunQueryDsl};

use crate::{schema::users, domain::entity::user::User};
use crate::infrastructure::utils::establish_connection;
use crate::schema::users::dsl::*;

pub fn find_all_users() -> Vec<User> {
	let connection = &mut establish_connection();
	users.load::<User>(connection).expect("Error loading users")
}