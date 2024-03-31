use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult,
)]
pub struct LightState {
    pub on: bool,
    pub colour: Option<[u8; 3]>,
}
