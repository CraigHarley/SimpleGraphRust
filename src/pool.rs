use mysql::Pool;
use std::env;

pub fn get_pool() -> Pool {
    let mut builder = mysql::OptsBuilder::new();
    let _dotenv = dotenv::dotenv();

    builder
        .ip_or_hostname(Some(env::var("DB_HOST").unwrap()))
        .db_name(Some(env::var("DB_DATABASE").unwrap()))
        .user(Some(env::var("DB_USER").unwrap()))
        .pass(Some(env::var("DB_PASSWORD").unwrap()));

    mysql::Pool::new(builder).unwrap()
}
