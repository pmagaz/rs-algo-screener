use super::db::Db;

#[derive()]
pub struct AppState {
    pub app_name: String,
    pub db_mem: Db,
    pub db_hdd: Db,
    pub db_bot: Db,
}
