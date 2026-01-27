pub mod get_all_flowers_dto;
use common::repository::flowers::FlowerRepository;

pub async fn run<T: FlowerRepository>(
    repo: &T,
    _req: get_all_flowers_dto::GetAllFlowersRequestDto,
) -> Result<get_all_flowers_dto::GetAllFlowersResponseDto, common::errors::AppError> {
    let flowers = repo.get_all_flowers().await?;
    Ok(get_all_flowers_dto::GetAllFlowersResponseDto { flowers })
}
