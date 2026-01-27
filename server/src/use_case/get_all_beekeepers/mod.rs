pub mod get_all_beekeepers_dto;
use common::errors::AppError;
use common::repository::beekeepers::BeekeeperRepository;
use crate::use_case::get_all_beekeepers::get_all_beekeepers_dto::GetAllBeekeepersResponseDto;

pub async fn run<T: BeekeeperRepository>(
    repo: &T,
    _dto: get_all_beekeepers_dto::GetAllBeekeepersRequestDto,
) -> Result<get_all_beekeepers_dto::GetAllBeekeepersResponseDto, AppError> {
    let beekeepers = repo.get_all_beekeepers().await;
    match beekeepers {
        Ok(beekeepers) => {
            let response: GetAllBeekeepersResponseDto = GetAllBeekeepersResponseDto { beekeepers };
            Ok(response)
        }
        Err(e) => {
            log::error!("Error fetching beekeepers: {}", e);
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common_type::models::beekeeper::Beekeeper;

    struct MockBeekeeperRepository;

    impl BeekeeperRepository for MockBeekeeperRepository {
        async fn get_all_beekeepers(&self) -> Result<Vec<Beekeeper>, AppError> {
            Ok(vec![
                Beekeeper {
                    id: Some(1),
                    name_jp: "山田養蜂場".to_string(),
                    name_en: Some("Yamada Bee Farm".to_string()),
                    founding_year: Some(1950),
                    location_prefecture_id: Some(33),
                    location_city: Some("苫田郡鏡野町".to_string()),
                    website_url: Some("https://www.3838.com/".to_string()),
                    note: None,
                },
            ])
        }
        async fn get_beekeeper_id_by_name(&self, _name: &str) -> Option<i32> {
            Some(1)
        }
        async fn insert_beekeeper(&self, _beekeeper: &Beekeeper) -> Result<(), AppError> {
            Ok(())
        }
        async fn has_beekeeper(&self, _beekeeper: &Beekeeper) -> bool {
            true
        }
    }

    #[tokio::test]
    async fn test_run() {
        let repo = MockBeekeeperRepository;
        let req = get_all_beekeepers_dto::GetAllBeekeepersRequestDto {};
        let result = run(&repo, req).await.unwrap();

        assert_eq!(result.beekeepers.len(), 1);
        assert_eq!(result.beekeepers[0].name_jp, "山田養蜂場");
    }
}
