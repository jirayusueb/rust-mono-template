pub use sea_orm_migration::prelude::*;

mod m20250701_000001_initial;
mod m20250701_000002_auth;
mod m20250701_000004_add_todo_indexes;
mod m20250704_000001_soft_delete_and_indexes;

pub use m20250701_000001_initial::Migration as InitialMigration;
pub use m20250701_000002_auth::Migration as AuthMigration;
pub use m20250701_000004_add_todo_indexes::Migration as AddTodoIndexesMigration;
pub use m20250704_000001_soft_delete_and_indexes::Migration as SoftDeleteAndIndexesMigration;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(InitialMigration),
            Box::new(AuthMigration),
            Box::new(AddTodoIndexesMigration),
            Box::new(SoftDeleteAndIndexesMigration),
        ]
    }
}
