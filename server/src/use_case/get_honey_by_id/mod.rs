pub mod get_honey_by_id_dto;
use common::repository::honeys::HoneyRepository;
use get_honey_by_id_dto::{GetHoneyByIdRequestDto, GetHoneyByIdResponseDto};

pub async fn run<T: HoneyRepository>(
    repo: &T,
    request: GetHoneyByIdRequestDto,
) -> GetHoneyByIdResponseDto {
    match repo.get_honey_by_id(request.id).await {
        Ok(detail) => GetHoneyByIdResponseDto { success: true, honey: Some(detail), error_message: None },
        Err(e) => GetHoneyByIdResponseDto { success: false, honey: None, error_message: Some(e) },
    }
}
