use actix_web::{get, put, web, HttpResponse, Error, HttpRequest};
use common::repository::flowers::FlowerRepositorySqlite;
use crate::{
    use_case::get_all_flowers as get_all_flowers_use_case,
    use_case::put_new_flower as put_new_flower_use_case,
    use_case::put_edit_flower as put_edit_flower_use_case,
    use_case::get_flower_by_id as get_flower_by_id_use_case,
    middleware::LoggedJson,
};
use common_type::models::flowers::Flower;

#[get("/honey-note/api/flowers")]
pub async fn get_all_flowers(
    pool: web::Data<sqlx::SqlitePool>,
) -> Result<HttpResponse, Error> {
    let repo = FlowerRepositorySqlite {
        pool: pool.get_ref().clone(),
    };
    let results = get_all_flowers_use_case::run(
        &repo,
        get_all_flowers_use_case::get_all_flowers_dto::GetAllFlowersRequestDto {},
    ).await;
    match results {
        Ok(res) => Ok(HttpResponse::Ok().json(res.flowers)),
        Err(e) => {
            log::error!("Error fetching flowers: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[get("/honey-note/api/flower/{id}")]
pub async fn get_flower_by_id(
    path: web::Path<i32>,
    pool: web::Data<sqlx::SqlitePool>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let repo = FlowerRepositorySqlite { pool: pool.get_ref().clone() };
    let dto = get_flower_by_id_use_case::get_flower_by_id_dto::GetFlowerByIdRequestDto { id };
    let result = get_flower_by_id_use_case::run(&repo, dto).await;

    if result.success {
        if let Some(f) = result.flower {
            Ok(HttpResponse::Ok().json(f))
        } else {
            Ok(HttpResponse::NotFound().json(result))
        }
    } else {
        Ok(HttpResponse::NotFound().json(result))
    }
}

#[put("/honey-note/api/flower/new")]
pub async fn put_new_flower(
    _req: HttpRequest,
    payload: LoggedJson<Flower>,
    pool: web::Data<sqlx::SqlitePool>,
) -> Result<HttpResponse, Error> {
    let dto = put_new_flower_use_case::put_new_flower_dto::PutNewFlowerRequestDto { flower: payload.into_inner() };
    let repo = FlowerRepositorySqlite { pool: pool.get_ref().clone() };
    let result = put_new_flower_use_case::run(&repo, dto, pool.get_ref()).await;
    if result.success {
        Ok(HttpResponse::Ok().json(result))
    } else {
        log::error!("put_new_flower failed: {:?}", result.error_message);
        Ok(HttpResponse::BadRequest().json(result))
    }
}

#[put("/honey-note/api/flower/edit/{id}")]
pub async fn put_edit_flower(
    path: web::Path<i32>,
    _req: HttpRequest,
    payload: LoggedJson<Flower>,
    pool: web::Data<sqlx::SqlitePool>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let dto = put_edit_flower_use_case::put_edit_flower_dto::PutEditFlowerRequestDto { 
        id, 
        flower: payload.into_inner() 
    };
    let repo = FlowerRepositorySqlite { pool: pool.get_ref().clone() };
    let result = put_edit_flower_use_case::run(&repo, dto, pool.get_ref()).await;
    if result.success {
        Ok(HttpResponse::Ok().json(result))
    } else {
        log::error!("put_edit_flower failed: {:?}", result.error_message);
        Ok(HttpResponse::BadRequest().json(result))
    }
}
