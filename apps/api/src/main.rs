use std::sync::Arc;

use anyhow::Context;
use axum::{routing::get, Json, Router};
use sea_orm::{ConnectOptions, Database};
use sea_orm_migration::MigratorTrait;
use serde::Serialize;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use api::bootstrap::AppState;
use api::features::auth::infrastructure::repositories::PostgresAuthRepository;
use api::features::auth::infrastructure::UserPortAdapter;
use api::features::auth::presentation::http::routes::auth_routes;
use api::features::todo::infrastructure::repositories::PostgresTodoRepository;
use api::features::todo::presentation::http::routes::todo_routes;
use api::features::user::infrastructure::repositories::PostgresUserRepository;
use api::migration::Migrator;
use api::shared::infrastructure::config::Config;
use api::shared::infrastructure::database::unit_of_work::SeaOrmUnitOfWork;
use api::shared::infrastructure::logging;
use api::shared::infrastructure::utils::argon2_hasher::Argon2PasswordHasher;
use api::shared::presentation::middleware::request_id;

#[derive(Serialize)]
struct Health {
    status: &'static str,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logging::init();
    dotenvy::dotenv().ok();

    let config = Config::from_env().context("load configuration")?;

    let mut db_opts = ConnectOptions::new(config.database_url.clone());
    db_opts
        .max_connections(config.db_pool.max_connections)
        .acquire_timeout(config.db_pool.acquire_timeout)
        .sqlx_logging(config.is_dev);
    let db = Database::connect(db_opts)
        .await
        .context("connect to database")?;
    Migrator::up(&db, None).await.context("run migrations")?;

    let todo_repo = Arc::new(PostgresTodoRepository::new(db.clone()));
    let user_repo = Arc::new(PostgresUserRepository::new(db.clone()));
    let auth_repo = Arc::new(PostgresAuthRepository::new(db.clone()));
    let uow = Arc::new(SeaOrmUnitOfWork::new(db));
    let password_hasher = Arc::new(Argon2PasswordHasher::new());
    let user_port = Arc::new(UserPortAdapter::new(user_repo.clone()));

    let state = AppState {
        todo_repo,
        user_repo,
        auth_repo,
        password_hasher,
        user_port,
        uow,
        is_dev: config.is_dev,
    };

    let app = Router::new()
        .route(
            "/api/health",
            get(|| async { Json(Health { status: "ok" }) }),
        )
        .nest("/api", todo_routes().merge(auth_routes()))
        .layer(TraceLayer::new_for_http())
        .layer(axum::middleware::from_fn(request_id))
        .layer(CorsLayer::very_permissive())
        .with_state(state);

    let listener =
        tokio::net::TcpListener::bind(std::net::SocketAddr::from(([0, 0, 0, 0], config.port)))
            .await
            .context("bind listener")?;
    tracing::info!("API listening on http://localhost:{}", config.port);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .context("serve")?;
    Ok(())
}
