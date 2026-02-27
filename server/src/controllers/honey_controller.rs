use actix_web::{get, put, web, HttpResponse, Error, HttpRequest};
use web::Json;
use common::repository::honeys::HoneyRepositorySqlite;
use crate::{
    use_case::put_new_honey::put_new_honey_dto::{PutNewHoneyRequestDto, PutNewHoneyResponseDto},
    use_case::put_new_honey as put_new_honey_use_case,
    use_case::put_edit_honey::put_edit_honey_dto::{PutEditHoneyRequestDto, PutEditHoneyResponseDto},
    use_case::put_edit_honey as put_edit_honey_use_case,
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
    payload: Json<HoneyEditRequest>,
    pool: web::Data<sqlx::SqlitePool>,
) -> Result<HttpResponse, Error> {
    // DTO変換
    let dto = PutEditHoneyRequestDto { edit: payload.into_inner() };

    // Repositoryの実装を生成
    let repo = HoneyRepositorySqlite { pool: pool.get_ref().clone() };

    // UseCase呼び出し
    let result: PutEditHoneyResponseDto = put_edit_honey_use_case::run(&repo, dto).await;

    if result.success {
        Ok(HttpResponse::Ok().json(result))
    } else {
        log::error!("put_edit_honey failed: {:?}", result.error_message);
        Ok(HttpResponse::BadRequest().json(result))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App, web};
    use sqlx::SqlitePool;
    use common_type::request::honey::new::HoneyNewRequest;
    use common_type::request::honey::basic::HoneyEditBasicRequest;

    async fn setup_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::query(
            "CREATE TABLE honey (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                name_jp         TEXT NOT NULL,
                name_en         TEXT,
                beekeeper_id    INTEGER,
                origin_country  TEXT,
                origin_region   TEXT,
                harvest_year    INTEGER,
                purchase_date   DATE,
                note            TEXT
            );",
        )
            .execute(&pool)
            .await
            .unwrap();
        pool
    }

    fn create_test_request(name: &str) -> HoneyNewRequest {
        HoneyNewRequest {
            basic: HoneyEditBasicRequest {
                name_jp: Some(name.to_string()),
                beekeeper_name: None,
                harvest_year: None,
                country: None,
                region: None,
                flower_names: vec![],
                honey_type: None,
                volume: None,
                purchase_date: None,
            },
            dynamic: vec![],
            created_at: None,
        }
    }

    #[actix_web::test]
    async fn test_put_new_honey_success() {
        let pool = setup_db().await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(put_new_honey)
        ).await;

        let payload = create_test_request("新しくはちみつ");
        let req = test::TestRequest::put()
            .uri("/honey-note/api/honey/new")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body: PutNewHoneyResponseDto = test::read_body_json(resp).await;
        assert!(body.success);
        assert!(body.id.is_some());
    }

    #[actix_web::test]
    async fn test_put_new_honey_already_exists() {
        let pool = setup_db().await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(put_new_honey)
        ).await;

        let payload = create_test_request("既存のはちみつ");

        // 1回目：成功するはず
        let req1 = test::TestRequest::put()
            .uri("/honey-note/api/honey/new")
            .set_json(&payload)
            .to_request();
        let resp1 = test::call_service(&app, req1).await;
        assert!(resp1.status().is_success());

        // 2回目：同じ名前なので失敗するはず
        let req2 = test::TestRequest::put()
            .uri("/honey-note/api/honey/new")
            .set_json(&payload)
            .to_request();
        let resp2 = test::call_service(&app, req2).await;

        assert_eq!(resp2.status(), actix_web::http::StatusCode::BAD_REQUEST);

        let body: PutNewHoneyResponseDto = test::read_body_json(resp2).await;
        assert!(!body.success);
        assert_eq!(body.error_message, Some("既に同じはちみつデータが存在します".to_string()));
    }
    #[actix_web::test]
    async fn test_put_edit_honey_success() {
        let pool = setup_db().await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(put_new_honey)
                .service(put_edit_honey)
        ).await;

        // 1. 新規作成
        let payload_new = create_test_request("編集前のはちみつ");
        let req_new = test::TestRequest::put()
            .uri("/honey-note/api/honey/new")
            .set_json(&payload_new)
            .to_request();
        let resp_new = test::call_service(&app, req_new).await;
        let body_new: PutNewHoneyResponseDto = test::read_body_json(resp_new).await;
        let honey_id = body_new.id.unwrap();

        // 2. 編集
        let payload_edit = HoneyEditRequest {
            id: honey_id,
            basic: HoneyEditBasicRequest {
                name_jp: Some("編集後のはちみつ".to_string()),
                beekeeper_name: None,
                harvest_year: None,
                country: None,
                region: None,
                flower_names: vec![],
                honey_type: None,
                volume: None,
                purchase_date: None,
            },
            dynamic: vec![],
            updated_at: None,
        };
        let req_edit = test::TestRequest::put()
            .uri("/honey-note/api/honey/edit")
            .set_json(&payload_edit)
            .to_request();

        let resp_edit = test::call_service(&app, req_edit).await;
        assert!(resp_edit.status().is_success());

        let body_edit: PutEditHoneyResponseDto = test::read_body_json(resp_edit).await;
        assert!(body_edit.success);
    }

    #[actix_web::test]
    async fn test_put_edit_honey_no_such_id() {
        let pool = setup_db().await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(put_edit_honey)
        ).await;

        let payload_edit = HoneyEditRequest {
            id: 9999, // 存在しないID
            basic: HoneyEditBasicRequest {
                name_jp: Some("ななしのはちみつ".to_string()),
                beekeeper_name: None,
                harvest_year: None,
                country: None,
                region: None,
                flower_names: vec![],
                honey_type: None,
                volume: None,
                purchase_date: None,
            },
            dynamic: vec![],
            updated_at: None,
        };
        let req_edit = test::TestRequest::put()
            .uri("/honey-note/api/honey/edit")
            .set_json(&payload_edit)
            .to_request();

        let resp_edit = test::call_service(&app, req_edit).await;
        assert_eq!(resp_edit.status(), actix_web::http::StatusCode::BAD_REQUEST);

        let body_edit: PutEditHoneyResponseDto = test::read_body_json(resp_edit).await;
        assert!(!body_edit.success);
        assert_eq!(body_edit.error_message, Some("NoSuchHoneyIdExist".to_string()));
    }
//     @todo 同じ値でも名前が違えば登録できるというテストを書く
}
