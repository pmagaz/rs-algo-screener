pub use mongodb::Client;

pub struct Db {
    pub client: Client,
    pub name: String,
}
