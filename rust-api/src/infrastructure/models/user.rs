use diesel::{Insertable, Queryable, AsChangeset};
use serde::{Serialize, Deserialize};
use validator::Validate;
use crate::schema::users;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
		pub name: &'a str,
		pub email: &'a str,
		pub password: &'a str,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Validate, Queryable, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
	#[validate(length(min = 1, message="名前は1文字以上で入力してください"))]
	pub name: String,
	#[validate(email(message="メールアドレスの形式が正しくありません"))]
	pub email: String,
	#[validate(length(min = 8, message="パスワードは8文字以上で入力してください"))]
	pub password: String,
}
