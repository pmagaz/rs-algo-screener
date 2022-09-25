use mongodb::{bson::doc, options::ClientOptions, Client};


use crate::error::RsAlgoError;

pub async fn connect(
    username: &str,
    password: &str,
    db_name: &str,
    uri: &str,
) -> Result<Client, RsAlgoError> {
    let db_uri = ["mongodb://", username, ":", password, uri].concat();
    log::info!("Connecting to {}...", db_name);

    let client_options = ClientOptions::parse(db_uri).await.unwrap();

    let client = Client::with_options(client_options).unwrap();
    client
        .database(db_name)
        .run_command(doc! {"ping": 1}, None)
        .await
        .unwrap();

    log::info!("Connected to {} ", db_name);

    Ok(client)
}
