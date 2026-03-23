-- Phase 1: User base and DB configuration

-- 1. Create users table
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,                        -- ログインID（lowercase に正規化して保存）
    email_hash TEXT NOT NULL,                      -- ハッシュ化したメールアドレス（比較専用）
    password_hash TEXT NOT NULL,                   -- ハッシュ化パスワード
    display_name TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    terminated_at DATETIME,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(username),
    UNIQUE(email_hash)
);

-- 2. Create trigger for updated_at in users table
CREATE TRIGGER users_updated_at
AFTER UPDATE ON users
FOR EACH ROW
BEGIN
    UPDATE users SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;

-- 3. Add user_id to existing tables
-- Note: SQLite does not support adding a NOT NULL column without a default value to an existing table.
-- For now, we add it as nullable, and in Phase 4 we might enforce it or handle existing data.

ALTER TABLE honey ADD COLUMN user_id INTEGER;
ALTER TABLE beekeeper ADD COLUMN user_id INTEGER;
ALTER TABLE flower ADD COLUMN user_id INTEGER;

-- 4. Create indexes for user_id (optional but recommended for performance)
CREATE INDEX idx_honey_user_id ON honey(user_id);
CREATE INDEX idx_beekeeper_user_id ON beekeeper(user_id);
CREATE INDEX idx_flower_user_id ON flower(user_id);
