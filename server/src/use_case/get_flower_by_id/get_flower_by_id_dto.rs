pub struct GetFlowerByIdRequestDto {
    pub id: i32,
}

#[derive(serde::Serialize)]
pub struct GetFlowerByIdResponseDto {
    pub flower: Option<common_type::models::flowers::Flower>,
    pub success: bool,
    pub error_message: Option<String>,
}
