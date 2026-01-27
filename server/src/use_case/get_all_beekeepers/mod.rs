pub mod get_all_beekeepers_dto;
use common::errors::AppError;
use common::repository::beekeepers::BeekeeperRepository;
use crate::use_case::get_all_beekeepers::get_all_beekeepers_dto::GetAllBeekeepersResponseDto;

pub async fn run<T: BeekeeperRepository>(
    repo: &T,
    _dto: get_all_beekeepers_dto::GetAllBeekeepersRequestDto,
) -> Result<get_all_beekeepers_dto::GetAllBeekeepersResponseDto, AppError> {
    let beekeepers = repo.get_all_beekeepers().await;
    match beekeepers {
        Ok(beekeepers) => {
            let response: GetAllBeekeepersResponseDto = GetAllBeekeepersResponseDto { beekeepers };
            Ok(response)
        }
        Err(e) => {
            log::error!("Error fetching beekeepers: {}", e);
            Err(e)
        }
    }
}
