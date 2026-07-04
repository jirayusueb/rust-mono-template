# Naming Convention Review Plan

<!-- markdownlint-disable MD013 -->

## Context

The user asked to review naming conventions against the referenced handbook: <https://gist.github.com/RezaOwliaei/477ed74fc77aa5df2a854789538dd79d>.

Initial findings:

- This repo is a monorepo with `apps/api` (Rust/Axum, SeaORM) and `apps/web` (Waku/React).
- The referenced handbook is TypeScript/Elysia-oriented, but its architecture/naming concepts map partly to this codebase: feature-first folders, clean architecture layers, ports/adapters, DTOs, handlers/controllers/routes, repositories, mappers, and shared kernel/application/infrastructure/presentation layers.
- `apps/api/CONTEXT.md` already defines domain language for Todo/Auth/Architecture and should be treated as the repo-local naming source alongside the gist.
- Current backend structure already follows the broad feature-first layout under `apps/api/src/features/{auth,todo,user}` with `domain`, `application`, `infrastructure`, and `presentation` layers.
- Current API naming is idiomatic Rust (`snake_case` files/modules, `PascalCase` types, `snake_case` functions), so the review should adapt the handbook’s TypeScript examples rather than copy TS file casing literally.

## Approach

Scope is `apps/api` only.

Make the smallest naming cleanup that aligns the Rust/Axum backend with the handbook's architecture terms and the repo-local language in `apps/api/CONTEXT.md`. Treat TypeScript/Elysia-specific examples as concepts, not literal casing rules, and write the Rust mapping down so future names have one local source.

Keep these as accepted Rust adaptations, not rename work:

- `snake_case` Rust files/modules instead of TypeScript `PascalCase.ts` files.
- Traits without `I` prefixes (`TodoRepository`, not `ITodoRepository`).
- Axum `routes.rs` modules instead of Elysia `*Controller.ts` classes.
- Grouped small DTOs in `dtos/mod.rs` instead of one file per DTO.
- Serde/validator request structs instead of TypeBox `*Schema` runtime objects.

Rust naming patterns to enforce:

| Concept      | Rust pattern                                                                                                                                              |
| ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Entity       | Singular `PascalCase` type in `domain/entities/<entity>.rs`; re-export from `domain/mod.rs` (`Todo`, `User`, `Account`, `Session`).                       |
| Value object | Singular `PascalCase` type in `domain/values/<value>.rs`; IDs use `*Id`; constructors stay `new` (`Title`, `Email`, `Password`, `TodoId`).                |
| Mapper       | Infrastructure mapper module with free functions like `to_domain`; no class/struct mapper unless state is needed. Use `mappers/` consistently.            |
| DTO          | Plain `PascalCase` structs in `application/dtos` or `presentation/http/dtos`; app DTOs use `*Command` / `*Query`, HTTP DTOs use `*Request` / `*Response`. |
| Repository   | Port trait named `*Repository` in `application/ports/*_repository.rs`; concrete adapter named `<Backend>*Repository` in `infrastructure/repositories`.    |
| Port         | Trait in `application/ports`, no TypeScript `I` prefix; cross-feature ports may be capability names like `UserPort`.                                      |

Recommended cleanup:

1. Add a short Rust naming convention section to `apps/api/CONTEXT.md` using the patterns above.
2. Rename infrastructure `mapper` modules/directories to `mappers` for consistency with the handbook and existing presentation naming.
3. Add explicit `Command`/`Query` suffixes to Todo application DTOs so they match the CQRS language already used by Auth and `CONTEXT.md`.
4. Rename the local `cmd` parameter in `GetCurrentUserHandler::handle` to `query` because it handles `GetCurrentUserQuery`.
5. Leave feature names (`auth`, `todo`, `user`) and route names unchanged; they are consistent with the current domain language and Rust module style.

## Files to modify

Likely future implementation files:

