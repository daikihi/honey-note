pub struct GetBeekeeperByIdRequestDto {
    pub id: i32,
}

#[derive(serde::Serialize)]
pub struct GetBeekeeperByIdResponseDto {
    pub beekeeper: Option<common_type::models::beekeeper::Beekeeper>,
    pub success: bool,
    pub error_message: Option<String>,
}
