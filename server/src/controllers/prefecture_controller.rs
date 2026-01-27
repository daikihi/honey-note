use actix_web::{error, get, web};
use common_type::models::prefectures::Prefecture as PrefectureModel;
use common::repository::prefectures::PrefectureRepositorySqlite;
use crate::use_case::get_all_prefectures as get_all_prefectures_use_case;

#[get("/honey-note/api/prefectures")]
pub async fn get_all_prefectures(
    pool: web::Data<sqlx::SqlitePool>,
) -> actix_web::Result<actix_web::web::Json<Vec<PrefectureModel>>> {
    let repo = PrefectureRepositorySqlite {
        pool: pool.get_ref().clone(),
    };
    match get_all_prefectures_use_case::run(&repo).await {
        Ok(_prefectures) => Ok(web::Json(_prefectures)),
        Err(e) => {
            log::error!("Failed to fetch prefectures: {}", e);
            Err(error::ErrorInternalServerError("Database error"))
        }
    }
}
