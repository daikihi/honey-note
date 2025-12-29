pub mod test_batch{
    use sqlx::SqlitePool;

    pub async fn setup_test_db(db_url: &str) -> SqlitePool {
        use sqlx::sqlite::SqlitePoolOptions;

        let pool = SqlitePoolOptions::new()
            .min_connections(1)
            .connect(db_url)
            .await
            .expect("Failed to connect to SQLite");

        // Run migrations
        sqlx::migrate!("../resources/db/migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        pool
    }
}