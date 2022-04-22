use actix_cors::Cors;
use actix_web::{get, http, web, App, HttpRequest, HttpResponse, HttpServer};

pub fn cors_middleware() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:8080")
        .allowed_origin("http://cluster.loc")
        .allowed_origin("ttp://rs-screener.ddns.net")
        .allowed_methods(vec!["GET", "PUT", "POST"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
}