- `apps/api/CONTEXT.md`
- `apps/api/src/features/auth/infrastructure/mapper/mod.rs` → `apps/api/src/features/auth/infrastructure/mappers/mod.rs`
- `apps/api/src/features/auth/infrastructure/mapper/account_mapper.rs` → `apps/api/src/features/auth/infrastructure/mappers/account_mapper.rs`
- `apps/api/src/features/auth/infrastructure/mapper/session_mapper.rs` → `apps/api/src/features/auth/infrastructure/mappers/session_mapper.rs`
- `apps/api/src/features/auth/infrastructure/mod.rs`
- `apps/api/src/features/auth/infrastructure/repositories/postgres_auth_repository.rs`
- `apps/api/src/features/todo/infrastructure/mapper/mod.rs` → `apps/api/src/features/todo/infrastructure/mappers/mod.rs`
- `apps/api/src/features/todo/infrastructure/mapper/todo_mapper.rs` → `apps/api/src/features/todo/infrastructure/mappers/todo_mapper.rs`
- `apps/api/src/features/todo/infrastructure/mod.rs`
- `apps/api/src/features/todo/infrastructure/repositories/postgres_todo_repository.rs`
- `apps/api/src/features/user/infrastructure/mapper/mod.rs` → `apps/api/src/features/user/infrastructure/mappers/mod.rs`
- `apps/api/src/features/user/infrastructure/mapper/user_mapper.rs` → `apps/api/src/features/user/infrastructure/mappers/user_mapper.rs`
- `apps/api/src/features/user/infrastructure/mod.rs`
- `apps/api/src/features/user/infrastructure/repositories/postgres_user_repository.rs`
- `apps/api/src/features/todo/application/dtos/mod.rs`
- `apps/api/src/features/todo/application/commands/create.rs`
- `apps/api/src/features/todo/application/commands/update.rs`
- `apps/api/src/features/todo/application/commands/delete.rs`
- `apps/api/src/features/todo/application/queries/get.rs`
- `apps/api/src/features/todo/application/queries/list.rs`
- `apps/api/src/features/todo/presentation/http/routes.rs`
- `apps/api/src/features/auth/application/queries/get_current_user.rs`

## Reuse

Existing references to reuse instead of inventing a new convention:

- `apps/api/CONTEXT.md` — repo-local ubiquitous language and architecture vocabulary.
- `apps/api/docs/adr/0001-vertical-slice-clean-architecture-cqrs.md` — confirms handler-level CQRS and ports/adapters naming.
- Gist sections:
  - Project structure / feature-first layout.
  - Application layer naming: use cases/commands/queries, ports, DTOs.
  - Infrastructure naming: concrete repositories/adapters/mappers/services.
  - Presentation naming: controllers/routes, request/response DTOs, mappers.
- Existing code patterns to keep:
  - `*Repository` trait ports and `Postgres*Repository` concrete adapters.
  - `*Handler` command/query handlers.
  - `*Request` / `*Response` HTTP DTOs.
  - Domain names: `Todo`, `TodoId`, `Title`, `Status`, `Pending`, `Completed`, `User`, `Account`, `Session`, `ProviderId`, `Email`.

## Steps

- [ ] Add a compact Rust naming convention section to `apps/api/CONTEXT.md` for entity, mapper, DTO, value-object, repository, and port patterns.
- [ ] Rename infrastructure `mapper` modules to `mappers` in auth, todo, and user, then update all `use ...::mapper::...` imports.
- [ ] In `todo/application/dtos/mod.rs`, rename:
  - `CreateTodo` → `CreateTodoCommand`
  - `UpdateTodo` → `UpdateTodoCommand`
  - `DeleteTodo` → `DeleteTodoCommand`
  - `GetTodo` → `GetTodoQuery`
  - `ListTodos` → `ListTodosQuery`
- [ ] Update Todo command/query handlers, route imports, route DTO construction, and tests to use the renamed DTOs.
- [ ] In `auth/application/queries/get_current_user.rs`, rename the `handle(&self, cmd: GetCurrentUserQuery)` parameter and references to `query`.
- [ ] Run formatting and compile/test checks.
- [ ] Skip broader cosmetic renames unless a compile error exposes a real missed reference.

## Verification

After implementation:

- `cargo fmt --check -p api`
- `cargo check -p api`
- `cargo test -p api`
