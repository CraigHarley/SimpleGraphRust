use mysql::Pool;

pub fn get_pool() -> Pool {
    mysql::Pool::new("mysql://root@localhost:3306/mysql")
        .unwrap()
}