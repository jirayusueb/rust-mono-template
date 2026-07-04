use std::time::Duration;

use anyhow::{Context, Result};

/// DB pool sizing + timeouts. Sensible defaults; override via env in production.
/// ponytail: only the knobs someone will actually tune — max_conns (load) and
/// acquire_timeout (fail-fast under pool exhaustion). The rest stay at sqlx defaults.
#[derive(Debug, Clone)]
pub struct DbPoolConfig {
    pub max_connections: u32,
    pub acquire_timeout: Duration,
}

impl DbPoolConfig {
    pub fn from_env() -> Self {
        let max_connections = std::env::var("DATABASE_MAX_CONNECTIONS")
            .ok()
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(50);
        let acquire_timeout = std::env::var("DATABASE_ACQUIRE_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .map(Duration::from_secs)
            .unwrap_or_else(|| Duration::from_secs(5));
        Self {
            max_connections,
            acquire_timeout,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub is_dev: bool,
    pub db_pool: DbPoolConfig,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://app:dev@localhost:5432/app".into());

        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "3001".into())
            .parse::<u16>()
            .context("PORT must be a valid port number (0–65535)")?;

        let app_env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".into());

        Ok(Self {
            database_url,
            port,
            is_dev: app_env == "development",
            db_pool: DbPoolConfig::from_env(),
        })
    }
}
