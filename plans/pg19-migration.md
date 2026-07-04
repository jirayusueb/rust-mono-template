# Plan: PostgreSQL 19 via Docker Compose + SQLite→PG Migration

## Context

The API (`apps/api`) persists via **SeaORM with the `sqlx-sqlite` feature**. The
repository layer is already **DB-agnostic** (pure SeaORM DSL), so the migration
is: swap the driver feature, rewrite migrations to PG dialect with native column
types, rename `Sqlite*` → `Postgres*`, and add Docker Compose for PG 19 Alpine.

No production data exists (dev template) — migrations are rewritten in place.

## Approach

Three workstreams in order: **infra** (Docker + env), **driver swap** (Cargo +
config), **native types** (schema models + migrations + mappers + repos).

---

## Part 1: Docker Compose + Environment

### New files

**`docker-compose.yml`** (repo root):

```yaml
services:
  db:
    image: postgres:19-alpine
    container_name: app_db
    environment:
      POSTGRES_DB: app
      POSTGRES_USER: app
      POSTGRES_PASSWORD: dev
    ports:
      - "5432:5432"
    volumes:
      - pgdata:/var/lib/postgresql/data
volumes:
  pgdata:
```

**`.env.example`** (repo root — `.env` itself is gitignored):

```
DATABASE_URL=postgres://app:dev@localhost:5432/app
PORT=3001
APP_ENV=development
```

### Modified files

| File                                               | Change                                                           |
| -------------------------------------------------- | ---------------------------------------------------------------- |
| `apps/api/Cargo.toml`                              | `sqlx-sqlite` → `sqlx-postgres`; add `dotenvy = "0.15"`          |
| `apps/api/src/shared/infrastructure/config/mod.rs` | Default `DATABASE_URL` → `postgres://app:dev@localhost:5432/app` |
| `apps/api/src/main.rs`                             | Add `dotenvy::dotenv().ok();` before `Config::from_env()`        |

---

## Part 2: Native Column Types

Current: everything is `TEXT` / `INTEGER`. Target: `UUID` for PKs/FKs, `TIMESTAMPTZ` for timestamps, `BOOLEAN` for flags.

### Type mapping

| Column                                     | SQLite (was) | PostgreSQL (now) | Schema model Rust type                |
| ------------------------------------------ | ------------ | ---------------- | ------------------------------------- |
| `*.id` (PK)                                | `TEXT`       | `UUID`           | `Uuid`                                |
| `*.user_id` (FK)                           | `TEXT`       | `UUID`           | `Uuid`                                |
| `accounts.id`                              | `TEXT`       | `UUID`           | `Uuid`                                |
| timestamps                                 | `TEXT`       | `TIMESTAMPTZ`    | `DateTimeUtc` (no change)             |
| `email_verified`                           | `INTEGER`    | `BOOLEAN`        | `bool` (no change)                    |
| `sessions.id`, `sessions.token`            | `TEXT`       | `TEXT`           | `String` (no change — opaque strings) |
| `accounts.account_id`, `provider_id`, etc. | `TEXT`       | `TEXT`           | `String` (no change)                  |

**Net:** 6 schema-model fields change `String` → `Uuid`. Timestamps and bools already map correctly — only the migration SQL changes for those.

### Domain newtype conversion impls

`UserId`, `TodoId`, `AccountId` all wrap `uuid::Uuid` with a private field. Add bidirectional `From` impls so mappers/repos use `.into()` instead of `from_str`/`to_string`:

```rust
// In each newtype file (user_id.rs, todo_id.rs, account_id.rs):
impl From<Uuid> for UserId  { fn from(u: Uuid) -> Self { Self(u) } }
impl From<UserId> for Uuid  { fn from(id: UserId) -> Self { id.0 } }
```

### Files to modify — schema models (4 files)

| File                                             | Change                             |
| ------------------------------------------------ | ---------------------------------- |
| `features/todo/infrastructure/schema/todo.rs`    | `id`, `user_id`: `String` → `Uuid` |
| `features/user/infrastructure/schema/user.rs`    | `id`: `String` → `Uuid`            |
| `features/auth/infrastructure/schema/account.rs` | `id`, `user_id`: `String` → `Uuid` |
| `features/auth/infrastructure/schema/session.rs` | `user_id`: `String` → `Uuid`       |

