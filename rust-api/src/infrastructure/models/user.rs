use diesel::Insertable;
use crate::schema::users;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
		pub name: &'a str,
		pub email: &'a str,
		pub password: &'a str,
}