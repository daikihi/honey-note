use common_type::models::flowers::Flower;
use sqlx::{Pool, Sqlite};

use crate::use_case::get_all_flowers::{self, get_all_flowers_dto::GetAllFlowersRequestDto};

pub fn get_all_flowers_request_adapter() -> Result<GetAllFlowersRequestDto, actix_web::Error> {
    // This function will handle the request to get all flowers
    // It will call the repository method to fetch flowers from the database
    // and return the response in the appropriate format.

    // Placeholder for actual implementation
    let path: &str = common::infrastructure::db::sqlx::DB_FILE_NAME;
    let pool: Pool<Sqlite> = common::infrastructure::db::sqlx::get_sqlite_pool(path.to_string());
    Ok(GetAllFlowersRequestDto { pool })
}

pub fn get_all_flowers_response_adapter(
    response: Result<
        get_all_flowers::get_all_flowers_dto::GetAllFlowersResponseDto,
        common::errors::AppError,
    >,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    match response {
        Ok(flowers) => {
            log::info!("Successfully fetched flowers: {:?}", flowers.flowers);
            Ok(actix_web::HttpResponse::Ok().json(flowers.flowers))
        }
        Err(e) => {
            log::error!("Error fetching flowers: {}", e);
            Err(actix_web::error::ErrorInternalServerError(e))
        }
    }
}
