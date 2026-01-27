use common_type::models::flowers::Flower as ModelFlower;

pub struct GetAllFlowersRequestDto {}

#[derive()]
pub struct GetAllFlowersResponseDto {
    pub flowers: Vec<ModelFlower>,
}
