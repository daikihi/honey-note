pub mod put_edit_flower_dto;
use common::repository::flowers::FlowerRepository;
use put_edit_flower_dto::{PutEditFlowerRequestDto, PutEditFlowerResponseDto};

pub async fn run<T: FlowerRepository>(
    repo: &T,
    req: PutEditFlowerRequestDto,
    pool: &sqlx::SqlitePool,
) -> PutEditFlowerResponseDto {
    let id = req.id;
    let flower = req.flower;

    // Begin transaction
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return PutEditFlowerResponseDto {
            success: false,
            error_message: Some(e.to_string()),
        },
    };

    match repo.exists_flower_by_id(id, &mut *tx).await {
        Ok(true) => {
            match repo.update_flower(id, &flower, &mut *tx).await {
                Ok(_) => {
                    if let Err(e) = tx.commit().await {
                        return PutEditFlowerResponseDto {
                            success: false,
                            error_message: Some(e.to_string()),
                        };
                    }
                    PutEditFlowerResponseDto {
                        success: true,
                        error_message: None,
                    }
                }
                Err(e) => PutEditFlowerResponseDto {
                    success: false,
                    error_message: Some(e.to_string()),
                },
            }
        }
        Ok(false) => PutEditFlowerResponseDto {
            success: false,
            error_message: Some("NoSuchFlowerIdExist".to_string()),
        },
        Err(e) => PutEditFlowerResponseDto {
            success: false,
            error_message: Some(e.to_string()),
        },
    }
}
