use common_type::models::flowers::Flower;

use crate::use_case::get_all_flowers::get_all_flowers_dto::GetAllFlowersRequestDto;

pub fn get_all_flowers_request_adapter() -> Result<GetAllFlowersRequestDto, actix_web::Error> {
    // This function will handle the request to get all flowers
    // It will call the repository method to fetch flowers from the database
    // and return the response in the appropriate format.

    // Placeholder for actual implementation
    let path = common::infrastructure::db::sqlx::DB_FILE_NAME;
    let pool = common::infrastructure::db::sqlx::get_sqlite_pool(path.to_string());
    Ok(GetAllFlowersRequestDto { pool })
}

pub fn get_all_flowers_response_adapter(
    response: Result<Vec<Flower>, common::errors::AppError>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    match response {
        Ok(flowers) => {
            log::info!("Successfully fetched flowers: {:?}", flowers);
            Ok(actix_web::HttpResponse::Ok().json(flowers))
        }
        Err(e) => {
            log::error!("Error fetching flowers: {}", e);
            Err(actix_web::error::ErrorInternalServerError(e))
        }
    }
}
