pub mod put_edit_beekeeper_dto;
use common::repository::beekeepers::BeekeeperRepository;
use put_edit_beekeeper_dto::{PutEditBeekeeperRequestDto, PutEditBeekeeperResponseDto};

pub async fn run<T: BeekeeperRepository>(
    repo: &T,
    req: PutEditBeekeeperRequestDto,
    pool: &sqlx::SqlitePool,
) -> PutEditBeekeeperResponseDto {
    let id = req.id;
    let beekeeper = req.beekeeper;

    // Begin transaction
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return PutEditBeekeeperResponseDto {
            success: false,
            error_message: Some(e.to_string()),
        },
    };

    match repo.exists_beekeeper_by_id(id, &mut *tx).await {
        Ok(true) => {
            match repo.update_beekeeper(id, &beekeeper, &mut *tx).await {
                Ok(_) => {
                    if let Err(e) = tx.commit().await {
                        return PutEditBeekeeperResponseDto {
                            success: false,
                            error_message: Some(e.to_string()),
                        };
                    }
                    PutEditBeekeeperResponseDto {
                        success: true,
                        error_message: None,
                    }
                }
                Err(e) => PutEditBeekeeperResponseDto {
                    success: false,
                    error_message: Some(e.to_string()),
                },
            }
        }
        Ok(false) => PutEditBeekeeperResponseDto {
            success: false,
            error_message: Some("NoSuchBeekeeperIdExist".to_string()),
        },
        Err(e) => PutEditBeekeeperResponseDto {
            success: false,
            error_message: Some(e.to_string()),
        },
    }
}
