use crate::use_case::get_honey_details::get_honey_details_dto::{
    GetHoneyDetailsRequestDto, GetHoneyDetailsResponseDto,
};

pub fn get_honey_details_request_adapter(
    honey_id: i32,
) -> Result<GetHoneyDetailsRequestDto, actix_web::Error> {
    let path = common::infrastructure::db::sqlx::DB_FILE_NAME;
    let pool = common::infrastructure::db::sqlx::get_sqlite_pool(path.to_string());
    Ok(GetHoneyDetailsRequestDto { honey_id, pool })
}

pub fn get_honey_details_response_adapter(
    adapter: GetHoneyDetailsResponseDto,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let honey_detail_opt = adapter.honey_detail;
    Ok(actix_web::HttpResponse::Ok().json(honey_detail_opt))
}
