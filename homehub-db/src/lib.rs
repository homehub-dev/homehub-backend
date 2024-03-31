mod entities;
pub mod extra_models;
pub mod queries;
pub use entities::prelude::*;
pub use entities::*;
use sea_orm::Database;
pub use sea_orm::DatabaseConnection;

pub async fn get_database(
    database_url: &str,
) -> anyhow::Result<DatabaseConnection> {
    println!("Connecting to database: {}", database_url);
    let database = Database::connect(database_url).await?;
    Ok(database)
}
