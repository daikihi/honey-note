use common::errors::AppError;

use crate::use_case::get_all_beekeepers::get_all_beekeepers_dto::GetAllBeekeepersResponseDto;

pub mod get_all_beekeepers_dto;

pub async fn run(dto: get_all_beekeepers_dto::GetAllBeekeepersRequestDto) -> Result<get_all_beekeepers_dto::GetAllBeekeepersResponseDto, AppError> {
    let _ = dto; // Now dto is empty
    let file_name = common::infrastructure::db::sqlx::db_file_name;
    let pool = common::infrastructure::db::sqlx::get_sqlite_pool(file_name.to_string());

    let beekeepers = common::repository::beekeepers::get_all_beekeepers(&pool).await;
    match beekeepers {
        Ok(beekeepers) => {
            let response: GetAllBeekeepersResponseDto = GetAllBeekeepersResponseDto {
                beekeepers,
            };
            Ok(response)
        }
        Err(e) => {
            log::error!("Error fetching beekeepers: {}", e);
            Err(e)
        }
    }

}