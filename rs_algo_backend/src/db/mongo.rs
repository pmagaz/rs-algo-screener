use mongodb::{bson::doc, options::ClientOptions, Client};
use std::env;

use crate::error::RsAlgoError;

pub async fn connect() -> Result<Client, RsAlgoError> {
    let db_user = &env::var("DB_USERNAME").expect("DB_USERNAME not found");
    let db_password = &env::var("DB_PASSWORD").expect("DB_PASSWORD not found");
    let db_name = &env::var("BACKEND_DATABASE").expect("BACKEND_DATABASE not found");

    let db_uri = [
        "mongodb://",
        db_user,
        ":",
        db_password,
        &env::var("BACKEND_MONGO_DB_URI").expect("BACKEND_MONGO_DB_URI not found"),
    ]
    .concat();

    let client_options = ClientOptions::parse(db_uri).await.unwrap();

    let client = Client::with_options(client_options).unwrap();
    client
        .database(db_name)
        .run_command(doc! {"ping": 1}, None)
        .await
        .unwrap();

    println!("[Server] Connecting to {} database", db_name);
    Ok(client)
}
