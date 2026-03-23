use common_type::models::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn insert_user(&self, user: User) -> Result<i64, String>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, String>;
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, String>;
    async fn exists_by_email_hash(&self, email_hash: &str) -> Result<bool, String>;
}

pub struct UserRepositorySqlite {
    pub pool: sqlx::SqlitePool,
}

#[async_trait]
impl UserRepository for UserRepositorySqlite {
    async fn insert_user(&self, user: User) -> Result<i64, String> {
        let db_user = crate::infrastructure::db::sqlx::user::User::from_model(user);
        db_user.insert(&self.pool).await.map_err(|e| e.to_string())
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, String> {
        crate::infrastructure::db::sqlx::user::User::find_by_username(username, &self.pool)
            .await
            .map(|opt| opt.map(|u| u.to_model()))
            .map_err(|e| e.to_string())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<User>, String> {
        crate::infrastructure::db::sqlx::user::User::find_by_id(id, &self.pool)
            .await
            .map(|opt| opt.map(|u| u.to_model()))
            .map_err(|e| e.to_string())
    }

    async fn exists_by_email_hash(&self, email_hash: &str) -> Result<bool, String> {
        crate::infrastructure::db::sqlx::user::User::exists_by_email_hash(email_hash, &self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}
