/// Shared database connection type for all repositories.
/// SeaORM's DatabaseConnection is internally pooled and Clone.
pub type DbPool = sea_orm::DatabaseConnection;

pub mod tx;
pub mod unit_of_work;
