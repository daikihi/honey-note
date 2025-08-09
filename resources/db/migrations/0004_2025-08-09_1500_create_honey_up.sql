PRAGMA foreign_keys = ON;

CREATE TABLE honey (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    name_jp         TEXT NOT NULL,                          -- 蜂蜜の日本語名（例: アカシアはちみつ）
    name_en         TEXT,                                   -- 英語名（例: Acacia Honey）
    beekeeper_id    INTEGER,                                -- 生産者（外部キー）
    origin_country  TEXT,                                   -- 原産国
    origin_region   TEXT,                                   -- 原産地域（都道府県、州など）
    harvest_year    INTEGER,                                -- 採蜜年（購入日と別）
    purchase_date   DATE,                                   -- 購入日
    note            TEXT,                                   -- 補足
    FOREIGN KEY (beekeeper_id) REFERENCES beekeeper(id)
);


CREATE TABLE honey_flower (
    honey_id    INTEGER NOT NULL,
    flower_id   INTEGER NOT NULL,
    PRIMARY KEY (honey_id, flower_id),
    FOREIGN KEY (honey_id) REFERENCES honey(id),
    FOREIGN KEY (flower_id) REFERENCES flower(id)
);


CREATE TABLE honey_batch_info (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    honey_id    INTEGER NOT NULL,
    measured_at DATE NOT NULL,               -- 測定日
    color       TEXT,                         -- 色
    taste       TEXT,                         -- 味のメモ
    crystallization_state TEXT,               -- 結晶化の状態
    moisture    REAL,                         -- 水分量 %
    FOREIGN KEY (honey_id) REFERENCES honey(id)
);

