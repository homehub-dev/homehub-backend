use sea_orm_migration::sea_orm::Iterable;
use sea_orm_migration::{prelude::*, sea_orm::EnumIter};

use crate::m20240317_190601_create_base_schema::Room;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Light::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Light::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Light::Name).string().not_null())
                    .col(
                        ColumnDef::new(Light::State)
                            .enumeration(
                                Alias::new("light_state"),
                                LightState::iter(),
                            )
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RoomLight::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(RoomLight::RoomId).integer().not_null())
                    .col(
                        ColumnDef::new(RoomLight::LightId).integer().not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(RoomLight::RoomId)
                            .col(RoomLight::LightId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("room_light_room_id_fk")
                            .from(RoomLight::Table, RoomLight::RoomId)
                            .to(Room::Table, Room::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("room_light_light_id_fk")
                            .from(RoomLight::Table, RoomLight::LightId)
                            .to(Light::Table, Light::Id),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RoomLight::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Light::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Light {
    Table,
    Id,
    Name,
    State,
}

#[derive(Iden, EnumIter)]
pub enum LightState {
    Off,
    On,
}

#[derive(DeriveIden)]
enum RoomLight {
    Table,
    RoomId,
    LightId,
}
