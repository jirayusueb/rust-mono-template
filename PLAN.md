# apps/api — Coding Standard & Consistency Pass

## Context

The API follows vertical-slice / clean-architecture / CQRS with three features: **todo** (the original reference), **auth**, and **user**. As auth and user were added after todo, conventions drifted. This plan normalizes all three features to a single consistent standard, fixes 35 `cargo fmt` failures and 15 `cargo clippy` warnings, and cleans up minor hygiene issues.

**Direction**: auth and user (the newer, majority 2/3 features) define the established convention. Todo (the outlier) is normalized to match.

---

## Issues Found

### A. `cargo fmt` — 35 files unformatted

Run `cargo fmt`. No manual decisions needed.

### B. `cargo clippy` — 15 warnings (2 root causes)

| File                                                                           | Warning                                   | Fix                                                                        |
| ------------------------------------------------------------------------------ | ----------------------------------------- | -------------------------------------------------------------------------- |
| `shared/infrastructure/database/tx.rs:71` (in `with_conn!` macro, expands 13×) | `matching on Some with ok() is redundant` | `if let Some(slot) = ...try_get().ok()` → `if let Ok(slot) = ...try_get()` |
| `auth/infrastructure/repositories/postgres_auth_repository.rs:120,124`         | `useless conversion to same type`         | Remove `.into()` after `Expr::value(...)` (already returns `SimpleExpr`)   |

### C. Error enum naming inconsistency

| Feature  | Domain error         | Infra error         |
| -------- | -------------------- | ------------------- |
| **todo** | `DomainError` ❌     | `InfraError` ❌     |
| **auth** | `AuthDomainError` ✅ | `AuthInfraError` ✅ |
| **user** | `UserDomainError` ✅ | `UserInfraError` ✅ |

**Fix**: Rename todo's `DomainError` → `TodoDomainError`, `InfraError` → `TodoInfraError`. Update all references.

### D. Module encapsulation inconsistency (todo leaks internals)

| Module                               | todo                                               | auth/user                              |
| ------------------------------------ | -------------------------------------------------- | -------------------------------------- |
| `domain/mod.rs`                      | `pub mod entities; pub mod error; pub mod values;` | `mod entities; mod error; mod values;` |
| `infrastructure/repositories/mod.rs` | `pub mod postgres_todo_repository;`                | `mod postgres_*_repository;`           |

**Fix**: Change todo's `pub mod` → `mod` in these two files. The `pub use` re-exports already expose the public API.

### E. Missing derives on domain entities

| Entity    | Derives                      |
| --------- | ---------------------------- |
| `Todo`    | `Debug, Clone, Serialize` ✅ |
| `User`    | `Debug, Clone, Serialize` ✅ |
| `Account` | _(none)_ ❌                  |
| `Session` | _(none)_ ❌                  |

**Fix**: Add `#[derive(Debug, Clone)]` to `Account` and `Session`.

### F. Missing mock attribute on UserRepository port

Every port has `#[cfg_attr(test, mockall::automock)]` **except** `UserRepository`. Add it.

### G. Import hygiene

| File                                  | Issue                                                              | Fix                                  |
| ------------------------------------- | ------------------------------------------------------------------ | ------------------------------------ |
| `auth/application/ports/user_port.rs` | Inline `chrono::DateTime<chrono::Utc>`                             | Add `use chrono::{DateTime, Utc};`   |
| `auth/application/dtos/mod.rs`        | `use` statement at bottom of file                                  | Move to top                          |
| `postgres_todo_repository.rs`         | `use sea_orm::*;` wildcard before specific imports from same crate | Replace with explicit imports        |
| `todo/domain/values/todo_id.rs`       | `impl std::default::Default` fully-qualified                       | Import `Default`, use `impl Default` |

### H. Unnecessary boilerplate

`AuthDeps` has a manual `Clone` impl (all fields are `Arc`, trivially cloneable). Replace with `#[derive(Clone)]`.

### I. Redundant re-export (minor)

`auth/domain/values/mod.rs` re-exports `AccountId` and `Password`, but `auth/domain/mod.rs` already does the same re-export. Remove from `values/mod.rs` to match todo/user which don't double-export.

---

## Approach

Batch into three passes:

1. **Mechanical**: `cargo fmt` + `cargo clippy --fix` (warnings B, A)
2. **Naming & visibility**: Rename error enums (C), fix module visibility (D), remove redundant re-export (I)
3. **Hygiene & derives**: Add derives (E), add mock (F), fix imports (G), derive Clone (H)

## Files to modify

```
src/features/todo/domain/error.rs                          — rename DomainError → TodoDomainError
src/features/todo/domain/mod.rs                            — rename re-export, pub mod → mod
src/features/todo/domain/entities/todo.rs                  — update import path
src/features/todo/domain/values/{title,status}.rs          — update import path
src/features/todo/infrastructure/error.rs                  — rename InfraError → TodoInfraError
src/features/todo/infrastructure/mod.rs                    — rename re-export
src/features/todo/infrastructure/repositories/mod.rs       — pub mod → mod
src/features/todo/infrastructure/repositories/postgres_todo_repository.rs — fix imports, update error ref
src/features/todo/infrastructure/mapper/todo_mapper.rs     — update error ref
src/features/auth/domain/entities/{account,session}.rs     — add derives
src/features/auth/domain/values/mod.rs                     — remove redundant re-export
src/features/auth/application/dtos/mod.rs                  — move import to top
src/features/auth/application/ports/user_port.rs           — fix chrono import
src/features/auth/infrastructure/repositories/postgres_auth_repository.rs — remove .into()
src/features/user/application/ports/user_repository.rs     — add automock attr
src/shared/infrastructure/database/tx.rs                   — fix clippy in with_conn! macro
src/bootstrap/mod.rs                                       — derive Clone on AuthDeps (move from dtos)
src/features/auth/application/dtos/mod.rs                  — derive Clone on AuthDeps
+ cargo fmt on all 35 files
```

## Steps

- [ ] 1. Run `cargo fmt` (fixes A — 35 files)
- [ ] 2. Fix `with_conn!` macro clippy warning in `tx.rs` (B)
- [ ] 3. Remove redundant `.into()` in `postgres_auth_repository.rs` (B)
- [ ] 4. Rename `DomainError` → `TodoDomainError` across todo feature (C)
- [ ] 5. Rename `InfraError` → `TodoInfraError` across todo feature (C)
- [ ] 6. Change todo `domain/mod.rs` and `repositories/mod.rs` to private mods (D)
- [ ] 7. Add `#[derive(Debug, Clone)]` to `Account` and `Session` (E)
- [ ] 8. Add `#[cfg_attr(test, mockall::automock)]` to `UserRepository` (F)
- [ ] 9. Fix import hygiene: chrono in user_port, dtos import position, sea_orm wildcard, TodoId Default (G)
- [ ] 10. Replace manual `Clone` impl with `#[derive(Clone)]` on `AuthDeps` (H)
- [ ] 11. Remove redundant re-exports from `auth/domain/values/mod.rs` (I)
- [ ] 12. Run `cargo fmt && cargo clippy && cargo test` to verify zero warnings/errors

## Verification

```bash
cd apps/api
cargo fmt --check        # must report no diffs
cargo clippy --all-targets  # must report zero warnings
cargo test               # all tests pass
cargo build              # clean build
```
