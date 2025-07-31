use common_type::models::flowers::Flower as ModelFlower;

pub struct GetAllFlowersRequestDto {
    pub pool: sqlx::SqlitePool,
}

#[derive()]
pub struct GetAllFlowersResponseDto {
    pub flowers: Vec<ModelFlower>,
}
