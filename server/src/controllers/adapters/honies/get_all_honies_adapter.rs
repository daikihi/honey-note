use crate::use_case::get_all_honies::get_all_honies_dto::{
    GetAllHoneysRequestDto, GetAllHoneysResponseDto,
};
use common_type::models::honey::Honey;
pub fn get_all_honeis_request_adapter() -> Result<GetAllHoneysRequestDto, actix_web::Error> {
    let path = common::infrastructure::db::sqlx::DB_FILE_NAME;
    let pool = common::infrastructure::db::sqlx::get_sqlite_pool(path.to_string());
    Ok(GetAllHoneysRequestDto { pool })
}

pub fn get_all_honies_response_adapter(
    dto: GetAllHoneysResponseDto,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let honeys: Vec<Honey> = dto.honeys;
    Ok(actix_web::HttpResponse::Ok().json(honeys))
}
