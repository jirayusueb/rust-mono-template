# Database Indexing

## Context

Every query path through the repositories was traced to check index coverage. **Only one is missing**: `todos.find_all(user_id)` does `WHERE user_id = ? ORDER BY created_at ASC` — a full table scan every time a user loads their todo list. Every other query is already covered by a PK, UNIQUE constraint, or existing index.

## Analysis

### Already covered (no action needed)

| Query                                                                           | Source                 | Index used                                         |
| ------------------------------------------------------------------------------- | ---------------------- | -------------------------------------------------- |
| `users.find_by_id` — `WHERE id = ?`                                             | sqlite_user_repository | PK `id`                                            |
| `users.find_by_email` — `WHERE email = ?`                                       | sqlite_user_repository | UNIQUE `email` (auto-indexed)                      |
| `accounts.find_credential_by_user_id` — `WHERE user_id = ? AND provider_id = ?` | sqlite_auth_repository | `idx_accounts_user_provider(user_id, provider_id)` |
| `sessions.find_session_by_token` — `WHERE token = ?`                            | sqlite_auth_repository | UNIQUE `token` (auto-indexed)                      |
| `sessions.delete_session` — `WHERE token = ?`                                   | sqlite_auth_repository | UNIQUE `token`                                     |
| `todos.find_by_id` — `WHERE id = ? AND user_id = ?`                             | sqlite_todo_repository | PK `id` (user_id is secondary filter)              |
| `todos.delete` — `WHERE id = ? AND user_id = ?`                                 | sqlite_todo_repository | PK `id`                                            |

### Missing — needs index

`todos.find_all` in `apps/api/src/features/todo/infrastructure/repositories/sqlite_todo_repository.rs`:

```rust
Entity::find()
    .filter(Column::UserId.eq(user_id.to_string()))
    .order_by(Column::CreatedAt, Order::Asc)
```

`todos` has only a PK on `id`. The `user_id` filter and `created_at` sort are unindexed → full scan + in-memory sort on every request.

## Approach

One composite index: `(user_id, created_at)`.

- Covers the `WHERE user_id = ?` filter (no scan).
- Returns rows pre-sorted by `created_at` within each user (no sort step).

Single-column `(user_id)` would also work for filtering but leaves SQLite sorting the result set. The composite is the same number of indexes (one) with strictly better query coverage, so there's no reason to pick the narrower one.

## Files to modify

| File                                                          | Change                                                                           |
| ------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| `apps/api/src/migration/m20250701_000004_add_todo_indexes.rs` | **New** — `CREATE INDEX idx_todos_user_created_at ON todos(user_id, created_at)` |
| `apps/api/src/migration/mod.rs`                               | Declare + register the new migration                                             |

## Reuse

- **Raw-SQL migration pattern** (`execute_unprepared` with `UP`/`DOWN` consts) — identical to `m20250701_000002_auth.rs` and `m20250701_000003_todo_user_id_not_null.rs`. No sea-query builder API needed; plain SQL is simpler and matches the existing style.
- **`IF NOT EXISTS`** in the `CREATE INDEX` — same idempotent pattern used by the auth migration's indexes.

## Steps

- [ ] Create `apps/api/src/migration/m20250701_000004_add_todo_indexes.rs` with:

  ```sql
  -- UP
  CREATE INDEX IF NOT EXISTS idx_todos_user_created_at ON todos(user_id, created_at);

  -- DOWN
  DROP INDEX IF EXISTS idx_todos_user_created_at;
  ```

- [ ] Register in `apps/api/src/migration/mod.rs` — `mod` declaration, `pub use`, and append to `Migrator::migrations()`
- [ ] `cargo build -p api` — compiles clean
- [ ] `cargo test -p api` — existing tests pass

## Verification

```sh
cargo build -p api
cargo test -p api

# Confirm the index exists after migration runs:
sqlite3 <db_path> ".indexes todos"
# Expected: idx_todos_user_created_at

# Confirm query plan uses the index:
sqlite3 <db_path> "EXPLAIN QUERY PLAN SELECT * FROM todos WHERE user_id = 'test' ORDER BY created_at ASC;"
# Expected: "SEARCH todos USING INDEX idx_todos_user_created_at (user_id=?)"
```
