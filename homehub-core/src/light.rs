pub use homehub_db::light::LightState;
use homehub_db::DatabaseConnection;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LightDto {
    pub id: uuid::Uuid,
    pub name: String,
    pub state: LightState,
    pub room: Option<RoomDto>,
}

#[derive(Debug, Serialize)]
pub struct RoomDto {
    pub id: uuid::Uuid,
    pub name: String,
}

impl From<(homehub_db::light::Model, Option<homehub_db::room::Model>)>
    for LightDto
{
    fn from(
        value: (homehub_db::light::Model, Option<homehub_db::room::Model>),
    ) -> Self {
        LightDto {
            id: value.0.id,
            name: value.0.name,
            state: value.0.state,
            room: value.1.map(|room| RoomDto {
                id: room.id,
                name: room.name,
            }),
        }
    }
}

pub async fn create_light(
    name: &str,
    room_id: Option<uuid::Uuid>,
    db: &DatabaseConnection,
) -> anyhow::Result<LightDto> {
    let light: LightDto =
        homehub_db::queries::light::create_light(name, room_id, db)
            .await?
            .into();

    Ok(light)
}

pub async fn get_light(
    id: &uuid::Uuid,
    db: &DatabaseConnection,
) -> anyhow::Result<LightDto> {
    let light: LightDto =
        homehub_db::queries::light::get_light(id, db).await?.into();
    Ok(light)
}

pub async fn set_light_state(
    id: &uuid::Uuid,
    state: LightState,
    db: &DatabaseConnection,
) -> anyhow::Result<LightDto> {
    let light: LightDto =
        homehub_db::queries::light::set_light_state(id, state, db)
            .await?
            .into();
    // Trigger listeners here
    Ok(light)
}

pub async fn update_light(
    id: &uuid::Uuid,
    name: Option<&str>,
    room_id: Option<Option<uuid::Uuid>>,
    db: &DatabaseConnection,
) -> anyhow::Result<LightDto> {
    let light: LightDto =
        homehub_db::queries::light::update_light(id, name, room_id, db)
            .await?
            .into();
    Ok(light)
}
