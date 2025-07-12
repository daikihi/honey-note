use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("データベースエラー: {0}")]
    DatabaseError(String),

    #[error("存在しません: {0}")]
    NotFound(String),

    #[error("不正な入力: {0}")]
    InvalidInput(String),
    // 必要に応じて追加
}
