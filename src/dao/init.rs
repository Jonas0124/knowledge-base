use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn db_connection() -> DbPool {
    dotenv().ok(); // 加载 .env 文件中的环境变量

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager).expect("Failed to create pool.")
}

// fn main() {
//     // 创建连接池
//     let pool = establish_connection();
//
//     // 获取一个连接
//     let conn: PooledConnection<ConnectionManager<MysqlConnection>> = pool.get().expect("Failed to get connection from pool.");
//
//     // 在此可以使用 conn 执行 Diesel 的查询操作
// }