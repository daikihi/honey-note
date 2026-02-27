//! はちみつ編集ユースケース
//!
//! HoneyEditRequest DTOを受け取り、リポジトリ経由で更新を行う。

pub mod put_edit_honey_dto;
use common::repository::honeys::HoneyRepository;
use common_type::models::honey_detail::HoneyDetail;
use put_edit_honey_dto::{PutEditHoneyRequestDto, PutEditHoneyResponseDto};

pub async fn run<T: HoneyRepository>(
    repo: &T,
    req: PutEditHoneyRequestDto,
) -> PutEditHoneyResponseDto {
    let id = req.edit.id;
    let honey_detail: HoneyDetail = req.to_honey_detail();

    // 既存チェック
    match repo.exists_honey_by_id(id).await {
        Ok(true) => {
            // 更新
            match repo.update_honey(id, honey_detail).await {
                Ok(_) => PutEditHoneyResponseDto { success: true, error_message: None },
                Err(e) => PutEditHoneyResponseDto { success: false, error_message: Some(e) },
            }
        }
        Ok(false) => {
            PutEditHoneyResponseDto {
                success: false,
                error_message: Some("NoSuchHoneyIdExist".to_string()),
            }
        }
        Err(e) => PutEditHoneyResponseDto { success: false, error_message: Some(e) },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common_type::request::honey::edit::HoneyEditRequest;
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
            Ok(false)
        }
        async fn exists_honey_by_id(&self, _id: i64) -> Result<bool, String> {
            Ok(self.exists)
        }
        async fn get_all_honeys(&self) -> Result<Vec<Honey>, String> {
            Ok(vec![])
        }
    }

    fn create_request(id: i64, name: &str) -> PutEditHoneyRequestDto {
        PutEditHoneyRequestDto {
            edit: HoneyEditRequest {
                id,
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
                updated_at: None,
            },
        }
    }

    #[tokio::test]
    async fn test_run_success() {
        let repo = MockHoneyRepository { exists: true };
        let req = create_request(1, "編集済みはちみつ");
        let result = run(&repo, req).await;

        assert!(result.success);
        assert!(result.error_message.is_none());
    }

    #[tokio::test]
    async fn test_run_no_such_id() {
        let repo = MockHoneyRepository { exists: false };
        let req = create_request(999, "存在しないはちみつ");
        let result = run(&repo, req).await;

        assert!(!result.success);
        assert_eq!(result.error_message, Some("NoSuchHoneyIdExist".to_string()));
    }
}
