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

// あんまりここに実装する必要は無かったけど、将来なにか実装するときのためのフレームとして
#[cfg(test)]
mod tests {
    use super::*;
    use common_type::request::honey::basic::HoneyEditBasicRequest;

    #[test]
    fn test_put_new_honey_request_dto_to_honey_detail() {
        let basic = HoneyEditBasicRequest {
            name_jp: Some("アカシア".to_string()),
            beekeeper_name: Some("山田養蜂場".to_string()),
            harvest_year: Some("2023".to_string()),
            country: Some("日本".to_string()),
            region: Some("北海道".to_string()),
            flower_names: vec!["アカシア".to_string()],
            honey_type: Some("単花蜜".to_string()),
            volume: Some("300g".to_string()),
            purchase_date: Some("2023-05-20T10:00:00Z".to_string()),
        };

        let request_dto = PutNewHoneyRequestDto {
            new: HoneyNewRequest {
                basic,
                dynamic: vec![],
                created_at: None,
            },
        };

        let honey_detail: HoneyDetail = request_dto.to_honey_detail();

        assert_eq!(honey_detail.basic.name_jp.0, "アカシア");
        assert_eq!(honey_detail.basic.beekeeper_name.unwrap().0, "山田養蜂場");
        assert_eq!(honey_detail.basic.harvest_year.unwrap(), 2023);
        assert_eq!(honey_detail.basic.country.unwrap().0, "日本");
        assert_eq!(honey_detail.basic.region.unwrap().0, "北海道");
        assert_eq!(honey_detail.basic.flower_names[0].0, "アカシア");
        assert_eq!(honey_detail.basic.honey_type.unwrap().0, "単花蜜");
        assert_eq!(honey_detail.basic.volume.unwrap().0, "300g");
        assert!(honey_detail.basic.purchase_date.is_some());
    }

    #[test]
    fn test_put_new_honey_response_dto_properties() {
        let response_dto = PutNewHoneyResponseDto {
            id: Some(123),
            success: true,
            error_message: Some("error".to_string()),
        };

        assert_eq!(response_dto.id, Some(123));
        assert_eq!(response_dto.success, true);
        assert_eq!(response_dto.error_message, Some("error".to_string()));
    }
}
