pub mod get_flower_by_id_dto;
use common::repository::flowers::FlowerRepository;
use get_flower_by_id_dto::{GetFlowerByIdRequestDto, GetFlowerByIdResponseDto};

pub async fn run<T: FlowerRepository>(
    repo: &T,
    req: GetFlowerByIdRequestDto,
) -> GetFlowerByIdResponseDto {
    let id = req.id;
    match repo.get_flower_by_id(id).await {
        Ok(flower) => GetFlowerByIdResponseDto {
            success: true,
            flower: Some(flower),
            error_message: None,
        },
        Err(e) => GetFlowerByIdResponseDto {
            success: false,
            flower: None,
            error_message: Some(e.to_string()),
        },
    }
}
