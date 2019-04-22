use mysql::Pool;

pub fn get_pool() -> Pool {
    let mut builder = mysql::OptsBuilder::new();
    builder
        .ip_or_hostname(Some("127.0.0.1"))
        .db_name(Some("sixdegrees"))
        .user(Some("root"))
        .pass(Some(""));

    mysql::Pool::new(builder).unwrap()
}
