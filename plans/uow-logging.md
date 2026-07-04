# Unit of Work + Structured Logging (tracing)

## Context

Two cross-cutting concerns from the Clean Architecture guide are missing:

1. **No transactional safety.** The `register` handler writes to 3 tables (user → credential → session) across 2 repositories (`UserPort`, `AuthRepository`). If `save_session` fails, the user and credential are already committed — partial state. The guide's Unit of Work pattern lets the application layer define a transaction boundary without importing DB types.

2. **No structured logging.** `main.rs` uses `println!`. The guide calls for structured logging as a cross-cutting concern. In Rust, `tracing` + `tracing-subscriber` IS the interface — no custom `ILogger` trait needed (the facade/implementation split is built in).

## Approach

### Unit of Work

The guide uses TypeScript's `AsyncLocalStorage` to propagate the transaction to repos transparently. The Rust equivalent is `tokio::task_local!`.

**Flow:**

1. `UnitOfWork` trait (application layer) defines `run_in_transaction(work)` — takes a boxed future, returns `Result<(), AppError>`.
2. `SeaOrmUnitOfWork` (infrastructure) calls `db.begin()`, stores the `DatabaseTransaction` in a task-local `Arc<Mutex<Option<DatabaseTransaction>>>` slot, runs the work, then commits on `Ok` / auto-rollbacks on `Err` (via `DatabaseTransaction::Drop`).
3. Repos check the task-local slot. If a txn is active, they execute queries against it (`&dyn ConnectionTrait`); otherwise they use the pooled `DatabaseConnection`. Both types implement `ConnectionTrait` (object-safe via `#[async_trait]`), so `exec(conn)` / `one(conn)` / `all(conn)` accept either.
4. `NoopUnitOfWork` for tests — just runs the work, no real transaction.

**Why only `register`?** Login and logout each do a single write. A single DB operation is already atomic. Wrapping it in a transaction adds overhead for zero benefit. UoW is for _multi-write atomicity_ — that's `register` only (3 writes across 2 repos).

**Result capture from inside the transaction:** The trait method returns `Result<(), AppError>` (not generic `T`) to stay object-safe (`Arc<dyn UnitOfWork>`). The register handler captures its return value via `Arc<std::sync::Mutex<Option<AuthResult>>>` — the standard Rust pattern for getting a typed result out of a boxed future.

### Logging

`tracing` macros are the facade (already zero-cost if no subscriber). `tracing-subscriber` is the implementation. Add `TraceLayer` (from `tower-http`, already a dep — just needs the `"trace"` feature) for automatic HTTP request/response spans.

## Files to modify

### Unit of Work — new files

| File                                             | Purpose                                           |
| ------------------------------------------------ | ------------------------------------------------- |
| `shared/application/unit_of_work.rs`             | `UnitOfWork` trait + `NoopUnitOfWork` (test impl) |
| `shared/infrastructure/database/tx.rs`           | `task_local!` slot + `with_conn!` macro for repos |
| `shared/infrastructure/database/unit_of_work.rs` | `SeaOrmUnitOfWork` impl                           |

### Unit of Work — modified files

| File                                                                  | Change                                                                 |
| --------------------------------------------------------------------- | ---------------------------------------------------------------------- |
| `shared/application/mod.rs`                                           | Add `pub mod unit_of_work;`                                            |
| `shared/infrastructure/database/mod.rs`                               | Add `pub mod tx; pub mod unit_of_work;`                                |
| `features/auth/application/dtos/mod.rs`                               | Add `uow: Arc<dyn UnitOfWork>` to `AuthDeps`                           |
| `bootstrap/mod.rs`                                                    | Add `uow: Arc<dyn UnitOfWork>` to `AppState`                           |
| `features/auth/application/commands/register.rs`                      | Wrap 3 writes in `uow.run_in_transaction`; capture result via slot     |
| `features/user/infrastructure/repositories/sqlite_user_repository.rs` | Use `with_conn!` macro for all queries                                 |
| `features/auth/infrastructure/repositories/sqlite_auth_repository.rs` | Use `with_conn!` macro for all queries                                 |
| `features/todo/infrastructure/repositories/sqlite_todo_repository.rs` | Use `with_conn!` macro for all queries                                 |
| `features/auth/infrastructure/adapters/user_port_adapter.rs`          | No change — delegates to `UserRepository` which handles txn internally |
| `main.rs`                                                             | Wire `SeaOrmUnitOfWork` into `AppState`                                |

### Logging — modified files

| File                                    | Change                                                                                 |
| --------------------------------------- | -------------------------------------------------------------------------------------- |
| `apps/api/Cargo.toml`                   | Add `tracing`, `tracing-subscriber`; add `"trace"` to `tower-http` features            |
| `shared/infrastructure/logging/mod.rs`  | Replace stub with `init()` function                                                    |
| `main.rs`                               | Call `logging::init()`; replace `println!` with `tracing::info!`; add `TraceLayer`     |
| `shared/presentation/mod.rs`            | Add `pub mod middleware;`                                                              |
| `shared/presentation/middleware/mod.rs` | New: request-id middleware (generates UUID, sets `x-request-id` header + tracing span) |

## Reuse

- **SeaORM `ConnectionTrait`** — both `DatabaseConnection` and `DatabaseTransaction` implement it; object-safe via `#[async_trait]`. Query methods (`exec`, `one`, `all`) accept `&C where C: ConnectionTrait`, so `&dyn ConnectionTrait` works for both.
- **SeaORM `TransactionTrait::begin()`** — returns owned `DatabaseTransaction`; `commit(self)` consumes it; `Drop` auto-rollbacks if not committed.
- **`tokio::task_local!`** — Rust equivalent of Node.js `AsyncLocalStorage`. Stores the txn slot per-task.
- **`tower-http`** — already in deps; just needs `"trace"` feature enabled.
- **`uuid`** — already in deps (`v4` feature); used for request IDs.