### Files to modify — mappers (4 files)

| File                                                    | Change                                                    |
| ------------------------------------------------------- | --------------------------------------------------------- |
| `features/todo/infrastructure/mapper/todo_mapper.rs`    | `from_str(&row.id)` → `row.id.into()`; same for `user_id` |
| `features/user/infrastructure/mapper/user_mapper.rs`    | `from_str(&row.id)` → `row.id.into()`                     |
| `features/auth/infrastructure/mapper/account_mapper.rs` | `from_str(&row.id)` → `row.id.into()`; same for `user_id` |
| `features/auth/infrastructure/mapper/session_mapper.rs` | `from_str(&row.user_id)` → `row.user_id.into()`           |

### Files to modify — domain newtypes (3 files)

| File                                        | Change                                     |
| ------------------------------------------- | ------------------------------------------ |
| `shared/kernel/user_id.rs`                  | Add `From<Uuid>` + `From<UserId> for Uuid` |
| `features/todo/domain/values/todo_id.rs`    | Same                                       |
| `features/auth/domain/values/account_id.rs` | Same                                       |

---

## Part 3: Repository Renames + Uuid Updates

### Rename `Sqlite*` → `Postgres*` (6 files + 3 `mod.rs`)

| Old path                             | New path                               |
| ------------------------------------ | -------------------------------------- |
| `todo/.../sqlite_todo_repository.rs` | `todo/.../postgres_todo_repository.rs` |
| `user/.../sqlite_user_repository.rs` | `user/.../postgres_user_repository.rs` |
| `auth/.../sqlite_auth_repository.rs` | `auth/.../postgres_auth_repository.rs` |

Each repo's `mod.rs` updated to export new name. `main.rs` imports updated.

### Uuid changes in repo code

In each repo, ID fields change from `.to_string()` to direct `Uuid`:

- `Set(todo.id.to_string())` → `Set(todo.id.into())`
- `Column::Id.eq(id.to_string())` → `Column::Id.eq(*id)` or `Column::Id.eq(id.into())`

The `with_conn!` macro, `Conn` enum, and UoW are already DB-agnostic — no changes.

---

## Part 4: Migration Rewrites (PG dialect + native types)

All 5 migration Rust files rewritten. Raw SQL via `execute_unprepared` (existing pattern).

### `m20250701_000001_initial.rs` — todos table

```sql
CREATE TABLE IF NOT EXISTS todos (
    id          UUID PRIMARY KEY NOT NULL,
    title       TEXT NOT NULL,
    status      TEXT NOT NULL DEFAULT 'pending' CHECK(status IN ('pending', 'completed')),
    created_at  TIMESTAMPTZ NOT NULL,
    updated_at  TIMESTAMPTZ NOT NULL
);
```

### `m20250701_000002_auth.rs` — users, accounts, sessions + todos.user_id

```sql
CREATE TABLE IF NOT EXISTS users (
    id              UUID PRIMARY KEY NOT NULL,
    email           TEXT NOT NULL UNIQUE,
    email_verified  BOOLEAN NOT NULL DEFAULT FALSE,
    name            TEXT,
    image           TEXT,
    created_at      TIMESTAMPTZ NOT NULL,
    updated_at      TIMESTAMPTZ NOT NULL
);
CREATE TABLE IF NOT EXISTS accounts (
    id          UUID PRIMARY KEY NOT NULL,
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    account_id  TEXT NOT NULL,
    provider_id TEXT NOT NULL,
    -- ... token fields stay TEXT/nullable ...
    password    TEXT,
    created_at  TIMESTAMPTZ NOT NULL,
    updated_at  TIMESTAMPTZ NOT NULL
);
CREATE INDEX idx_accounts_provider ON accounts(provider_id, account_id);
CREATE INDEX idx_accounts_user_provider ON accounts(user_id, provider_id);
CREATE TABLE IF NOT EXISTS sessions (
    id          TEXT PRIMARY KEY NOT NULL,
    token       TEXT NOT NULL UNIQUE,
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    expires_at  TIMESTAMPTZ NOT NULL,
    ...
);
ALTER TABLE todos ADD COLUMN user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE;
```

