//! put_edit_honeyユースケース用DTO

use common_type::request::honey::edit::HoneyEditRequest;
use common_type::models::honey_detail::HoneyDetail;

#[derive(Debug, Clone)]
pub struct PutEditHoneyRequestDto {
    pub edit: HoneyEditRequest,
}

impl PutEditHoneyRequestDto {
    /// Dto→ドメインモデル（HoneyDetail）への変換
    pub fn to_honey_detail(&self) -> HoneyDetail {
        self.edit.to_honey_input()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PutEditHoneyResponseDto {
    pub success: bool,
    pub error_message: Option<String>,
}
