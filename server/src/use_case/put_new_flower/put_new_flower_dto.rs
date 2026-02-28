use common_type::models::flowers::Flower;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PutNewFlowerRequestDto {
    pub flower: Flower,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PutNewFlowerResponseDto {
    pub id: Option<i32>,
    pub success: bool,
    pub error_message: Option<String>,
}
