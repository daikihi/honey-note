pub mod beekeeper;
pub mod flower;
pub mod honey;
pub mod prefecture;

use once_cell::sync::OnceCell;
use sqlx::SqlitePool;

static POOL: OnceCell<SqlitePool> = OnceCell::new();

pub fn get_sqlite_pool(path: String) -> sqlx::SqlitePool {
    POOL.get_or_init(|| {
        SqlitePool::connect_lazy(path.as_str()).expect("Failed to create SQLite pool")
    })
    .clone()
}

pub static DB_FILE_NAME: &str = "resources/db/honey_note.db";
