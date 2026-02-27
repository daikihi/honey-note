//! はちみつ新規作成ユースケース
//!
//! HoneyNewRequest DTOを受け取り、リポジトリ経由で新規作成を行う。

pub mod put_new_honey_dto;
use common::repository::honeys::HoneyRepository;
use common_type::models::honey_detail::HoneyDetail;
use put_new_honey_dto::{PutNewHoneyRequestDto, PutNewHoneyResponseDto};

pub async fn run<T: HoneyRepository>(
    repo: &T,
    req: PutNewHoneyRequestDto,
) -> PutNewHoneyResponseDto {
    let honey_detail: HoneyDetail = req.to_honey_detail();
    // 既存チェック
    match repo.exists_honey(&honey_detail).await {
        Ok(true) => {
            return PutNewHoneyResponseDto {
                id: None,
                success: false,
                error_message: Some("既に同じはちみつデータが存在します".to_string()),
            };
        }
        Ok(false) => {
            // 新規登録
            match repo.insert_honey(honey_detail).await {
                Ok(id) => PutNewHoneyResponseDto { id: Some(id), success: true, error_message: None },
                Err(e) => PutNewHoneyResponseDto { id: None, success: false, error_message: Some(e) },
            }
        }
        Err(e) => PutNewHoneyResponseDto { id: None, success: false, error_message: Some(e) },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common_type::request::honey::new::HoneyNewRequest;
    use common_type::request::honey::basic::HoneyEditBasicRequest;
    use common_type::models::honey::Honey;

    struct MockHoneyRepository {
        exists: bool,
    }

    impl HoneyRepository for MockHoneyRepository {
        async fn insert_honey(&self, _honey: HoneyDetail) -> Result<i64, String> {
            Ok(100)
        }
        async fn update_honey(&self, _id: i64, _honey: HoneyDetail) -> Result<(), String> {
            Ok(())
        }
        async fn exists_honey(&self, _honey: &HoneyDetail) -> Result<bool, String> {
            Ok(self.exists)
        }
        async fn exists_honey_by_id(&self, _id: i64) -> Result<bool, String> {
            Ok(true)
        }
        async fn get_all_honeys(&self) -> Result<Vec<Honey>, String> {
            Ok(vec![])
        }
        async fn get_honey_by_id(&self, _id: i64) -> Result<HoneyDetail, String> {
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

    fn create_request(name: &str) -> PutNewHoneyRequestDto {
        PutNewHoneyRequestDto {
            new: HoneyNewRequest {
                basic: HoneyEditBasicRequest {
                    name_jp: Some(name.to_string()),
                    beekeeper_name: None,
                    harvest_year: None,
                    country: None,
                    region: None,
                    flower_names: vec![],
                    honey_type: None,
                    volume: None,
                    purchase_date: None,
                },
                dynamic: vec![],
                created_at: None,
            },
        }
    }

    #[tokio::test]
    async fn test_run_success() {
        let repo = MockHoneyRepository { exists: false };
        let req = create_request("新規はちみつ");
        let result = run(&repo, req).await;

        assert!(result.success);
        assert_eq!(result.id, Some(100));
        assert!(result.error_message.is_none());
    }

    #[tokio::test]
    async fn test_run_already_exists() {
        let repo = MockHoneyRepository { exists: true };
        let req = create_request("既存はちみつ");
        let result = run(&repo, req).await;

        assert!(!result.success);
        assert_eq!(result.id, None);
        assert_eq!(result.error_message, Some("既に同じはちみつデータが存在します".to_string()));
    }
}
