pub mod get_all_honies_dto;
use common::repository::honeys::HoneyRepository;
use get_all_honies_dto::{GetAllHoneysRequestDto, GetAllHoneysResponseDto};

pub async fn run<T: HoneyRepository>(
    repo: &T,
    _request_dto: GetAllHoneysRequestDto,
) -> GetAllHoneysResponseDto {
    let honeys = match repo.get_all_honeys().await {
        Ok(h) => h,
        Err(e) => {
            log::error!("Error fetching honeys: {}", e);
            vec![]
        }
    };
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
        async fn exists_honey(&self, _honey: &HoneyDetail) -> Result<bool, String> {
            Ok(false)
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
