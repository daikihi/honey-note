use common_type::models::beekeeper::Beekeeper;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PutNewBeekeeperRequestDto {
    pub beekeeper: Beekeeper,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PutNewBeekeeperResponseDto {
    pub id: Option<i32>,
    pub success: bool,
    pub error_message: Option<String>,
}
