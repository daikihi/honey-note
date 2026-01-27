//! はちみつリポジトリ（新規作成用）
//!
//! DBアクセスは未実装。traitとモック実装のみ。

// HoneyDetail型は、はちみつの詳細情報を表すドメインモデルです。
// 新規作成や編集時にDBへ保存する際のデータ構造として利用します。
use common_type::models::honey_detail::HoneyDetail;
use async_trait::async_trait;

#[async_trait]
pub trait HoneyRepository: Send + Sync {
    async fn insert_honey(&self, honey: HoneyDetail) -> Result<i64, String>;
}

/// モック実装例
pub struct HoneyRepositoryMock;

#[async_trait]
impl HoneyRepository for HoneyRepositoryMock {
    async fn insert_honey(&self, _honey: HoneyDetail) -> Result<i64, String> {
        // DBアクセスは未実装
        Ok(1) // 仮のID
    }
}
