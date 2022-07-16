use actix_web::middleware::Logger;
use env_logger;
use std::env;

pub fn logger_middleware() -> Logger {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::try_init();
    Logger::new("%U %s in %T")
}
