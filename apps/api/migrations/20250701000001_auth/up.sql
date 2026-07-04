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
    id                        UUID PRIMARY KEY NOT NULL,
    user_id                   UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    account_id                TEXT NOT NULL,
    provider_id               TEXT NOT NULL,
    access_token              TEXT,
    refresh_token             TEXT,
    id_token                  TEXT,
    access_token_expires_at   TIMESTAMPTZ,
    refresh_token_expires_at  TIMESTAMPTZ,
    scope                     TEXT,
    password                  TEXT,
    created_at                TIMESTAMPTZ NOT NULL,
    updated_at                TIMESTAMPTZ NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_accounts_provider ON accounts(provider_id, account_id);
CREATE INDEX IF NOT EXISTS idx_accounts_user_provider ON accounts(user_id, provider_id);

CREATE TABLE IF NOT EXISTS sessions (
    id          TEXT PRIMARY KEY NOT NULL,
    token       TEXT NOT NULL UNIQUE,
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    expires_at  TIMESTAMPTZ NOT NULL,
    ip_address  TEXT,
    user_agent  TEXT,
    created_at  TIMESTAMPTZ NOT NULL,
    updated_at  TIMESTAMPTZ NOT NULL
);

ALTER TABLE todos ADD COLUMN user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE;
