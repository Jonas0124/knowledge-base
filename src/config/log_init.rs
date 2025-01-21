use tracing_subscriber::FmtSubscriber;
use tracing_subscriber::EnvFilter;
pub async fn init_logging() {
    // 设置日志的过滤器，默认级别是 info
    let filter = EnvFilter::new("info");
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(filter)
        .finish();

    // 设置全局日志订阅器
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // 使用 log4rs 加载配置文件
    log4rs::init_file("log4rs.yml", Default::default())
        .expect("Failed to initialize log4rs");
}