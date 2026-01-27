use actix_web::get;

#[get("/honey-note/api/beekeepers")]
pub async fn get_all_beekeepers(
    pool: actix_web::web::Data<sqlx::SqlitePool>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    use crate::use_case::get_all_beekeepers as get_all_beekeepers_use_case;
    use crate::use_case::get_all_beekeepers::get_all_beekeepers_dto;
    use common::repository::beekeepers::BeekeeperRepositorySqlite;

    let dto = get_all_beekeepers_dto::GetAllBeekeepersRequestDto {};
    let repo = BeekeeperRepositorySqlite {
        pool: pool.get_ref().clone(),
    };
    let results = get_all_beekeepers_use_case::run(&repo, dto).await;
    match results {
        Ok(res) => Ok(actix_web::HttpResponse::Ok().json(res.beekeepers)),
        Err(e) => {
            log::error!("Error fetching beekeepers: {}", e);
            Ok(actix_web::HttpResponse::InternalServerError().finish())
        }
    }
}
