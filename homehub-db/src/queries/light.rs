use sea_orm::{prelude::Uuid, ActiveValue, DatabaseConnection};
use sea_orm::{ActiveModelTrait, QueryFilter};
use sea_orm::{ColumnTrait, EntityTrait};

use crate::extra_models::light::LightState;

type LightResult = anyhow::Result<(
    crate::entities::light::Model,
    Option<crate::entities::room::Model>,
)>;

pub async fn create_light(
    name: &str,
    room_id: Option<Uuid>,
    db: &DatabaseConnection,
) -> LightResult {
    let light = crate::entities::light::ActiveModel {
        name: ActiveValue::Set(name.to_owned()),
        state: ActiveValue::Set(LightState {
            on: false,
            colour: None,
        }),
        ..Default::default()
    };

    let light_model = light.insert(db).await?;

    if let Some(room_id) = room_id {
        let room_light = crate::entities::room_light::ActiveModel {
            room_id: ActiveValue::Set(room_id),
            light_id: ActiveValue::Set(light_model.id.to_owned()),
        };

        room_light.insert(db).await?;

        let room_model = crate::entities::room::Entity::find_by_id(room_id)
            .one(db)
            .await?;

        return Ok((light_model, room_model));
    };

    Ok((light_model, None))
}

pub async fn get_light(id: &Uuid, db: &DatabaseConnection) -> LightResult {
    let light = crate::entities::light::Entity::find_by_id(*id)
        .find_with_related(crate::entities::room::Entity)
        .all(db)
        .await?;

    if let Some(light) = light.into_iter().next() {
        return Ok((light.0, light.1.into_iter().next()));
    }
    Err(anyhow::anyhow!("No lights found"))
}

pub async fn update_light(
    id: &Uuid,
    name: Option<&str>,
    room_id: Option<Option<Uuid>>,
    db: &DatabaseConnection,
) -> LightResult {
    let (light, room) = get_light(id, db).await?;
    let mut light: crate::entities::light::ActiveModel = light.into();
    if let Some(name) = name {
        light.name = ActiveValue::Set(name.to_owned());
    }
    if let Some(Some(room_id)) = room_id {
        let existing_room_light = crate::entities::room_light::Entity::find()
            .filter(crate::entities::room_light::Column::LightId.eq(*id))
            .one(db)
            .await?;
        if let Some(existing_room_light) = existing_room_light {
            let existing_room_light: crate::entities::room_light::ActiveModel =
                existing_room_light.into();
            existing_room_light.delete(db).await?;
        }
        let room_light = crate::entities::room_light::ActiveModel {
            room_id: ActiveValue::Set(room_id),
            light_id: ActiveValue::Set(id.to_owned()),
        };

        room_light.insert(db).await?;
    }
    let light = light.update(db).await?;
    Ok((light, room))
}

pub async fn set_light_state(
    id: &Uuid,
    state: LightState,
    db: &DatabaseConnection,
) -> LightResult {
    let (light, room) = get_light(id, db).await?;
    let mut light: crate::entities::light::ActiveModel = light.into();
    light.state = ActiveValue::Set(state);
    let light = light.update(db).await?;
    Ok((light, room))
}
