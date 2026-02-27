use common_type::models::honey_detail::HoneyDetail;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct GetHoneyByIdRequestDto {
    pub id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHoneyByIdResponseDto {
    pub success: bool,
    pub honey: Option<HoneyDetail>,
    pub error_message: Option<String>,
}
