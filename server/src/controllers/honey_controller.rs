use actix_web::{get, put, web, HttpResponse, Error, HttpRequest};
use web::Json;
use crate::{
    controllers::adapters::honies::get_all_honies_adapter::{
        get_all_honeis_request_adapter, get_all_honies_response_adapter,
    },
    use_case::get_all_honies::get_all_honies_dto::{
        GetAllHoneysRequestDto, GetAllHoneysResponseDto,
    },
};
use common_type::request::honey::edit::HoneyEditRequest;

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

// 新規作成APIフレーム
// #[put("/honey-note/api/honey/new")]
// pub async fn put_new_honey(
//     req: HttpRequest,
//     payload: Json<HoneyInputNewRequest>,
// ) -> Result<HttpResponse, Error> {
//     // TODO: 新規作成ロジックを実装
//     Ok(HttpResponse::Ok().finish())
// }

// 編集APIフレーム
#[put("/honey-note/api/honey/edit")]
pub async fn put_edit_honey(
    req: HttpRequest,
    payload: Json<HoneyEditRequest>,
) -> Result<HttpResponse, Error> {
    // TODO: 編集ロジックを実装
    Ok(HttpResponse::Ok().finish())
}
