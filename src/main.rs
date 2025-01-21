use crate::config::log_init::init_logging;

mod router;
mod handler;
mod service;
mod models;
mod dao;
mod define;
mod middleware;
mod schema;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logging().await;
    router::run_server().await
}
