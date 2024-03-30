pub use sea_orm_migration::prelude::*;

mod m20240317_190601_create_base_schema;
mod m20240330_012419_add_light;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240317_190601_create_base_schema::Migration),
            Box::new(m20240330_012419_add_light::Migration),
        ]
    }
}
