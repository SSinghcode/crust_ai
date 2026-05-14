CREATE TABLE IF NOT EXISTS chats (
    unid       UUID                   PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    user_unid  UUID                   NOT NULL REFERENCES users(unid) ON DELETE CASCADE,
    title      CHARACTER VARYING(255) NOT NULL DEFAULT 'New Chat',
    mode       PascalCase  NOT NULL DEFAULT 'Developer' CHECK CONSTRAINT ('Developer', 'General'), 
    created_at TIMESTAMPTZ            NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ            NOT NULL DEFAULT now()
);


-- Seed data — mirrors public/chats.json
INSERT INTO chats (unid, user_unid, title, mode) VALUES
('00000000-0000-0000-0000-000000000001', '10000000-0000-0000-0000-000000000001', 'React hooks best practices',      'developer'),
('00000000-0000-0000-0000-000000000002', '10000000-0000-0000-0000-000000000001', 'Rust lifetime errors',             'developer'),
('00000000-0000-0000-0000-000000000003', '10000000-0000-0000-0000-000000000001', 'Setting up PostgreSQL with SQLx',  'developer'),
('00000000-0000-0000-0000-000000000004', '10000000-0000-0000-0000-000000000001', 'Leptos SSR vs CSR',                'general'),
('00000000-0000-0000-0000-000000000005', '10000000-0000-0000-0000-000000000001', 'Tailwind CSS dark mode setup',     'developer'),
('00000000-0000-0000-0000-000000000006', '10000000-0000-0000-0000-000000000001', 'Async Rust with Tokio',            'developer');
