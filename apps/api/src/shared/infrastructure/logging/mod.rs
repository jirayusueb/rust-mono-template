use tracing_subscriber::EnvFilter;

/// Initialize structured logging.
///
/// Follows the clean-architecture guide's "structured logging" requirement:
/// JSON in production (machine-parseable for log aggregation), pretty + color
/// in development. Level filter defaults to `info`, override via `RUST_LOG`.
///
/// `tracing` is the swappable logger abstraction (subscriber = backend);
/// there is no `ILogger` trait — that pattern is a TS workaround, not needed
/// in Rust where tracing macros + subscriber layers cover it.
pub fn init() {
    let is_dev = std::env::var("APP_ENV")
        .map(|v| v == "development")
        .unwrap_or(true);

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let subscriber = tracing_subscriber::fmt().with_env_filter(filter);

    if is_dev {
        subscriber.pretty().try_init().ok();
    } else {
        subscriber.json().try_init().ok();
    }
}
