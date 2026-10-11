-- 既存データに空名・前後空白がある場合は移行を止め、手動で確認する。
CREATE TABLE beekeeper_name_migration_check (
    name_jp TEXT NOT NULL CHECK (
        name_jp <> '' AND name_jp = trim(name_jp, char(9,10,11,12,13,32,133,160,5760,8192,8193,8194,8195,8196,8197,8198,8199,8200,8201,8202,8232,8233,8239,8287,12288))
    )
);
INSERT INTO beekeeper_name_migration_check (name_jp)
SELECT name_jp FROM beekeeper;
DROP TABLE beekeeper_name_migration_check;

-- 同じユーザーに同名の養蜂場を重複登録しない。
CREATE UNIQUE INDEX idx_beekeeper_user_name ON beekeeper (user_id, name_jp);

-- DB へ直接書き込む経路でも空名・前後空白を残さない。
CREATE TRIGGER beekeeper_name_insert_check
BEFORE INSERT ON beekeeper
FOR EACH ROW
WHEN NEW.name_jp = '' OR NEW.name_jp <> trim(NEW.name_jp, char(9,10,11,12,13,32,133,160,5760,8192,8193,8194,8195,8196,8197,8198,8199,8200,8201,8202,8232,8233,8239,8287,12288))
BEGIN
    SELECT RAISE(ABORT, '養蜂場名は空にできず、前後に空白を含められません');
END;

CREATE TRIGGER beekeeper_name_update_check
BEFORE UPDATE OF name_jp ON beekeeper
FOR EACH ROW
WHEN NEW.name_jp = '' OR NEW.name_jp <> trim(NEW.name_jp, char(9,10,11,12,13,32,133,160,5760,8192,8193,8194,8195,8196,8197,8198,8199,8200,8201,8202,8232,8233,8239,8287,12288))
BEGIN
    SELECT RAISE(ABORT, '養蜂場名は空にできず、前後に空白を含められません');
END;
