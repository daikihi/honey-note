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
