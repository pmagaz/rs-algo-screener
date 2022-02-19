use mongodb::{bson::doc, options::ClientOptions, Client};
use std::env;

use crate::error::CustomError;

pub async fn connect() -> Result<Client, CustomError> {
    let client_options = ClientOptions::parse(&env::var("BACKEND_MONGO_DB_URI").unwrap())
        .await
        .unwrap();

    let client = Client::with_options(client_options).unwrap();
    client
        .database("db_users")
        .run_command(doc! {"ping": 1}, None)
        .await
        .unwrap();
    Ok(client)
}
