use mongodb::Client;

#[derive()]
pub struct AppState {
    pub app_name: String,
    pub db: Client,
}
