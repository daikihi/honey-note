use actix_web::get;
use common::errors::AppError;

use crate::controllers::adapters::flowers::get_all_flowers_adapter::get_all_flowers_request_adapter;

#[get("/honey-note/api/flowers")]
pub async fn get_all_flowers() -> Result<actix_web::HttpResponse, actix_web::Error> {
    use crate::use_case::get_all_flowers;

    let req = get_all_flowers_request_adapter();

    match req {
        Ok(_req) => {
            let flowers: Result<
                get_all_flowers::get_all_flowers_dto::get_all_flowers_response_dto,
                AppError,
            > = get_all_flowers::run(_req).await;
            match flowers {
                Ok(_flowers) => Ok(actix_web::HttpResponse::Ok().json(_flowers.flowers)),
                Err(e) => {
                    log::error!("Error fetching flowers: {}", e);
                    Err(actix_web::error::ErrorInternalServerError(e))
                }
            }
        }
        Err(e) => {
            log::error!("Error creating request adapter: {}", e);
            return Err(actix_web::error::ErrorInternalServerError(e));
        }
    }
}
