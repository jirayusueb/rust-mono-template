# Refactor All Mappers to Gist Pattern

## Context

The gist ([RezaOwliaei/elysia-clean-architecture-guide](https://gist.github.com/RezaOwliaei/477ed74fc77aa5df2a854789538dd79d)) prescribes mappers as a dedicated type with directional mapping methods, at both layers:

- **Infrastructure Data Mappers**: `toDomain` / `toPersistence` — DB row ↔ domain entity, using the Rehydration Pattern (`restore()` not `create()`).
- **Presentation Mappers**: Transport DTO ↔ use-case DTO / domain port types.

Current state already follows rehydration (`restore()` in all infra mappers ✅). What's missing:

1. **No `to_persistence`** in infra mappers — repos inline `ActiveModel` construction.
2. **Free functions** instead of named `Mapper` types (gist uses `UserMapper.toDomain()`).
3. **Presentation mappers use `impl From<>`** — gist prescribes a dedicated `Mapper` struct that maps transport DTO ↔ use-case DTO / domain port types.
4. **Presentation DTOs are monolithic `mod.rs`** — gist prescribes one file per DTO.

## Approach

### A. Infrastructure mappers

Convert each infra mapper from free functions → `Mapper` struct with `to_domain` + `to_active_model`. Extract inline `ActiveModel` construction from repos into the mapper.

Query logic stays in repos — only mapping moves:

- `on_conflict` upsert logic (Todo, User repos)
- `update_session_expiry` column-expr update (Auth repo)
- `delete` soft-delete queries

### B. Presentation mappers

Convert `impl From<>` → dedicated `Mapper` struct with associated functions. Routes call `AuthMapper::to_sign_up_command(req)` etc. instead of `UserResponse::from(user)`.

**Auth** (`AuthMapper`):

- `to_sign_up_command(SignUpRequest) -> SignUpCommand`
- `to_sign_in_command(SignInRequest, ip, user_agent) -> SignInCommand`
- `to_user_response(AuthUserInfo) -> UserResponse`
- `to_session_details(SessionInfo) -> SessionDetails`
- `to_session_response(AuthUserInfo, SessionInfo) -> SessionResponse`

**Todo** (`TodoMapper`):

- `to_create_command(CreateTodoRequest, UserId) -> Result<CreateTodoCommand, AppError>`
- `to_update_command(UpdateTodoRequest, UserId, TodoId) -> Result<UpdateTodoCommand, AppError>`
- `to_todo_response(Todo) -> TodoResponse`
- `to_create_response(TodoId) -> CreateTodoResponse`

Note: `Title::new` (value-object validation) stays in the mapper — it's input shaping, and the command struct requires a `Title` not a raw `String`.

### C. Presentation DTOs → multi-file

Split `dtos/mod.rs` into one file per DTO, matching the gist structure:

```
auth/presentation/http/dtos/
  mod.rs                    (re-exports)
  sign_up_request.rs
  sign_in_request.rs
  user_response.rs
  session_details.rs        (was SessionResponse — inner session metadata)
  session_response.rs       (was SessionPayload — full GET /session wrapper)

todo/presentation/http/dtos/
  mod.rs                    (re-exports)
  create_todo_request.rs
  update_todo_request.rs
  todo_response.rs
  create_todo_response.rs
```

**Naming fix**: `SessionPayload` → `SessionResponse` (it's the `GET /auth/session` response), and inner `SessionResponse` → `SessionDetails` (session metadata). This aligns all DTOs on `*Request` / `*Response`. The web client (`apps/web/src/lib/api.ts`, `auth-client.tsx`) mirrors the type and gets the same rename.

## Files to modify

### Infrastructure mappers + repos

| File                                                           | Change                     |
| -------------------------------------------------------------- | -------------------------- |
| `auth/infrastructure/mappers/account_mapper.rs`                | Struct + `to_active_model` |
| `auth/infrastructure/mappers/session_mapper.rs`                | Struct + `to_active_model` |
| `auth/infrastructure/mappers/mod.rs`                           | Re-export structs          |
| `auth/infrastructure/repositories/postgres_auth_repository.rs` | Delegate saves to mappers  |
| `todo/infrastructure/mappers/todo_mapper.rs`                   | Struct + `to_active_model` |
| `todo/infrastructure/mappers/mod.rs`                           | Re-export struct           |
| `todo/infrastructure/repositories/postgres_todo_repository.rs` | Delegate saves to mapper   |
| `user/infrastructure/mappers/user_mapper.rs`                   | Struct + `to_active_model` |
| `user/infrastructure/mappers/mod.rs`                           | Re-export struct           |
| `user/infrastructure/repositories/postgres_user_repository.rs` | Delegate saves to mapper   |

### Presentation mappers

| File                                    | Change                                                       |
| --------------------------------------- | ------------------------------------------------------------ |
| `auth/presentation/http/mappers/mod.rs` | `struct AuthMapper` with associated fns                      |
| `todo/presentation/http/mappers/mod.rs` | `struct TodoMapper` with associated fns                      |
| `auth/presentation/http/routes.rs`      | Call `AuthMapper::` instead of `From::from`                  |
| `todo/presentation/http/routes.rs`      | Call `TodoMapper::` instead of `From::from` + inline mapping |

### Presentation DTOs → multi-file

| File                                                  | Change                                  |
| ----------------------------------------------------- | --------------------------------------- |
| `auth/presentation/http/dtos/mod.rs`                  | Split into per-DTO files, re-export     |
| `auth/presentation/http/dtos/sign_up_request.rs`      | New — extracted                         |
| `auth/presentation/http/dtos/sign_in_request.rs`      | New — extracted                         |
| `auth/presentation/http/dtos/user_response.rs`        | New — extracted                         |
| `auth/presentation/http/dtos/session_details.rs`      | New — extracted (was `SessionResponse`) |
| `auth/presentation/http/dtos/session_response.rs`     | New — extracted (was `SessionPayload`)  |
| `todo/presentation/http/dtos/mod.rs`                  | Split into per-DTO files, re-export     |
| `todo/presentation/http/dtos/create_todo_request.rs`  | New — extracted                         |
| `todo/presentation/http/dtos/update_todo_request.rs`  | New — extracted                         |
| `todo/presentation/http/dtos/todo_response.rs`        | New — extracted                         |
| `todo/presentation/http/dtos/create_todo_response.rs` | New — extracted                         |

## Reuse

- `Entity::restore()` factories — already exist on all entities (added in prior commit)

## Steps

### Infrastructure

- [ ] 1. `account_mapper.rs` → `struct AccountMapper { to_domain, to_active_model }`
- [ ] 2. `session_mapper.rs` → `struct SessionMapper { to_domain, to_active_model }`
- [ ] 3. `todo_mapper.rs` → `struct TodoMapper { to_domain, to_active_model }`
- [ ] 4. `user_mapper.rs` → `struct UserMapper { to_domain, to_active_model }`
- [ ] 5. Update infra `mod.rs` files to re-export structs
- [ ] 6. Update 3 postgres repositories — delegate save mapping to mappers

### Presentation DTOs

- [ ] 7. Split auth `dtos/mod.rs` → 5 per-DTO files + re-export `mod.rs`
- [ ] 8. Split todo `dtos/mod.rs` → 4 per-DTO files + re-export `mod.rs`

### Presentation mappers

- [ ] 9. `auth/presentation/http/mappers/mod.rs` → `struct AuthMapper`
- [ ] 10. `todo/presentation/http/mappers/mod.rs` → `struct TodoMapper`
- [ ] 11. Update `auth/routes.rs` — call `AuthMapper::` methods
- [ ] 12. Update `todo/routes.rs` — call `TodoMapper::` methods

- [ ] 13. Rename `SessionPayload` → `SessionResponse`, inner `SessionResponse` → `SessionDetails` across API + web

### Verify

- [ ] 14. `cargo check` + `cargo test`
- [ ] 15. `cd apps/web && npx tsc --noEmit`

## Verification

```sh
cd apps/api && cargo check    # compiles
cd apps/api && cargo test     # all tests pass
```
