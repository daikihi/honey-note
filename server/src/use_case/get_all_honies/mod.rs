pub mod get_all_honies_dto;
use common::repository::honeys::HoneyRepository;
use get_all_honies_dto::{GetAllHoneysRequestDto, GetAllHoneysResponseDto};

pub async fn run<T: HoneyRepository>(
    repo: &T,
    _request_dto: GetAllHoneysRequestDto,
) -> GetAllHoneysResponseDto {
    let honeys = repo.get_all_honeys().await.unwrap_or_else(|e| {
        log::error!("Error fetching honeys: {}", e);
        vec![]
    });
    GetAllHoneysResponseDto { honeys }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common_type::models::honey::Honey;
    use common_type::models::honey_detail::HoneyDetail;

    struct MockHoneyRepository;

    impl HoneyRepository for MockHoneyRepository {
        async fn insert_honey(&self, _honey: HoneyDetail) -> Result<i64, String> {
            Ok(1)
        }
        async fn update_honey(&self, _id: i64, _honey: HoneyDetail) -> Result<(), String> {
            Ok(())
        }
        async fn exists_honey(&self, _honey: &HoneyDetail) -> Result<bool, String> {
            Ok(false)
        }
        async fn exists_honey_by_id(&self, _id: i64) -> Result<bool, String> {
            Ok(true)
        }
        async fn get_all_honeys(&self) -> Result<Vec<Honey>, String> {
            Ok(vec![
                Honey {
                    id: Some(1),
                    name_jp: "アカシア".to_string(),
                    name_en: None,
                    beekkeeper: None,
                    origin_country: None,
                    origin_region: None,
                    harvest_year: None,
                    purchase_date: None,
                    note: None,
                },
            ])
        }
        async fn get_honey_by_id(&self, _id: i64) -> Result<HoneyDetail, String> {
            use common_type::request::honey::basic::HoneyEditBasicRequest;
            Ok(HoneyDetail {
                basic: HoneyEditBasicRequest {
                    name_jp: Some("Mock".to_string()),
                    beekeeper_name: None,
                    harvest_year: None,
                    country: None,
                    region: None,
                    flower_names: vec![],
                    honey_type: None,
                    volume: None,
                    purchase_date: None,
                }.to_honey_input_basic(),
                dynamic: vec![],
                created_at: None,
                updated_at: None,
            })
        }
    }

    #[tokio::test]
    async fn test_run() {
        let repo = MockHoneyRepository;
        let req = GetAllHoneysRequestDto {};
        let result = run(&repo, req).await;

        assert_eq!(result.honeys.len(), 1);
        assert_eq!(result.honeys[0].name_jp, "アカシア");
    }
}
