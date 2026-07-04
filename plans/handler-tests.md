# Plan: Application-Layer Handler Tests with rstest + mockall

## Context

The codebase has domain-layer tests only (Todo state machine, Password, Email value objects). Zero application-layer tests exist — all 9 handlers across auth + todo features are untested. These handlers contain real branching logic (auth flows, error mapping, domain-state guards) that should be verified with mocked ports.

`rstest` for parametrized test cases, `mockall` for auto-generating mock implementations of the 4 port traits.

## Approach

1. Add `rstest` + `mockall` as dev-dependencies.
2. Annotate each port trait with `#[cfg_attr(test, mockall::automock)]` — generates `MockXxx` structs available to every inline test module in the crate.
3. Write inline `#[cfg(test)] mod tests` at the bottom of each handler file, using `rstest` parametrization for multi-case scenarios.

## Port Traits to Mock

| Trait            | File                                                 | Mock struct          |
| ---------------- | ---------------------------------------------------- | -------------------- |
| `TodoRepository` | `features/todo/application/ports/todo_repository.rs` | `MockTodoRepository` |
| `AuthRepository` | `features/auth/application/ports/auth_repository.rs` | `MockAuthRepository` |
| `UserPort`       | `features/auth/application/ports/user_port.rs`       | `MockUserPort`       |
| `PasswordHasher` | `shared/application/utils/password_hasher.rs`        | `MockPasswordHasher` |

`UserRepository` is NOT mocked — handlers never use it directly (only `UserPortAdapter` bridges to it).

## Files to Modify

- `apps/api/Cargo.toml` — add dev-dependencies
- `apps/api/src/features/todo/application/ports/todo_repository.rs` — add automock
- `apps/api/src/features/auth/application/ports/auth_repository.rs` — add automock
- `apps/api/src/features/auth/application/ports/user_port.rs` — add automock
- `apps/api/src/shared/application/utils/password_hasher.rs` — add automock
- `apps/api/src/features/todo/application/commands/create.rs` — add tests
- `apps/api/src/features/todo/application/commands/update.rs` — add tests
- `apps/api/src/features/todo/application/commands/delete.rs` — add tests
- `apps/api/src/features/todo/application/queries/get.rs` — add tests
- `apps/api/src/features/todo/application/queries/list.rs` — add tests
- `apps/api/src/features/auth/application/commands/login.rs` — add tests
- `apps/api/src/features/auth/application/commands/register.rs` — add tests
- `apps/api/src/features/auth/application/commands/logout.rs` — add tests
- `apps/api/src/features/auth/application/queries/get_current_user.rs` — add tests

## Test Cases per Handler

### Todo feature (MockTodoRepository)

**CreateTodoHandler** — 2 tests:

- [ ] ✅ Creates todo, returns ID when save succeeds
- [ ] ❌ Save fails → `Internal` error propagated

**UpdateTodoHandler** — rstest parametrized + standalone:

- [ ] ✅ `#[rstest]` update combinations: title-only, status→Completed, status→Pending, both, neither
- [ ] ❌ Not found → `NotFound`
- [ ] ❌ Complete already-completed → `Conflict`
- [ ] ❌ Reopen pending → `Conflict`

**DeleteTodoHandler** — 2 tests:

- [ ] ✅ Deletes existing todo
- [ ] ❌ Not found → `NotFound`

**GetTodoHandler** — 2 tests:

- [ ] ✅ Returns todo when found
- [ ] ❌ `NotFound` when missing

**ListTodosHandler** — 2 tests:

- [ ] ✅ Returns todos for user
- [ ] ✅ Returns empty list

### Auth feature (MockAuthRepository + MockUserPort + MockPasswordHasher)

**LoginHandler** — rstest parametrized:

- [ ] ✅ Successful login (password verifies, session created)
- [ ] `#[rstest]` failure paths: user not found, credential not found, password mismatch → all `Unauthorized`

**RegisterHandler** — 2-3 tests:

- [ ] ✅ Successful registration (user created, credential saved, session created)
- [ ] ❌ Email already taken → `Conflict`
- [ ] ❌ Weak password → `Validation`

**LogoutHandler** — 2 tests:

- [ ] ✅ Deletes session
- [ ] ❌ Delete fails → error propagated

**GetCurrentUserHandler** — rstest parametrized:

- [ ] ✅ Returns user + session info when valid
- [ ] `#[rstest]` failure paths: session not found, session expired, user not found → all `Unauthorized`

## Implementation Steps

- [ ] Add `[dev-dependencies]` to `apps/api/Cargo.toml`: `rstest = "0.25"`, `mockall = "0.13"`
- [ ] Add `#[cfg_attr(test, mockall::automock)]` above `#[async_trait]` on all 4 port traits
- [ ] Verify `cargo check --tests` compiles (mocks generated)
- [ ] Write todo handler tests (5 files) — simpler CRUD, good warmup
- [ ] Write auth handler tests (4 files) — more complex, multi-mock wiring via `AuthDeps`
- [ ] `cargo test` — all green
- [ ] `cargo clippy --tests` — no warnings

## Verification

- `cargo test` — all new tests pass, existing domain tests still pass
- `cargo clippy --tests` — clean
- Spot-check: a deliberately broken mock expectation (wrong return) causes test failure
