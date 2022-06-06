use actix_cors::Cors;
use actix_web::http;
use std::env;

pub fn cors_middleware() -> Cors {
    let env = env::var("ENV").unwrap();

    let cors = match &*env {
        "production" => Cors::default(),
        "development" => Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_origin("http://cluster.loc"),
        &_ => Cors::default(),
    };

    cors.allowed_origin("http://rs-screener.ddns.net")
        .allowed_methods(vec!["GET", "PUT", "DELETE", "POST"])
        .allowed_headers(vec![
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
            http::header::CONTENT_TYPE,
        ])
        .max_age(3600)
}
