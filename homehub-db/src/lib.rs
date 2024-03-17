mod entities;
pub use entities::*;

use sea_orm::{Database, DatabaseConnection};

pub async fn get_database() -> anyhow::Result<DatabaseConnection> {
    let database_url = std::env::var("DATABASE_URL")?;
    let database = Database::connect(&database_url).await?;
    Ok(database)
}
