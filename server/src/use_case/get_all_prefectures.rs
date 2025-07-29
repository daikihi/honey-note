use common::{
    errors::AppError,
    repository::prefectures::get_all_prefectures as repository_get_all_prefectures,
};
use sqlx::pool;

use common_type::models::prefectures::Prefecture;
pub async fn run(
    pool: &pool::Pool<sqlx::Sqlite>,
) -> Result<Vec<Prefecture>, AppError> {
    match repository_get_all_prefectures(pool).await {
        Ok(prefectures) => Ok(prefectures),
        Err(e) => {
            log::error!("Failed to fetch prefectures: {}", e);
            Err(e)
        }
    }
}
