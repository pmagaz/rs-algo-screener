use actix_web::{web, App, HttpResponse, HttpServer, Result};
use actix_web_httpauth::middleware::HttpAuthentication;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

fn verify_credentials(credentials: &Credentials) -> bool {
    true
}
