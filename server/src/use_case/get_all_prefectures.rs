use common::errors::AppError;
use common::repository::prefectures::PrefectureRepository;
use common_type::models::prefectures::Prefecture;

pub async fn run<T: PrefectureRepository>(repo: &T) -> Result<Vec<Prefecture>, AppError> {
    match repo.get_all_prefectures().await {
        Ok(prefectures) => Ok(prefectures),
        Err(e) => {
            log::error!("Failed to fetch prefectures: {}", e);
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockPrefectureRepository;

    impl PrefectureRepository for MockPrefectureRepository {
        async fn get_all_prefectures(&self) -> Result<Vec<Prefecture>, AppError> {
            Ok(vec![
                Prefecture {
                    id: 1,
                    name_jp: "北海道".to_string(),
                    name_en: "Hokkaido".to_string(),
                },
                Prefecture {
                    id: 2,
                    name_jp: "青森県".to_string(),
                    name_en: "Aomori".to_string(),
                },
            ])
        }
        async fn get_prefecture_by_name(&self, _name: &str) -> Result<Prefecture, AppError> {
            Err(AppError::NotFound("Not implemented".to_string()))
        }
        async fn insert_prefecture(&self, _model: &Prefecture) -> Result<(), AppError> {
            Ok(())
        }
        async fn has_prefecture(&self, _model: &Prefecture) -> Result<bool, AppError> {
            Ok(true)
        }
    }

    #[tokio::test]
    async fn test_run() {
        let repo = MockPrefectureRepository;
        let result = run(&repo).await.unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name_jp, "北海道");
        assert_eq!(result[1].name_jp, "青森県");
    }
}
