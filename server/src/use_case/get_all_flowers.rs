pub mod get_all_flowers_dto;

pub async fn run(
    req: get_all_flowers_dto::get_all_flowers_request_dto,
) -> Result<get_all_flowers_dto::get_all_flowers_response_dto, common::errors::AppError> {
    let flowers = common::repository::flowers::get_all_flowers(&req.pool).await?;
    Ok(get_all_flowers_dto::get_all_flowers_response_dto { flowers })
}
