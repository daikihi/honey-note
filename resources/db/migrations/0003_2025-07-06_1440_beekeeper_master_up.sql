PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS beekeeper (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name_jp TEXT NOT NULL,
    name_en TEXT,
    founding_year INTEGER,
    location_prefecture_id INTEGER,
    location_city TEXT,
    website_url TEXT,
    note TEXT,
    FOREIGN KEY(location_prefecture_id) REFERENCES prefecture(id) ON DELETE SET NULL
);