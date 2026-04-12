use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,      // lowercase正規化済み
    pub email_hash: String,    // 比較用ハッシュ
    pub password_hash: String, // ハッシュ化済みパスワード
    pub display_name: Option<String>,
    pub created_at: Option<String>,
    pub terminated_at: Option<String>,
    pub updated_at: Option<String>,
}
