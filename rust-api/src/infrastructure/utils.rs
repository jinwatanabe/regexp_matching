use dotenv::dotenv;
use std::env;
use diesel::{r2d2::{Pool, ConnectionManager}, mysql::MysqlConnection};

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn establish_connection() -> MysqlPool  {
    dotenv().ok();


    let database_url = get_database_url();
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::new(manager).expect("Failed to create pool")
}

fn get_database_url() -> String {
    let database_url = if cfg!(test) {
        env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set")
    } else {
        env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    };
    database_url
}