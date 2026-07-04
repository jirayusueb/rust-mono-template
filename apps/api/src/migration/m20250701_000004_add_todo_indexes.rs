use sea_orm_migration::prelude::*;

const UP: &str = r#"
CREATE INDEX IF NOT EXISTS idx_todos_user_created_at ON todos(user_id, created_at);
"#;

const DOWN: &str = r#"
DROP INDEX IF EXISTS idx_todos_user_created_at;
"#;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(UP).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(DOWN).await?;
        Ok(())
    }
}
