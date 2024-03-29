use homehub_db::DatabaseConnection;

pub struct AppState {
    pub db: DatabaseConnection,
    pub config: homehub_core::config::Config,
}
