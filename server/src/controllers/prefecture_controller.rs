use actix_web::{error, get, web};
use common::models::prefectures::Prefecture as PrefectureModel;
use common::repository::prefectures::get_all_prefectures as repository_get_all_prefectures;

#[get("/prefectures")]
pub async fn get_all_prefectures() -> actix_web::Result<actix_web::web::Json<Vec<PrefectureModel>>>
{
    // should move to parameter of program or somethins static place
    let file_name = "resources/db/honey_note.db";
    let pool = common::infrastructure::db::sqlx::get_sqlite_pool(file_name.to_string());
    match repository_get_all_prefectures(&pool).await {
        Ok(_prefectures) => Ok(web::Json(_prefectures)),
        Err(e) => {
            log::error!("Failed to fetch prefectures: {}", e);
            Err(error::ErrorInternalServerError("Database error"))
        }
    }
}
