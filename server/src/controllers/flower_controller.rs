use actix_web::get;
use common::errors::AppError;

use crate::controllers::adapters::flowers::get_all_flowers_adapter::{
    get_all_flowers_request_adapter, get_all_flowers_response_adapter,
};

#[get("/honey-note/api/flowers")]
pub async fn get_all_flowers() -> Result<actix_web::HttpResponse, actix_web::Error> {
    use crate::use_case::get_all_flowers;
    use get_all_flowers::get_all_flowers_dto::GetAllFlowersRequestDto;

    let req: Result<GetAllFlowersRequestDto, actix_web::Error> = get_all_flowers_request_adapter();

    match req {
        Ok(_req) => {
            let flowers: Result<
                get_all_flowers::get_all_flowers_dto::GetAllFlowersResponseDto,
                AppError,
            > = get_all_flowers::run(_req).await;
            get_all_flowers_response_adapter(flowers)
        }
        Err(e) => {
            log::error!("Error creating request adapter: {}", e);
            return Err(actix_web::error::ErrorInternalServerError(e));
        }
    }
}
