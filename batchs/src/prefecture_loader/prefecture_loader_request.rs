pub struct PrefectureLoaderRequestDto<'a> {
    pub file_name: String,
    pub pool: &'a sqlx::SqlitePool,
}
