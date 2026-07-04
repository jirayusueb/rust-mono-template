# Auth: better-auth model, opaque tokens, argon2

We chose the better-auth schema model (User 1→many Accounts, User 1→many Sessions) over a simple User-only table. This separates identity (User) from authentication method (Account), making OAuth providers a drop-in later — just add a new Account row with providerId and token fields. The OAuth token columns (access_token, refresh_token, etc.) are included in the accounts table now, nullable for credential accounts, so no schema migration is needed when OAuth is added.

Sessions use opaque tokens (UUID v4) stored in an httpOnly cookie, validated via DB lookup on each request. This was chosen over JWT for simplicity and revocation — deleting a session row immediately invalidates the token, which JWT cannot do without a revocation list. JWT's stateless advantage is irrelevant for a single-server SQLite app.

Passwords are hashed with argon2 (OWASP-recommended, PHC format) over bcrypt. The PasswordHasher trait lives in shared/application since it's a cross-cutting concern, not auth-specific — a future password-change feature would reuse it.
