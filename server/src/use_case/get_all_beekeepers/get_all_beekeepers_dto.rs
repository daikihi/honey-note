use common::models::beekeeper::Beekeeper;

pub struct GetAllBeekeepersRequestDto {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetAllBeekeepersResponseDto {
    pub beekeepers: Vec<Beekeeper>,
}