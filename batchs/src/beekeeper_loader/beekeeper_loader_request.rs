pub struct BeekeeperLoaderRequestDto<'a> {
    pub file_name: &'a str,
    pub pool: sqlx::SqlitePool,
}

impl<'a> BeekeeperLoaderRequestDto<'a> {
    pub fn new(file_name: &'a str, pool: sqlx::SqlitePool) -> Self {
        BeekeeperLoaderRequestDto {
            file_name,
            pool,
        }
    }
}
