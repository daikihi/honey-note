pub mod beekeeper;
pub mod flower;
pub mod honey;
pub mod prefecture;

use once_cell::sync::OnceCell;
use sqlx::SqlitePool;

static POOL: OnceCell<SqlitePool> = OnceCell::new();

pub fn get_sqlite_pool(path: String) -> sqlx::SqlitePool {
    #[cfg(test)]
    {
        // For tests, we want each test to have its own database if a unique path is provided.
        // If "sqlite::memory:" is used, it's a private in-memory DB per connection.
        // We use connect_lazy here, but we should probably use connect if we want to ensure it's up.
        // However, the test caller usually handles this.
        SqlitePool::connect_lazy(path.as_str()).expect("Failed to create SQLite pool")
    }
    #[cfg(not(test))]
    {
        POOL.get_or_init(|| {
            SqlitePool::connect_lazy(path.as_str()).expect("Failed to create SQLite pool")
        })
        .clone()
    }
}

// todo should be switched by environment name by parameter
pub static DB_FILE_NAME: &str = "resources/db/honey_note.db";
