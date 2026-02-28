pub mod put_new_flower_dto;
use common::repository::flowers::FlowerRepository;
use put_new_flower_dto::{PutNewFlowerRequestDto, PutNewFlowerResponseDto};

pub async fn run<T: FlowerRepository>(
    repo: &T,
    req: PutNewFlowerRequestDto,
    pool: &sqlx::SqlitePool,
) -> PutNewFlowerResponseDto {
    let flower = req.flower;

    // Begin transaction
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return PutNewFlowerResponseDto {
            id: None,
            success: false,
            error_message: Some(e.to_string()),
        },
    };

    match repo.has_flower(&flower, &mut *tx).await {
        Ok(true) => return PutNewFlowerResponseDto {
            id: None,
            success: false,
            error_message: Some("既に同じ名前の蜜源が存在します".to_string()),
        },
        Ok(false) => {
            match repo.insert_flower(&flower, &mut *tx).await {
                Ok(_) => {
                    let id = repo.get_flower_id_by_name(&flower.name_jp, &mut *tx).await;
                    if let Err(e) = tx.commit().await {
                        return PutNewFlowerResponseDto {
                            id: None,
                            success: false,
                            error_message: Some(e.to_string()),
                        };
                    }
                    PutNewFlowerResponseDto {
                        id,
                        success: true,
                        error_message: None,
                    }
                }
                Err(e) => PutNewFlowerResponseDto {
                    id: None,
                    success: false,
                    error_message: Some(e.to_string()),
                },
            }
        }
        Err(e) => PutNewFlowerResponseDto {
            id: None,
            success: false,
            error_message: Some(e.to_string()),
        },
    }
}
