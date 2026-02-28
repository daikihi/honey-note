use common_type::models::beekeeper::Beekeeper;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PutEditBeekeeperRequestDto {
    pub id: i32,
    pub beekeeper: Beekeeper,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PutEditBeekeeperResponseDto {
    pub success: bool,
    pub error_message: Option<String>,
}
