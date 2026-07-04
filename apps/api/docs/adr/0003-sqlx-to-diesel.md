# ADR-0003: Migration from sqlx to Diesel

**Date:** 2025-07-02  
**Status:** Accepted

## Context

The API originally used **sqlx** for SQLite persistence: raw SQL strings, positional
tuple-row `query_as`, manual mappers converting `String` ↔ domain newtypes, and
`sqlx::migrate!` for schema management.

Motivation for migration: gain ORM-level features (Queryable/Insertable derives,
type-safe query DSL, auto-generated schema.rs) to reduce boilerplate and catch
query errors at compile time.

## Decision

1. **diesel-async** with `SyncConnectionWrapper<SqliteConnection>`. SQLite has no
   true async driver — SyncConnectionWrapper wraps the sync Diesel SQLite connection
   behind an async API surface. This keeps `#[async_trait]` repo signatures unchanged.

2. **bb8 pool** via `diesel_async::pooled_connection::bb8::Pool`. Pool is `Clone`,
   cloned into each repo — same pattern as sqlx's `SqlitePool`.

3. **Full Diesel DSL** — all `query_as("SELECT...")` replaced with
   `table.filter(...).select(...).get_result()`. Schema auto-generated from
   migrations via `diesel print-schema` → `src/schema.rs`.

4. **Keep domain/infra separation.** Domain entities (Todo, User, Account, Session)
   stay Diesel-free. Diesel `Queryable`/`Selectable` structs live in
   `infrastructure/schema/`. Mapper converts row → domain entity. Mapper boilerplate
   survives but simplifies (no string→newtype parsing).

5. **FromSql on domain newtypes** (Title, Status, Email, UserId, TodoId) — they
   deserialize directly from SQLite Text columns. **ToSql** implemented on Title,
   Status, Email (str delegation via `as_str()` / `&'static str` match). **UserId
   and TodoId (Uuid-backed) omit ToSql** due to Diesel's `to_sql<'b>` lifetime
   constraint: local allocations can't satisfy `'b` tied to the Output. Instead,
   these types use `.to_string()` in repo expressions. Trade-off: minor verbosity
   in repositories for ID columns only.

6. **Sync migrations at startup.** `SqliteConnection::run_pending_migrations` runs
   synchronously before the async pool is created. SQLite is blocking anyway, and
   migrations only run once at startup.

## Consequences

- `sqlx` removed; `diesel`, `diesel-async`, `bb8`, `diesel_migrations` added.
- 5 domain VO files coupled to Diesel (FromSql/ToSql or FromSqlRow derives).
- 3 error files wrap `diesel::result::Error` + pool errors.
- 4 mapper files simplified (timestamp parsing + nullable handling only).
- 3 repositories rewritten with type-safe DSL.
- Domain entities, application ports, presentation layer **unchanged**.

## Rejected Alternatives

- **sync Diesel + `spawn_blocking`**: more ceremony, no benefit over SyncConnectionWrapper.
- **Merge Diesel structs into domain**: loses persistence ignorance.
- **`diesel::sql_query` raw SQL**: would keep raw SQL strings, defeating the purpose.
