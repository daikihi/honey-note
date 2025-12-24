#[cfg(test)]
mod test_beekeeper_load{
    use super::*;

    #[tokio::test]
    async fn testmain(){
        let memery_sqlx_test = "sqlite::memory:";
        let pool = SqlitePool::connect(memery_sqlx_test).await.unwrap();
        sqlx::migrate!("./resources/db").run(&pool).await.unwrap();
    }
}
