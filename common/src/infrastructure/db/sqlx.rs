pub mod prefecture;

pub async fn create_sqlite_pool()-> sqlx::SqlitePool {
    let database_url = "sqlite://./resources/db/honey_note.db";
    sqlx::SqlitePool::connect(database_url)
        .await
        .expect("Failed to create SQLite pool")
}