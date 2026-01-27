//! put_new_honeyユースケース用DTO

use common_type::request::honey::new::HoneyNewRequest;
use common_type::models::honey_detail::HoneyDetail;

#[derive(Debug, Clone)]
pub struct PutNewHoneyRequestDto {
    pub new: HoneyNewRequest,
}

impl PutNewHoneyRequestDto {
    /// Dto→ドメインモデル（HoneyDetail）への変換
    pub fn to_honey_detail(&self) -> HoneyDetail {
        self.new.to_honey_detail()
    }
}

#[derive(Debug, Clone,serde::Serialize, serde::Deserialize)]
pub struct PutNewHoneyResponseDto {
    pub id: Option<i64>,
    pub success: bool,
    pub error_message: Option<String>,
}
