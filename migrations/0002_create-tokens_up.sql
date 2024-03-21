CREATE TABLE Tokens (
    id uuid PRIMARY KEY,
    key VARCHAR UNIQUE NOT NULL,
    user_id uuid UNIQUE NOT NULL,
    is_revoked BOOLEAN NOT NULL DEFAULT true
)