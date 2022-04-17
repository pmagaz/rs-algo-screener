use super::db::{Client, Db};

#[derive()]
pub struct AppState {
    pub app_name: String,
    // pub db: Client,
    // pub mem_name: String,
    pub db_mem: Db,
    pub db_hdd: Db,
}
