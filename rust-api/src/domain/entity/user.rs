use diesel::{Queryable, Insertable, sql_types::Datetime};
use serde::{Serialize, Deserialize};
use crate::schema::users;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Queryable)]
#[derive(Insertable)]
#[table_name = "users"]
pub struct User {
		pub id: i32,
		pub name: String,
		pub email: String,
		pub password: String,
}