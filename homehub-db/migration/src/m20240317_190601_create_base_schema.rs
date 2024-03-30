use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let user_result = manager
            .create_table(
                Table::create()
                    .table(AppUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AppUser::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AppUser::Name).string().not_null())
                    .col(ColumnDef::new(AppUser::Email).string().not_null())
                    .col(
                        ColumnDef::new(AppUser::PasswordHash)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AppUser::CreatedAt).timestamp().default(
                            SimpleExpr::Keyword(Keyword::CurrentTimestamp),
                        ),
                    )
                    .col(
                        ColumnDef::new(AppUser::UpdatedAt).timestamp().default(
                            SimpleExpr::Keyword(Keyword::CurrentTimestamp),
                        ),
                    )
                    .col(
                        ColumnDef::new(AppUser::Locale)
                            .string()
                            .default("en-GB"),
                    )
                    .to_owned(),
            )
            .await;

        let location_result = manager
            .create_table(
                Table::create()
                    .table(Location::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Location::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Location::Name).string().not_null())
                    .col(
                        ColumnDef::new(Location::CreatedAt)
                            .timestamp()
                            .default(SimpleExpr::Keyword(
                                Keyword::CurrentTimestamp,
                            )),
                    )
                    .col(
                        ColumnDef::new(Location::UpdatedAt)
                            .timestamp()
                            .default(SimpleExpr::Keyword(
                                Keyword::CurrentTimestamp,
                            )),
                    )
                    .to_owned(),
            )
            .await;

        let room_result = manager
            .create_table(
                Table::create()
                    .table(Room::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Room::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Room::Name).string().not_null())
                    .col(ColumnDef::new(Room::CreatedAt).timestamp().default(
                        SimpleExpr::Keyword(Keyword::CurrentTimestamp),
                    ))
                    .col(ColumnDef::new(Room::UpdatedAt).timestamp().default(
                        SimpleExpr::Keyword(Keyword::CurrentTimestamp),
                    ))
                    .col(ColumnDef::new(Room::LocationId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_room_location")
                            .from(Room::Table, Room::LocationId)
                            .to(Location::Table, Location::Id),
                    )
                    .to_owned(),
            )
            .await;

        user_result.and(room_result.and(location_result))
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AppUser::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Room::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Location::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AppUser {
    Table,
    Id,
    Name,
    Email,
    PasswordHash,
    CreatedAt,
    UpdatedAt,
    Locale,
}

#[derive(DeriveIden)]
enum Location {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Room {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
    LocationId,
}
