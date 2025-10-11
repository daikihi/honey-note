use common::repository::honeys;
use common_type::models::honey::{front_app_model::HoneyDetail, Honey};

use crate::use_case::get_honey_details::get_honey_details_dto::{
    GetHoneyDetailsRequestDto, GetHoneyDetailsResponseDto,
};

pub mod get_honey_details_dto;

pub async fn run(request_dto: GetHoneyDetailsRequestDto) -> GetHoneyDetailsResponseDto {
    let pool: &sqlx::SqlitePool = &request_dto.pool;
    let honey_id: i32 = request_dto.honey_id;
    let model_honey_opt: Option<Honey> = honeys::get_honey_by_id(pool, honey_id).await;
    match model_honey_opt {
        Some(h) => {
            let honey_detail: HoneyDetail = create_honey_detail(h);
            GetHoneyDetailsResponseDto {
                honey_detail: Some(honey_detail),
            }
        }
        None => GetHoneyDetailsResponseDto { honey_detail: None },
    }
}

fn create_honey_detail(model_honey: Honey) -> HoneyDetail {
    HoneyDetail {
        honey: model_honey,
        flowers: vec![],
    }
}
