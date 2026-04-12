use common_type::models::user::User as ModelUser;
use sqlx::{Sqlite, Executor};

#[derive(Debug, sqlx::FromRow, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub email_hash: String,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub created_at: Option<String>,
    pub terminated_at: Option<String>,
    pub updated_at: Option<String>,
}

impl User {
    /// Domain Model から DB 用構造体へ変換します
    pub fn from_model(model: ModelUser) -> Self {
        Self {
            id: model.id,
            username: model.username,
            email_hash: model.email_hash,
            password_hash: model.password_hash,
            display_name: model.display_name,
            created_at: model.created_at,
            terminated_at: model.terminated_at,
            updated_at: model.updated_at,
        }
    }

    /// DB 用構造体から Domain Model へ変換します
    pub fn to_model(self) -> ModelUser {
        ModelUser {
            id: self.id,
            username: self.username,
            email_hash: self.email_hash,
            password_hash: self.password_hash,
            display_name: self.display_name,
            created_at: self.created_at,
            terminated_at: self.terminated_at,
            updated_at: self.updated_at,
        }
    }

    /// 新規ユーザーを登録します
    /// 
    /// # Arguments
    /// * `executor` - SQLx のトランザクションまたはプール
    /// 
    /// # Returns
    /// 登録されたユーザーの ID (ROWID) を返します
    pub async fn insert<'a, E>(&self, executor: E) -> Result<i64, sqlx::Error>
    where
        E: Executor<'a, Database = Sqlite>,
    {
        // ユーザー名、メールアドレスハッシュ、パスワードハッシュ、表示名を保存します。
        // id は AUTOINCREMENT により自動生成されます。
        // created_at, updated_at は DB のデフォルト値およびトリガーによって管理されます。
        let query = r#"
            INSERT INTO users (username, email_hash, password_hash, display_name)
            VALUES (?, ?, ?, ?)
        "#;
        let result = sqlx::query(query)
            .bind(&self.username)
            .bind(&self.email_hash)
            .bind(&self.password_hash)
            .bind(&self.display_name)
            .execute(executor)
            .await?;
        
        // 最後に挿入された ID を取得して返却します
        Ok(result.last_insert_rowid())
    }

    /// ユーザー名でユーザーを検索します
    /// 退会済み（terminated_at IS NOT NULL）のユーザーは対象外とします
    /// 
    /// # Arguments
    /// * `username` - 検索するユーザー名（呼び出し側で小文字化されている必要があります）
    pub async fn find_by_username<'a, E>(username: &str, executor: E) -> Result<Option<Self>, sqlx::Error>
    where
        E: Executor<'a, Database = Sqlite>,
    {
        // terminated_at が NULL である（＝有効な）ユーザーのみを取得します
        let query = "SELECT * FROM users WHERE username = ? AND terminated_at IS NULL";
        sqlx::query_as::<_, Self>(query)
            .bind(username)
            .fetch_optional(executor)
            .await
    }

    /// ID でユーザーを検索します
    /// 退会済み（terminated_at IS NOT NULL）のユーザーは対象外とします
    pub async fn find_by_id<'a, E>(id: i32, executor: E) -> Result<Option<Self>, sqlx::Error>
    where
        E: Executor<'a, Database = Sqlite>,
    {
        // terminated_at が NULL である（＝有効な）ユーザーのみを取得します
        let query = "SELECT * FROM users WHERE id = ? AND terminated_at IS NULL";
        sqlx::query_as::<_, Self>(query)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    /// メールアドレスのハッシュ値が既に登録されているか確認します
    /// 重複チェック（UNIQUE 制約違反の防止）に使用します
    pub async fn exists_by_email_hash<'a, E>(email_hash: &str, executor: E) -> Result<bool, sqlx::Error>
    where
        E: Executor<'a, Database = Sqlite>,
    {
        // レコードの存在確認のみを行うため、EXISTS 句を使用して効率的にチェックします
        let query = "SELECT EXISTS(SELECT 1 FROM users WHERE email_hash = ?)";
        let result: (i64,) = sqlx::query_as(query)
            .bind(email_hash)
            .fetch_one(executor)
            .await?;
        
        // SQLite の EXISTS は 1 または 0 を返すため、bool に変換します
        Ok(result.0 != 0)
    }
}
