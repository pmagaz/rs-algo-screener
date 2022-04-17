use mongodb::{bson::doc, options::ClientOptions, Client};
use std::env;

use crate::error::RsAlgoError;

pub async fn connect(
    username: &str,
    password: &str,
    db_name: &str,
    uri: &str,
) -> Result<Client, RsAlgoError> {
    let db_uri = ["mongodb://", username, ":", password, uri].concat();
    println!("[Server] Connecting to {}...", db_name);

    let client_options = ClientOptions::parse(db_uri).await.unwrap();

    let client = Client::with_options(client_options).unwrap();
    client
        .database(db_name)
        .run_command(doc! {"ping": 1}, None)
        .await
        .unwrap();

    println!("[Server] Connected to {} ", db_name);

    Ok(client)
}
