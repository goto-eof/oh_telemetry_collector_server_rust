use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "telemetry")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub request_id: i32,
    pub code: String,
    pub key: String,
    pub value: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
