CREATE TABLE accounts
(
    id              BIGINT PRIMARY KEY NOT NULL,
    did             VARCHAR(255) UNIQUE,
    avatar          VARCHAR(255)                DEFAULT '',
    banner          VARCHAR(255)                DEFAULT '',
    followers_count INTEGER                     DEFAULT 0,
    follows_count   INTEGER                     DEFAULT 0,
    posts_count     INTEGER                     DEFAULT 0,
    description     VARCHAR(255)                DEFAULT '',
    created_at      TIMESTAMPTZ        NOT NULL DEFAULT CURRENT_TIMESTAMP, -- When the account was first indexed.
    updated_at      TIMESTAMPTZ        NOT NULL DEFAULT CURRENT_TIMESTAMP  -- When the account was last updated.
);

CREATE TABLE posts
(
    id         BIGINT PRIMARY KEY  NOT NULL,
    cid        VARCHAR(255) UNIQUE NOT NULL,
    uri        VARCHAR(8000)       NOT NULL,
    author_did VARCHAR(128)        NOT NULL,
    created_at TIMESTAMPTZ         NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ         NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE reposts
(
    id               BIGINT PRIMARY KEY NOT NULL,
    hash_id          VARCHAR(64) UNIQUE NOT NULL,
    original_post_id BIGINT             NOT NULL REFERENCES posts (id) ON DELETE CASCADE,
    created_at       TIMESTAMPTZ        NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at       TIMESTAMPTZ        NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE notifications
(
    id         BIGINT PRIMARY KEY NOT NULL,
    cid        VARCHAR(255)       NOT NULL,
    uri        VARCHAR(8000)      NOT NULL,
    created_at TIMESTAMPTZ        NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ        NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE media
(
    id          BIGINT PRIMARY KEY NOT NULL,
    type        VARCHAR(255)       NOT NULL,
    mime_type   VARCHAR(255)       NOT NULL,
    size        INTEGER            NOT NULL,
    link        VARCHAR(8000)      NOT NULL,
    description VARCHAR(255)                DEFAULT '',
    created_at  TIMESTAMPTZ        NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMPTZ        NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE sessions
(
    id          INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,       -- Unique ID of the session.
    did         VARCHAR(128) UNIQUE NOT NULL,                           -- The unique DID of the user.
    handle      VARCHAR(255)        NOT NULL,                           -- The @ handle of the user.
    access_jwt  TEXT                NOT NULL,                           -- The access token.
    refresh_jwt TEXT                NOT NULL,                           -- The refresh token.
    created_at  TIMESTAMPTZ         NOT NULL DEFAULT CURRENT_TIMESTAMP, -- When the session was created.
    updated_at  TIMESTAMPTZ         NOT NULL DEFAULT CURRENT_TIMESTAMP  -- When the session was last updated.
);

CREATE INDEX idx_posts_cid ON posts (cid);
CREATE INDEX idx_accounts_did ON accounts (did);
