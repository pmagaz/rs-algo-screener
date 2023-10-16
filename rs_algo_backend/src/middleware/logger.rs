use actix_web::middleware::Logger;
use env_logger::{self, Builder, WriteStyle};
use log::LevelFilter;
use std::env;

pub fn logger_middleware() -> Logger {
    //env::set_var("RUST_LOG", "actix_web=info");
    // env::set_var("RUST_LOG", "error");

    // env_logger::try_init();
    // Logger::new("%U %s in %T")
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::builder().try_init();

    Logger::default()
}
