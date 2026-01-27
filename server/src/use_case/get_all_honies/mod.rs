pub mod get_all_honies_dto;
use common::repository::honeys::HoneyRepository;
use get_all_honies_dto::{GetAllHoneysRequestDto, GetAllHoneysResponseDto};

pub async fn run<T: HoneyRepository>(
    repo: &T,
    _request_dto: GetAllHoneysRequestDto,
) -> GetAllHoneysResponseDto {
    let honeys = match repo.get_all_honeys().await {
        Ok(h) => h,
        Err(e) => {
            log::error!("Error fetching honeys: {}", e);
            vec![]
        }
    };
    GetAllHoneysResponseDto { honeys }
}
