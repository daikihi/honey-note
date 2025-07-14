use crate::use_case::get_all_beekeepers::get_all_beekeepers_dto;

pub fn get_all_beekeepers_request_adapter() -> get_all_beekeepers_dto::GetAllBeekeepersRequestDto {
    get_all_beekeepers_dto::GetAllBeekeepersRequestDto {}
}
pub fn get_all_beekeepers_response_adapter(response: Result<get_all_beekeepers_dto::GetAllBeekeepersResponseDto, common::errors::AppError>) -> Result<actix_web::HttpResponse, actix_web::Error> {
    match response {
        Ok(dto) => {
            log::info!("Successfully fetched beekeepers: {:?}", dto.beekeepers);
            Ok(actix_web::HttpResponse::Ok().json(dto.beekeepers))
        }
        Err(e) => {
            log::error!("Error fetching beekeepers: {}", e);
            Err(actix_web::error::ErrorInternalServerError(e))
        }
    }
}
