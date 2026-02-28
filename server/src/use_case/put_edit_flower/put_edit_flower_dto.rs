use common_type::models::flowers::Flower;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PutEditFlowerRequestDto {
    pub id: i32,
    pub flower: Flower,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PutEditFlowerResponseDto {
    pub success: bool,
    pub error_message: Option<String>,
}
