use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use std::env;
use once_cell::sync::Lazy;

// 连接池的类型
pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

// 全局静态连接池
pub static DB_POOL: Lazy<DbPool> = Lazy::new(|| {
    dotenv().ok(); // 加载 .env 文件中的环境变量

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder()
        .max_size(10)
        .min_idle(Some(3)) // 最大连接数
        .build(manager)
        .expect("Failed to create pool.")
});

// 在需要的地方使用全局的 DB_POOL
pub fn db_connection() -> DbPool {
    DB_POOL.clone()
}