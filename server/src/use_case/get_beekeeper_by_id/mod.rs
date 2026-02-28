pub mod get_beekeeper_by_id_dto;
use common::repository::beekeepers::BeekeeperRepository;
use get_beekeeper_by_id_dto::{GetBeekeeperByIdRequestDto, GetBeekeeperByIdResponseDto};

pub async fn run<T: BeekeeperRepository>(
    repo: &T,
    req: GetBeekeeperByIdRequestDto,
) -> GetBeekeeperByIdResponseDto {
    let id = req.id;
    match repo.get_beekeeper_by_id(id).await {
        Ok(beekeeper) => GetBeekeeperByIdResponseDto {
            success: true,
            beekeeper: Some(beekeeper),
            error_message: None,
        },
        Err(e) => GetBeekeeperByIdResponseDto {
            success: false,
            beekeeper: None,
            error_message: Some(e.to_string()),
        },
    }
}
