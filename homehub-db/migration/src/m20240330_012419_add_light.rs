use sea_orm_migration::{
    prelude::extension::postgres::Type, sea_orm::Iterable,
};
use sea_orm_migration::{prelude::*, sea_orm::EnumIter};

use crate::m20240317_190601_create_base_schema::{GenerateUuid, Room};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(LightStateEnum)
                    .values(LightStateVariants::iter())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Light::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Light::Id)
                            .uuid()
                            .not_null()
                            .default(SimpleExpr::FunctionCall(Func::cust(
                                GenerateUuid,
                            )))
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Light::Name).string().not_null())
                    .col(
                        ColumnDef::new(Light::State)
                            .enumeration(
                                LightStateEnum,
                                LightStateVariants::iter(),
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
                    .col(ColumnDef::new(RoomLight::RoomId).uuid().not_null())
                    .col(ColumnDef::new(RoomLight::LightId).uuid().not_null())
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
#[derive(DeriveIden)]
struct LightStateEnum;

#[derive(DeriveIden, EnumIter)]
pub enum LightStateVariants {
    Off,
    On,
}

#[derive(DeriveIden)]
enum RoomLight {
    Table,
    RoomId,
    LightId,
}
