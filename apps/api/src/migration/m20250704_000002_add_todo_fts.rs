use sea_orm_migration::prelude::*;

const UP: &str = r#"
ALTER TABLE todos ADD COLUMN title_tsv tsvector
    GENERATED ALWAYS AS (to_tsvector('english', coalesce(title, ''))) STORED;

CREATE INDEX IF NOT EXISTS idx_todos_fts
    ON todos USING GIN (title_tsv) WHERE deleted_at IS NULL;
"#;

const DOWN: &str = r#"
DROP INDEX IF EXISTS idx_todos_fts;
ALTER TABLE todos DROP COLUMN title_tsv;
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
