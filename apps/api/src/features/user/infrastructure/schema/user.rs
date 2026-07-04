use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "Text", unique)]
    pub email: String,
    pub email_verified: bool,
    #[sea_orm(column_type = "Text", nullable)]
    pub name: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub image: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

/// No has_many relations — children (todos/accounts/sessions) declare belongs_to
/// pointing here. Adding has_many would create backward feature deps. YAGNI.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