## Steps

### Part 1: Dependencies

- [ ] **1.** Add to `apps/api/Cargo.toml`:

  ```toml
  tracing = "0.1"
  tracing-subscriber = { version = "0.3", features = ["env-filter"] }
  ```

  Change `tower-http` to `tower-http = { version = "0.6", features = ["cors", "trace"] }`.

### Part 2: Structured Logging

- [ ] **2.** Implement `shared/infrastructure/logging/mod.rs`:
  - `pub fn init()` — sets up `tracing_subscriber::fmt()` with `EnvFilter` (defaults to `info`, overridable via `RUST_LOG`).
  - Call from `main.rs` as the first line.
- [ ] **3.** Add request-ID middleware (`shared/presentation/middleware/`):
  - Generates `Uuid::new_v4()`, sets `x-request-id` response header.
  - Creates a `tracing::Span` with the request ID so all logs within the request are correlated.
  - Follows the guide's "Correlation ID" pattern.
- [ ] **4.** Wire into `main.rs`:
  - `logging::init()` before anything else.
  - Replace `println!("API listening...")` with `tracing::info!("API listening...")`.
  - Add `.layer(TraceLayer::new_for_http())` and the request-ID middleware to the router.

### Part 3: Unit of Work

- [ ] **5.** Create `shared/application/unit_of_work.rs`:

  ```rust
  #[cfg_attr(test, mockall::automock)]
  #[async_trait]
  pub trait UnitOfWork: Send + Sync {
      async fn run_in_transaction(
          &self,
          work: Pin<Box<dyn Future<Output = Result<(), AppError>> + Send>>,
      ) -> Result<(), AppError>;
  }

  // Test impl — runs work without a real transaction
  pub struct NoopUnitOfWork;
  ```

  Add `pub mod unit_of_work;` to `shared/application/mod.rs`.

- [ ] **6.** Create `shared/infrastructure/database/tx.rs`:
  - `task_local! { pub static CURRENT_TX: Arc<tokio::sync::Mutex<Option<DatabaseTransaction>>>; }`
  - `with_conn!` macro: checks `CURRENT_TX.try_get()`, locks the slot, yields `&dyn ConnectionTrait` (txn if active, else `&self.db`).

- [ ] **7.** Create `shared/infrastructure/database/unit_of_work.rs`:
  - `SeaOrmUnitOfWork { db: DbPool }`
  - `run_in_transaction`: `db.begin()`, store in `Arc<Mutex<Some(txn)>>`, `CURRENT_TX.scope(slot, work)`, commit on Ok / drop on Err.
  - Add modules to `shared/infrastructure/database/mod.rs`.

- [ ] **8.** Update the 3 SQLite repos to use `with_conn!`:
  - Replace each `.exec(&self.db)` / `.one(&self.db)` / `.all(&self.db)` call with `with_conn!(self, |conn| ...)`.
  - Pattern: match on `CURRENT_TX.try_get()`, lock slot, get `&dyn ConnectionTrait`, run query.

- [ ] **9.** Add `uow: Arc<dyn UnitOfWork>` to `AuthDeps` and `AppState`. Wire `SeaOrmUnitOfWork` in `main.rs`.

- [ ] **10.** Update `register.rs`:
  - Wrap the 3 writes (create user, save credential, save session) inside `self.deps.uow.run_in_transaction(Box::pin(async move { ... }))`.
  - Capture `AuthResult` via `Arc<std::sync::Mutex<Option<AuthResult>>>`.
  - Keep email-existence check OUTSIDE the transaction (read-only, no need for txn).
  - Update tests: add `Arc::new(NoopUnitOfWork)` to `AuthDeps`.

## Verification

```sh
cargo build -p api           # compiles clean
cargo test -p api            # all existing tests pass (NoopUnitOfWork used in test deps)
cargo run -p api             # server starts, logs are structured (tracing format)

# Manual: POST /api/auth/register → should work as before, now with transactional safety
# Manual: check logs show request IDs and structured fields
# Manual: set RUST_LOG=debug → see TraceLayer request/response logs + SeaORM query traces
```

## Notes / Ponytail

- **`login` and `logout` NOT wrapped in UoW.** Each does a single DB write — already atomic. Wrapping adds a round-trip for zero benefit. Add when a handler needs multi-write atomicity.
- **No custom `ILogger` trait.** The guide defines `ILogger` because TypeScript lacks a built-in logging facade. Rust's `tracing` IS the facade — macros are the interface, subscriber is the swappable implementation. Defining a wrapper trait would be re-implementing what `tracing` already provides.
- **`with_conn!` macro instead of a helper function.** The task-local `MutexGuard` can't outlive the function call (it must span the `.await`). A macro inlines the guard into the method body, keeping the borrow alive across the await. This is the standard Rust workaround for "return a borrow from a lock guard."
- **`Result<(), AppError>` not `Result<T, AppError>`.** Generic methods on `async_trait` break object safety (`Arc<dyn UnitOfWork>` stops working). The slot-capture pattern for return values is the trade-off — ugly but localized to `register.rs`, the only handler that needs it.
- **JSON logging format skipped.** Default `fmt` subscriber produces readable text. Switch to `.json()` on the subscriber when shipping to a log aggregator — one line change, no code touch.
