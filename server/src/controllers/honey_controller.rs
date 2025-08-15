use actix_web::get;

use crate::{
    controllers::adapters::honies::get_all_honies_adapter::{
        get_all_honeis_request_adapter, get_all_honies_response_adapter,
    },
    use_case::get_all_honies::get_all_honies_dto::{
        GetAllHoneysRequestDto, GetAllHoneysResponseDto,
    },
};

#[get("/honey-note/api/honeys")]
pub async fn get_all_honeys() -> Result<actix_web::HttpResponse, actix_web::Error> {
    use crate::use_case::get_all_honies;
    let request_dto: Result<GetAllHoneysRequestDto, actix_web::Error> =
        get_all_honeis_request_adapter();
    match request_dto {
        Ok(res) => {
            let use_case_result: GetAllHoneysResponseDto = get_all_honies::run(res).await;
            log::info!("size of response is {}", use_case_result.honeys.iter().len());
            get_all_honies_response_adapter(use_case_result)
        }
        Err(e) => Err(actix_web::error::ErrorInternalServerError("Fix here")),
    }
}
