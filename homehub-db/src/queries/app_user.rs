use crate::{
    app_user::{ActiveModel, Column, Model},
    AppUser as Entity,
};
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::{prelude::Uuid, ActiveValue, DatabaseConnection};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FilteredAppUserModel {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub locale: Option<String>,
}

impl From<Model> for FilteredAppUserModel {
    fn from(val: Model) -> Self {
        FilteredAppUserModel {
            id: val.id,
            name: val.name.clone(),
            email: val.email.clone(),
            locale: val.locale.clone(),
        }
    }
}
pub async fn create_user(
    name: &str,
    email: &str,
    password_hash: &str,
    locale: Option<&str>,
    db: &DatabaseConnection,
) -> anyhow::Result<Model> {
    let user = ActiveModel {
        name: ActiveValue::Set(name.to_owned()),
        email: ActiveValue::Set(email.to_owned()),
        password_hash: ActiveValue::Set(password_hash.to_owned()),
        locale: ActiveValue::Set(locale.map(|s| s.to_owned())),
        ..Default::default()
    };

    Ok(user.insert(db).await?)
}

pub async fn find_user_by_email(
    email: &str,
    db: &DatabaseConnection,
) -> anyhow::Result<Option<Model>> {
    let user = Entity::find()
        .filter(Column::Email.eq(email))
        .one(db)
        .await?;
    Ok(user)
}

pub async fn find_by_id(
    id: Uuid,
    db: &DatabaseConnection,
) -> anyhow::Result<Option<Model>> {
    let user = Entity::find_by_id(id).one(db).await?;
    Ok(user)
}
