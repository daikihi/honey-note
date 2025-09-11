use actix_web::{get, web};

use crate::{
    controllers::adapters::honies::{
        get_all_honies_adapter::{get_all_honeis_request_adapter, get_all_honies_response_adapter},
        get_honey_details_adapter::get_honey_details_request_adapter,
    },
    use_case::{
        get_all_honies::get_all_honies_dto::{GetAllHoneysRequestDto, GetAllHoneysResponseDto},
        get_honey_details::get_honey_details_dto::GetHoneyDetailsRequestDto,
    },
};

use crate::controllers::adapters::honies::get_honey_details_adapter::get_honey_details_response_adapter;

#[get("/honey-note/api/honeys")]
pub async fn get_all_honeys() -> Result<actix_web::HttpResponse, actix_web::Error> {
    use crate::use_case::get_all_honies;
    let request_dto: Result<GetAllHoneysRequestDto, actix_web::Error> =
        get_all_honeis_request_adapter();
    match request_dto {
        Ok(res) => {
            let use_case_result: GetAllHoneysResponseDto = get_all_honies::run(res).await;
            log::info!(
                "size of response is {}",
                use_case_result.honeys.iter().len()
            );
            get_all_honies_response_adapter(use_case_result)
        }
        Err(e) => Err(actix_web::error::ErrorInternalServerError("Fix here")),
    }
}

#[get("/honey-note/api/honeys/{honey_id}")]
pub async fn get_honey_details(
    path: web::Path<i32>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let honey_id = path.into_inner();
    log::info!("get_honey_details called {}", honey_id);
    let request_dto: Result<GetHoneyDetailsRequestDto, actix_web::Error> =
        get_honey_details_request_adapter(honey_id);
    let use_case_result = match request_dto {
        Ok(r) => crate::use_case::get_honey_details::run(r).await,
        Err(e) => {
            log::error!("error occurred in adapter: {:?}", e);
            return Err(actix_web::error::ErrorInternalServerError("Fix here"));
        }
    };
    get_honey_details_response_adapter(use_case_result)
}
