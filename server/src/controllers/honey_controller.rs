use actix_web::{get, put, web, HttpResponse, Error, HttpRequest};
use web::Json;
use common::repository::honeys::HoneyRepositorySqlite;
use crate::{
    use_case::put_new_honey::put_new_honey_dto::{PutNewHoneyRequestDto, PutNewHoneyResponseDto},
    use_case::put_new_honey as put_new_honey_use_case,
};
use common_type::request::honey::edit::HoneyEditRequest;
use common_type::request::honey::new::HoneyNewRequest;

#[get("/honey-note/api/honeys")]
pub async fn get_all_honeys(
    pool: web::Data<sqlx::SqlitePool>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    use crate::use_case::get_all_honies as get_all_honies_use_case;
    let request_dto = get_all_honies_use_case::get_all_honies_dto::GetAllHoneysRequestDto {};
    let repo = HoneyRepositorySqlite {
        pool: pool.get_ref().clone(),
    };
    let use_case_result = get_all_honies_use_case::run(&repo, request_dto).await;
    log::info!("size of response is {}", use_case_result.honeys.iter().len());
    Ok(actix_web::HttpResponse::Ok().json(use_case_result.honeys))
}

// 新規作成APIフレーム
#[put("/honey-note/api/honey/new")]
pub async fn put_new_honey(
    _req: HttpRequest,
    payload: Json<HoneyNewRequest>,
    pool: web::Data<sqlx::SqlitePool>,
) -> Result<HttpResponse, Error> {
    // DTO変換
    let dto = PutNewHoneyRequestDto { new: payload.into_inner() };
    println!("request = {:?}", dto);
    // Repositoryの実装を生成
    let repo = HoneyRepositorySqlite { pool: pool.get_ref().clone() };
    // UseCase呼び出し
    let result: PutNewHoneyResponseDto = put_new_honey_use_case::run(&repo, dto).await;
    if result.success {
        println!("ok response = {:?}", result);
        Ok(HttpResponse::Ok().json(result))
    } else {
        println!("bad response = {:?}", result);
        Ok(HttpResponse::BadRequest().json(result))
    }
}

// 編集APIフレーム
#[put("/honey-note/api/honey/edit")]
pub async fn put_edit_honey(
    _req: HttpRequest,
    _payload: Json<HoneyEditRequest>,
) -> Result<HttpResponse, Error> {
    // TODO: 編集ロジックを実装
    Ok(HttpResponse::Ok().finish())
}
