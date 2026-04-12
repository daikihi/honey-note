use serde::{Deserialize, Serialize};

/// セッションに保存するデータ構造
/// 仕様に基づき、将来の構造変更に備えてバージョン管理を含めます
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionData {
    pub version: u32,
    pub user_id: i32,
    pub username: String,
}

impl SessionData {
    pub fn new(user_id: i32, username: String) -> Self {
        Self {
            version: 1,
            user_id,
            username,
        }
    }
}