**Note:** `user_id` added as `UUID NOT NULL` directly — migration 3 becomes unnecessary.

### `m20250701_000003_todo_user_id_not_null.rs` — **DELETE**

Folded into migration 2 (column created NOT NULL from the start). Remove from `migration/mod.rs` Migrator list.

### `m20250701_000004_add_todo_indexes.rs` — unchanged logic

```sql
CREATE INDEX IF NOT EXISTS idx_todos_user_created_at ON todos(user_id, created_at);
```

(PG-compatible as-is. Later replaced by partial index in migration 5.)

### `m20250704_000001_soft_delete_and_indexes.rs` — UP works on PG, DOWN rewrite

UP is already PG-valid (`ALTER TABLE ADD COLUMN`, partial index, DROP/CREATE INDEX). DOWN had SQLite table-rebuild — rewrite to simple PG:

```sql
ALTER TABLE todos DROP COLUMN deleted_at;
```

### `migration/mod.rs`

Remove `TodoUserIdNotNullMigration` from module list and Migrator vec.

---

## Reuse

- **SeaORM DSL** in all repos — zero query rewrites, only ID type changes
- **`with_conn!` macro** + `Conn` enum + `SeaOrmUnitOfWork` — fully DB-agnostic, no changes
- **`DateTimeUtc`** already maps to PG `TIMESTAMPTZ` — no schema model changes for timestamps
- **`bool`** already maps to PG `BOOLEAN` — no schema model changes for `email_verified`
- **Raw SQL migration pattern** (`UP`/`DOWN` constants + `execute_unprepared`) — keep as-is
- Existing `FromStr` + `Display` impls on newtypes stay (still useful for CLI/URL parsing)

---

## Steps

- [ ] **1.** Create `docker-compose.yml` (PG 19 Alpine) and `.env.example`
- [ ] **2.** `Cargo.toml`: swap `sqlx-sqlite` → `sqlx-postgres`, add `dotenvy`
- [ ] **3.** `config/mod.rs`: change default `DATABASE_URL` to PG connection string
- [ ] **4.** `main.rs`: add `dotenvy::dotenv().ok()`, update repo import names
- [ ] **5.** Domain newtypes: add `From<Uuid>` / `From<Self> for Uuid` (3 files)
- [ ] **6.** Schema models: `String` → `Uuid` for 6 ID columns (4 files)
- [ ] **7.** Mappers: `from_str` → `.into()` for ID fields (4 files)
- [ ] **8.** Rename 3 repo files `sqlite_*` → `postgres_*`; update `mod.rs` exports (6 files)
- [ ] **9.** Repo code: `.to_string()` → `.into()` for ID fields in `Set(...)` and `.eq(...)`
- [ ] **10.** Rewrite 5 migration files to PG dialect + native types
- [ ] **11.** `migration/mod.rs`: remove migration 3 from Migrator
- [ ] **12.** `docker compose up -d` → `cargo build -p api` → `cargo test -p api`

---

## Verification

```sh
# 1. Start PG
docker compose up -d

# 2. Compile
cargo build -p api

# 3. Tests
cargo test -p api

# 4. Run — migrations auto-apply on startup
cp .env.example .env
cargo run -p api
# → "API listening on http://localhost:3001"

# 5. Smoke test via curl
curl -s localhost:3001/api/health          # {"status":"ok"}
curl -s -X POST localhost:3001/api/auth/register \
  -H 'Content-Type: application/json' \
  -d '{"email":"a@b.c","password":"12345678","name":"Test"}'
# → creates user in PG

# 6. Verify native types in PG
docker exec app_db psql -U app -d app -c '\d users'
# → id uuid, email_verified boolean, created_at timestamp with time zone
docker exec app_db psql -U app -d app -c '\d todos'
# → id uuid, user_id uuid, deleted_at timestamp with time zone
```
