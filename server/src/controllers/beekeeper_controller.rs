use actix_web::get;
use common::errors::AppError;

#[get("/honey-note/api/beekeepers")]
pub async fn get_all_beekeepers() -> Result<actix_web::HttpResponse, actix_web::Error> {
    use crate::use_case::get_all_beekeepers::get_all_beekeepers_dto;
    use crate::use_case::get_all_beekeepers;
    use crate::controllers::adapters::beekeepers::get_all_beekeepers_adapter::get_all_beekeepers_response_adapter;
    
    let dto = get_all_beekeepers_dto::GetAllBeekeepersRequestDto {};
    let results: Result<get_all_beekeepers_dto::GetAllBeekeepersResponseDto, AppError> = get_all_beekeepers::run(dto).await;
    let response: Result<actix_web::HttpResponse, actix_web::Error> = get_all_beekeepers_response_adapter(results);
    response
}
