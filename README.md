# rust-mono-template

An opinionated fullstack monorepo template: **Rust (Axum) API** + **Waku (React) web app**, managed together with a single workspace root.

The API is a vertical-slice / clean-architecture reference: domain → application (CQRS, ports/adapters) → infrastructure (SeaORM + Postgres). Auth is session-cookie based with opaque tokens and Argon2 password hashing — modeled on better-auth's data shape, not the library.

## Layout

```
apps/
  api/                 Rust API (Axum + SeaORM + Postgres)
    src/
      features/<feature>/
        domain/             entities, value objects, domain errors
        application/        commands, queries, ports (traits), DTOs
        infrastructure/     SeaORM adapters, mappers
        presentation/http/  axum routes, extractors, mappers
      shared/               kernel (AppError, UserId), cross-cutting utils
    migrations/             sql migrations
    docs/adr/               architecture decision records
  web/                Waku React app
Cargo.toml           rust workspace root
package.json         bun workspace root (web app + lint/format tools)
```

## Stack

**API**

- Axum 0.8, Tokio, Tower (CORS, trace, governor rate-limit)
- SeaORM 1 + Postgres 18, sqlx runtime
- Argon2 password hashing, opaque session tokens
- `validator`, `thiserror`, `tracing`, `chrono`, `uuid` (v7 for sortable IDs)
- rstest + mockall for tests

**Web**

- Waku (React, RSC-ready)
- oxlint / oxfmt

**Tooling**

- `cargo` for the Rust workspace
- `bun` for the JS workspace
- `concurrently` to run both in dev

## Getting started

```sh
# 1. Postgres (local dev creds are in docker-compose.yml)
docker compose up -d

# 2. Copy env
cp .env.example .env

# 3. Run migrations + both apps
cargo run -p api -- migrate     # if a migrate subcommand exists, else use sea-orm-cli
bun install
bun dev                          # web + api concurrently
```

API default port: `3001`. Web dev server comes from Waku defaults.

## Configuration

| Var | Example | Notes |
|-----|---------|-------|
| `DATABASE_URL` | `postgres://app:dev@localhost:5432/app` | Postgres DSN |
| `DATABASE_MAX_CONNECTIONS` | `50` | SeaORM pool |
| `DATABASE_ACQUIRE_TIMEOUT_SECS` | `5` | acquire timeout |
| `PORT` | `3001` | API port |
| `APP_ENV` | `development` | |

Copy `.env.example` → `.env` and edit. `.env` is gitignored; `.env.example` is tracked.

## Architecture notes

- **Vertical slice by feature.** Each `features/<feature>` owns its domain, application, infrastructure, and presentation layers. Cross-feature imports go through published module roots, not deep paths.
- **CQRS.** Writes = `application::commands::*Handler`, reads = `application::queries::*Handler`. Each command/query has explicit `*Input` / `*Output` DTOs.
- **Ports & adapters.** Application code depends on traits in `application::ports`; SeaORM implementations live in `infrastructure::adapters` and are wired in `bootstrap`.
- **Opaque session tokens.** Sessions are random UUIDs stored in DB and read from the `session` cookie — no JWT. `GET /auth/session` is public: returns `{ user, session }` when authenticated, `null` otherwise.
- **Errors.** A single `AppError` kernel type maps domain/application errors to HTTP responses.

See `apps/api/docs/adr/` for the decisions behind these choices.

## License

MIT — use it as a starting point, no attribution required.
