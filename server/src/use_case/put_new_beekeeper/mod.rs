pub mod put_new_beekeeper_dto;
use common::repository::beekeepers::BeekeeperRepository;
use put_new_beekeeper_dto::{PutNewBeekeeperRequestDto, PutNewBeekeeperResponseDto};

pub async fn run<T: BeekeeperRepository>(
    repo: &T,
    req: PutNewBeekeeperRequestDto,
    pool: &sqlx::SqlitePool,
    user_id: i32,
) -> PutNewBeekeeperResponseDto {
    let beekeeper = req.beekeeper;
    
    // Begin transaction
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return PutNewBeekeeperResponseDto {
            id: None,
            success: false,
            error_message: Some(e.to_string()),
        },
    };

    if repo.has_beekeeper(&beekeeper, user_id, &mut *tx).await {
        return PutNewBeekeeperResponseDto {
            id: None,
            success: false,
            error_message: Some("既に同じ名前の養蜂場が存在します".to_string()),
        };
    }

    match repo.insert_beekeeper(&beekeeper, user_id, &mut *tx).await {
        Ok(_) => {
            let id = repo.get_beekeeper_id_by_name(&beekeeper.name_jp, user_id, &mut *tx).await;
            if let Err(e) = tx.commit().await {
                return PutNewBeekeeperResponseDto {
                    id: None,
                    success: false,
                    error_message: Some(e.to_string()),
                };
            }
            PutNewBeekeeperResponseDto {
                id,
                success: true,
                error_message: None,
            }
        }
        Err(e) => PutNewBeekeeperResponseDto {
            id: None,
            success: false,
            error_message: Some(e.to_string()),
        },
    }
}
