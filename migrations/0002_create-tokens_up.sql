CREATE TABLE Tokens (
    id uuid PRIMARY KEY,
    key VARCHAR UNIQUE NOT NULL,
    user_id uuid NOT NULL,
    expire_at TIMESTAMP NOT NULL,
    is_revoked BOOLEAN NOT NULL DEFAULT true
)