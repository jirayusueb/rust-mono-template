use sea_orm_migration::prelude::*;

const UP: &str = r#"
ALTER TABLE todos ADD COLUMN deleted_at TIMESTAMPTZ;

DROP INDEX IF EXISTS idx_todos_user_created_at;
CREATE INDEX IF NOT EXISTS idx_todos_user_created
    ON todos(user_id, created_at) WHERE deleted_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_sessions_expires
    ON sessions(expires_at);
"#;

const DOWN: &str = r#"
DROP INDEX IF EXISTS idx_sessions_expires;
DROP INDEX IF EXISTS idx_todos_user_created;
CREATE INDEX IF NOT EXISTS idx_todos_user_created_at ON todos(user_id, created_at);
ALTER TABLE todos DROP COLUMN deleted_at;
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
