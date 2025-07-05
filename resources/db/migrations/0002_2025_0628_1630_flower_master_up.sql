PRAGMA foreign_keys = ON;

-- メイン：花マスタ
-- SQLite においては、Text と Varchar は同じ扱いなのでText型を使用
CREATE TABLE IF NOT EXISTS flower (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name_jp TEXT NOT NULL,
    name_en TEXT,
    scientific_name TEXT,
    short_note TEXT,       -- description に相当する簡単な説明
    flower_type TEXT,
    image_path TEXT,
    note TEXT
);

-- 別名（alias） N:1 flower_alias -> flower
CREATE TABLE IF NOT EXISTS flower_alias (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    flower_id INTEGER NOT NULL,
    alias TEXT NOT NULL,
    FOREIGN KEY(flower_id) REFERENCES flower(id) ON DELETE CASCADE
);

-- 色（color） N:1 flower_color -> flower
CREATE TABLE IF NOT EXISTS flower_color (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    flower_id INTEGER NOT NULL,
    color TEXT NOT NULL,
    FOREIGN KEY(flower_id) REFERENCES flower(id) ON DELETE CASCADE
);

-- 原産地域・分布（origin region） N:1 flower_origin_region -> flower
CREATE TABLE IF NOT EXISTS flower_origin_region (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    flower_id INTEGER NOT NULL,
    region TEXT NOT NULL,
    FOREIGN KEY(flower_id) REFERENCES flower(id) ON DELETE CASCADE
);

-- 開花月（bloom months）N:1 flower_bloom_month -> flower
CREATE TABLE IF NOT EXISTS flower_bloom_month (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    flower_id INTEGER NOT NULL,
    month INTEGER NOT NULL CHECK (month BETWEEN 1 AND 12),
    FOREIGN KEY(flower_id) REFERENCES flower(id) ON DELETE CASCADE
);
