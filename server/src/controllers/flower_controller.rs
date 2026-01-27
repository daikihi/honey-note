use actix_web::get;

#[get("/honey-note/api/flowers")]
pub async fn get_all_flowers(
    pool: actix_web::web::Data<sqlx::SqlitePool>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    use crate::use_case::get_all_flowers as get_all_flowers_use_case;
    use common::repository::flowers::FlowerRepositorySqlite;

    let repo = FlowerRepositorySqlite {
        pool: pool.get_ref().clone(),
    };
    let results = get_all_flowers_use_case::run(
        &repo,
        get_all_flowers_use_case::get_all_flowers_dto::GetAllFlowersRequestDto {},
    ).await;
    match results {
        Ok(res) => Ok(actix_web::HttpResponse::Ok().json(res.flowers)),
        Err(e) => {
            log::error!("Error fetching flowers: {}", e);
            Ok(actix_web::HttpResponse::InternalServerError().finish())
        }
    }
}
