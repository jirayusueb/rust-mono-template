# API

A Rust/Axum backend serving as the architecture showcase for a monorepo starter
template. The Todo feature is the reference implementation proving that vertical
slice, ports/adapters, and CQRS compose cleanly.

## Language

**Todo**:
An actionable item with a title and a lifecycle state.
_Avoid_: Task, item, checklist item

**TodoId**:
The unique identifier for a Todo.
_Avoid_: Task ID, identifier

**Title**:
The human-readable description of what a Todo is about.
_Avoid_: Name, label, text

**Status**:
The lifecycle state of a Todo — Pending or Completed.
_Avoid_: Done, finished, state

**Pending**:
A Todo that has been created but not yet completed.
_Avoid_: Open, active, incomplete

**Completed**:
A Todo that has been finished. Can be reopened back to Pending.
_Avoid_: Done, finished, checked

## Auth Language

**User**:
A person's identity in the system, identified by a unique email. Has profile
data (name, image) but holds no credentials directly — credentials live on
Accounts.
_Avoid_: Member, person, profile

**Account**:
An authentication method linked to a User. For email/password, the Account holds
the password hash with providerId "credential". For OAuth, it holds provider
tokens. A User can have multiple Accounts (e.g. credential + GitHub).
_Avoid_: Sign-in, credential (when referring to the row), profile

**Session**:
An active sign-in, represented by an opaque token stored in an httpOnly cookie.
Linked to a User via their ID. Has an expiry and optional device metadata (IP,
user agent). Revocable by deleting the row.
_Avoid_: Token (when referring to the row), connection

**ProviderId**:
The authentication provider for an Account — "credential" for email/password,
or an OAuth provider name (e.g. "github", "google").
_Avoid_: Provider, auth type, method

**Email**:
A User's unique email address. Used as the sign-in identifier and uniqueness
constraint.
_Avoid_: Username, sign-in id

## Architecture Language

**Command**:
An intent to change state. Routed through a command handler that loads the
domain aggregate, applies a state transition, and persists the result. Returns
an ID or confirmation, never domain data.
_Avoid_: Mutation, write operation

**Query**:
A request for data. Routed through a query handler that reads from the
repository directly. Never mutates state.
_Avoid_: Read operation, getter

**Port**:
A trait defined by the application layer describing what it needs from the
outside world (e.g. `TodoRepository`). The domain and application layers depend
on ports, never on concrete adapters.
_Avoid_: Interface, contract, gateway

**Adapter**:
A concrete implementation of a port (e.g. `SqliteTodoRepository`). Lives in the
infrastructure layer. The application layer never knows which adapter is plugged
in.
_Avoid_: Implementation, repository (when referring to the concrete impl)

## Rust Naming Conventions

- **Entity**: singular `PascalCase` type in `domain/entities/<entity>.rs`,
  re-exported from `domain/mod.rs` (e.g. `Todo`, `User`).
- **Value object**: singular `PascalCase` type in `domain/values/<value>.rs`;
  identifiers use `*Id` (e.g. `Title`, `Email`, `TodoId`).
- **Mapper**: infrastructure `mappers/` module with stateless functions like
  `to_domain`; add a struct only when state is needed.
- **DTO**: application DTOs use `*Command` / `*Query`; HTTP DTOs use
  `*Request` / `*Response`.
- **Repository**: port trait is `*Repository`; concrete adapter is
  `<Backend>*Repository` (e.g. `PostgresTodoRepository`).
- **Port**: trait in `application/ports`, no `I` prefix; use capability names
  for cross-feature ports (e.g. `UserPort`).
