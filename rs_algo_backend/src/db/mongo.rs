use mongodb::{bson::doc, options::ClientOptions, Client};
use std::env;

use crate::error::CustomError;

pub async fn connect() -> Result<Client, CustomError> {
    let db_uri = &env::var("BACKEND_MONGO_DB_URI").expect("BACKEND_MONGO_DB_URI not found");
    let db_name = &env::var("BACKEND_DATABASE").expect("BACKEND_DATABASE not found");

    let client_options = ClientOptions::parse(db_uri).await.unwrap();

    let client = Client::with_options(client_options).unwrap();
    client
        .database(db_name)
        .run_command(doc! {"ping": 1}, None)
        .await
        .unwrap();

    println!("[Server] Connecting to {} database", db_name,);
    Ok(client)
}
