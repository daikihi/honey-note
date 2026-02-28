use actix_web::{get, put, web, HttpResponse, Error, HttpRequest};
use common::repository::beekeepers::BeekeeperRepositorySqlite;
use crate::{
    use_case::get_all_beekeepers as get_all_beekeepers_use_case,
    use_case::get_all_beekeepers::get_all_beekeepers_dto,
    use_case::put_new_beekeeper as put_new_beekeeper_use_case,
    use_case::put_edit_beekeeper as put_edit_beekeeper_use_case,
    use_case::get_beekeeper_by_id as get_beekeeper_by_id_use_case,
    middleware::LoggedJson,
};
use common_type::models::beekeeper::Beekeeper;

#[get("/honey-note/api/beekeepers")]
pub async fn get_all_beekeepers(
    pool: web::Data<sqlx::SqlitePool>,
) -> Result<HttpResponse, Error> {
    let dto = get_all_beekeepers_dto::GetAllBeekeepersRequestDto {};
    let repo = BeekeeperRepositorySqlite {
        pool: pool.get_ref().clone(),
    };
    let results = get_all_beekeepers_use_case::run(&repo, dto).await;
    match results {
        Ok(res) => Ok(HttpResponse::Ok().json(res.beekeepers)),
        Err(e) => {
            log::error!("Error fetching beekeepers: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[get("/honey-note/api/beekeeper/{id}")]
pub async fn get_beekeeper_by_id(
    path: web::Path<i32>,
    pool: web::Data<sqlx::SqlitePool>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let repo = BeekeeperRepositorySqlite { pool: pool.get_ref().clone() };
    let dto = get_beekeeper_by_id_use_case::get_beekeeper_by_id_dto::GetBeekeeperByIdRequestDto { id };
    let result = get_beekeeper_by_id_use_case::run(&repo, dto).await;

    if result.success {
        if let Some(b) = result.beekeeper {
            Ok(HttpResponse::Ok().json(b))
        } else {
            Ok(HttpResponse::NotFound().json(result))
        }
    } else {
        Ok(HttpResponse::NotFound().json(result))
    }
}

#[put("/honey-note/api/beekeeper/new")]
pub async fn put_new_beekeeper(
    _req: HttpRequest,
    payload: LoggedJson<Beekeeper>,
    pool: web::Data<sqlx::SqlitePool>,
) -> Result<HttpResponse, Error> {
    let dto = put_new_beekeeper_use_case::put_new_beekeeper_dto::PutNewBeekeeperRequestDto { beekeeper: payload.into_inner() };
    let repo = BeekeeperRepositorySqlite { pool: pool.get_ref().clone() };
    let result = put_new_beekeeper_use_case::run(&repo, dto, pool.get_ref()).await;
    if result.success {
        Ok(HttpResponse::Ok().json(result))
    } else {
        log::error!("put_new_beekeeper failed: {:?}", result.error_message);
        Ok(HttpResponse::BadRequest().json(result))
    }
}

#[put("/honey-note/api/beekeeper/edit/{id}")]
pub async fn put_edit_beekeeper(
    path: web::Path<i32>,
    _req: HttpRequest,
    payload: LoggedJson<Beekeeper>,
    pool: web::Data<sqlx::SqlitePool>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let dto = put_edit_beekeeper_use_case::put_edit_beekeeper_dto::PutEditBeekeeperRequestDto { 
        id, 
        beekeeper: payload.into_inner() 
    };
    let repo = BeekeeperRepositorySqlite { pool: pool.get_ref().clone() };
    let result = put_edit_beekeeper_use_case::run(&repo, dto, pool.get_ref()).await;
    if result.success {
        Ok(HttpResponse::Ok().json(result))
    } else {
        log::error!("put_edit_beekeeper failed: {:?}", result.error_message);
        Ok(HttpResponse::BadRequest().json(result))
    }
}
