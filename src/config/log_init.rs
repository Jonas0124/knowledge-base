use std::env;
use dotenvy::dotenv;
use serde_json::json;
use tracing::info;
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn req_log<T>(data: T)
where T: serde::Serialize
{
    info!("统一入参参数打印:{}", json!(data))
}

pub async fn init_logging() {
    dotenv().ok(); // 加载 .env 文件中的环境变量

    let log_dir = env::var("LOG_PATH")
        .expect("DATABASE_URL must be set in .env file");

    // 初始化日志系统 --------------------------------
    std::fs::create_dir_all(&log_dir).unwrap();

    // 控制台输出（必须保留守卫）
    let (stdout_writer, _guard) = non_blocking(std::io::stdout());

    // 文件输出
    let file_appender = rolling::daily(log_dir, "app.log");

    // 配置日志订阅器
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env()
            .add_directive("info".parse().unwrap())
        )
        // 文件层
        .with(
            Layer::new()
                .with_writer(file_appender)
                .with_ansi(false)
                .json() // 文件用JSON格式
                .with_span_list(false)
        )
        .init();
}