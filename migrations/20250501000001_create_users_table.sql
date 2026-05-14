CREATE TABLE users (
    unid                    UUID         PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    email                   VARCHAR(255) UNIQUE NOT NULL,
    username                VARCHAR(100) UNIQUE NOT NULL,
    first_name              VARCHAR(100),
    last_name               VARCHAR(100),
    password_hash           TEXT         NOT NULL,
    is_active               BOOLEAN      NOT NULL DEFAULT true,
    role                    VARCHAR(50)  NOT NULL DEFAULT 'user',
    created_at              TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at              TIMESTAMPTZ  NOT NULL DEFAULT now(),
    last_login_at           TIMESTAMPTZ,
    last_password_change_at TIMESTAMPTZ,
    failed_login_attempts   INT          NOT NULL DEFAULT 0,
    locked_until            TIMESTAMPTZ
);

-- Fast login lookups by email
CREATE INDEX idx_users_email ON users (email);

-- Seed user required by chats/documents seed data
INSERT INTO users (unid, email, username, password_hash) VALUES
('10000000-0000-0000-0000-000000000001', 'shamsher@example.com', 'Shamsher', 'placeholder');
