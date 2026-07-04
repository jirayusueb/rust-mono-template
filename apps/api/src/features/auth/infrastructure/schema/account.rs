use sea_orm::entity::prelude::*;

use crate::features::user::infrastructure::schema::user;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "accounts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub account_id: String,
    #[sea_orm(column_type = "Text")]
    pub provider_id: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub access_token: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub refresh_token: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub id_token: Option<String>,
    pub access_token_expires_at: Option<DateTimeUtc>,
    pub refresh_token_expires_at: Option<DateTimeUtc>,
    #[sea_orm(column_type = "Text", nullable)]
    pub scope: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub password: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "user::Entity",
        from = "Column::UserId",
        to = "user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
