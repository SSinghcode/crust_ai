CREATE TABLE IF NOT EXISTS messages (
    unid       UUID                  PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    chat_unid  UUID                  NOT NULL REFERENCES chats(unid) ON DELETE CASCADE,
    role       CHARACTER VARYING(20) NOT NULL CHECK (role IN ('user', 'assistant')),
    content    TEXT                  NOT NULL,
    created_at TIMESTAMPTZ           NOT NULL DEFAULT now()
);


-- Seed data for the first two chats
INSERT INTO messages (unid, chat_unid, role, content) VALUES
(
    '20000000-0000-0000-0000-000000000001',
    '00000000-0000-0000-0000-000000000001',
    'user',
    'How do React hooks like useState and useEffect actually work under the hood?'
),
(
    '20000000-0000-0000-0000-000000000002',
    '00000000-0000-0000-0000-000000000001',
    'assistant',
    'React hooks are special functions that let you hook into React state and lifecycle features from function components.'
),
(
    '20000000-0000-0000-0000-000000000003',
    '00000000-0000-0000-0000-000000000001',
    'user',
    'Can you show me an example with useState?'
),
(
    '20000000-0000-0000-0000-000000000004',
    '00000000-0000-0000-0000-000000000001',
    'assistant',
    'Sure! Here is a simple counter component using useState and useEffect.'
),
(
    '20000000-0000-0000-0000-000000000005',
    '00000000-0000-0000-0000-000000000002',
    'user',
    'Why does the borrow checker complain about returning a reference from a function?'
),
(
    '20000000-0000-0000-0000-000000000006',
    '00000000-0000-0000-0000-000000000002',
    'assistant',
    'Rust lifetimes ensure that references never outlive the data they point to. When returning a reference, you must tell Rust how long that reference is valid.'
);
