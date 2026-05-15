-- Stores uploaded files (txt, pdf, etc.) that users attach to chats.
-- content holds the extracted text; raw binary blobs go to object storage.
CREATE TYPE FileType AS ENUM ('Pdf', 'Txt', 'Docx', 'Csv', 'Md');

CREATE TABLE IF NOT EXISTS documents (
    unid        UUID                   PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    user_unid   UUID                   NOT NULL REFERENCES users(unid) ON DELETE CASCADE,
    chat_unid   UUID                   REFERENCES chats(unid) ON DELETE SET NULL,
    filename    CHARACTER VARYING(255) NOT NULL,
    file_type   FileType               NOT NULL,
    content     TEXT,
    size_bytes  BIGINT,
    created_at  TIMESTAMPTZ            NOT NULL DEFAULT now()
);
