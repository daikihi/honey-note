pub mod get_all_flowers_dto;

pub async fn run(
    req: get_all_flowers_dto::GetAllFlowersRequestDto,
) -> Result<get_all_flowers_dto::GetAllFlowersResponseDto, common::errors::AppError> {
    let flowers = common::repository::flowers::get_all_flowers(&req.pool).await?;
    Ok(get_all_flowers_dto::GetAllFlowersResponseDto { flowers })
}
