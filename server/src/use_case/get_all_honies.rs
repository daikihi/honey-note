pub mod get_all_honies_dto;

use common::repository::honeys;
use get_all_honies_dto::{GetAllHoneysRequestDto, GetAllHoneysResponseDto};

pub async fn run(request_dto: GetAllHoneysRequestDto) -> GetAllHoneysResponseDto {
    let pool = request_dto.pool;
    let honeys: Vec<common_type::models::honey::Honey> = honeys::get_all_honies(&pool).await;
    GetAllHoneysResponseDto { honeys: honeys }
}
