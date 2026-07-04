# Centralized Configuration (env-based, fail-fast)

## Context

Following the [gist's config pattern](https://gist.github.com/RezaOwliaei/477ed74fc77aa5df2a854789538dd79d) (§6 Configuration + §11 Environment-Based Configuration). The guide calls for a centralized config in `shared/infrastructure/config/` that reads env vars once at startup and fails fast on missing values — instead of scattering `process.env` / `std::env` reads across the codebase.

**Current state:** `config/mod.rs` is a placeholder comment. The database URL and server port are **hardcoded consts** in `main.rs`:

```rust
const DATABASE_URL: &str = "sqlite://todos.db?mode=rwc";     // line 21
// ...
TcpListener::bind(([0, 0, 0, 0], 3001))                     // line 56
```

No env vars are read anywhere. No `.env` file exists. The auth feature uses DB-stored session tokens (no JWT secret needed).

## Approach

Replace the hardcoded values with a `Config` struct loaded from env vars, with sensible defaults so `cargo run` still works without any `.env`. One file, one struct, one `from_env()` constructor. No crate — `std::env::var` is the stdlib (ponytail rung 3).

**Config values:**

| Field          | Env var        | Default                      | Notes                              |
| -------------- | -------------- | ---------------------------- | ---------------------------------- |
| `database_url` | `DATABASE_URL` | `sqlite://todos.db?mode=rwc` | Keeps current dev workflow working |
| `port`         | `PORT`         | `3001`                       | Current hardcoded value            |

No `JWT_SECRET` — auth uses session tokens. No `HOST` — `0.0.0.0` is standard, add when needed.

**Fail-fast:** `from_env()` returns `anyhow::Result<Config>`. A missing required var (none currently, but the pattern supports adding them) produces a clear error. Invalid `PORT` (non-numeric / out of range) fails immediately with context.

## Files to modify

| File                                               | Change                                                     |
| -------------------------------------------------- | ---------------------------------------------------------- |
| `apps/api/src/shared/infrastructure/config/mod.rs` | Replace stub with `Config` struct + `Config::from_env()`   |
| `apps/api/src/main.rs`                             | Load `Config`, use `config.database_url` and `config.port` |

## Reuse

- `std::env::var` — stdlib, no new dependency.
- `anyhow::Context` — already in deps, already used in `main.rs`.
- The existing `DATABASE_URL` const value becomes the default — zero behavior change for `cargo run`.

## Steps

- [ ] **1.** Implement `Config` in `shared/infrastructure/config/mod.rs`:

  ```rust
  use anyhow::{Context, Result};

  #[derive(Debug, Clone)]
  pub struct Config {
      pub database_url: String,
      pub port: u16,
  }

  impl Config {
      pub fn from_env() -> Result<Self> {
          let database_url = std::env::var("DATABASE_URL")
              .unwrap_or_else(|_| "sqlite://todos.db?mode=rwc".into());

          let port = std::env::var("PORT")
              .unwrap_or_else(|_| "3001".into())
              .parse::<u16>()
              .context("PORT must be a valid port number (0–65535)")?;

          Ok(Self { database_url, port })
      }
  }
  ```

- [ ] **2.** Update `main.rs`:
  - Remove `const DATABASE_URL` (line 21).
  - First line inside `main()`: `let config = Config::from_env().context("load configuration")?;`
  - Replace `Database::connect(DATABASE_URL)` → `Database::connect(&config.database_url)`.
  - Replace the hardcoded `3001` in `TcpListener::bind(...)` → `config.port`.

## Verification

```sh
cargo build -p api     # compiles clean
cargo test -p api      # all existing tests pass
cargo run -p api       # server starts on :3001, connects to todos.db (defaults)

# Override via env:
DATABASE_URL="sqlite://test.db?mode=rwc" PORT=4000 cargo run -p api
# → "API listening on http://localhost:4000", uses test.db

# Bad port fails fast:
PORT=abc cargo run -p api    # → error: "PORT must be a valid port number"
PORT=99999 cargo run -p api  # → error: invalid value for PORT
```

## Notes / Ponytail

- **No `config` / `envy` crate.** Two env reads with one `.parse()` — `std::env` is enough. A crate adds a dependency for zero gain until the config surface grows significantly.
- **Defaults, not required.** The gist's `getRequiredEnv` throws on missing vars. In Rust, returning `Result` and failing at startup is the same fail-fast behavior — but giving `DATABASE_URL`/`PORT` defaults keeps `cargo run` working with zero setup. Add required vars (e.g., a real `DATABASE_URL` for prod) by switching `unwrap_or_else` to `?` + `.context("DATABASE_URL is required")`.
- **No `.env` file loading.** `dotenvy` or similar would auto-load `.env`, but `cargo run` with `VAR=val` prefix or `direnv` already works. Add when the team standardizes on `.env`.
- **No `Host` field.** `0.0.0.0` is baked into the bind call. If `127.0.0.1`-only binding is ever needed, add `host: String` then.
