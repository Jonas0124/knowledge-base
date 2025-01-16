use r2d2_redis::{r2d2, RedisConnectionManager};
use std::env;
use once_cell::sync::Lazy;
use r2d2_redis::r2d2::Pool;

pub type RedisPool = Pool<RedisConnectionManager>;

// 静态全局 Redis 连接池
pub static REDIS_POOL: Lazy<RedisPool> = Lazy::new(|| {
    // 获取 Redis 连接 URL 或者从环境变量获取
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let manager = RedisConnectionManager::new(redis_url).expect("Invalid Redis URL");

    // 配置连接池的最大连接数和核心连接数
    Pool::builder()
        .max_size(10)      // 最大连接数
        .min_idle(Some(2))       // 核心连接数（空闲连接数）
        .build(manager)
        .expect("Failed to create Redis pool")
});

// 获取 Redis 连接
pub async fn get_redis_connection() -> redis::RedisResult<r2d2::PooledConnection<RedisConnectionManager>> {
    let connection = REDIS_POOL.get().expect("Failed to get Redis connection from pool");
    Ok(connection)
}