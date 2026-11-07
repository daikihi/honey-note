use common::repository::users::UserRepository;
use common_type::models::user::DISPLAY_NAME_MAX_CHARS;

pub async fn update_display_name(
    repo: &impl UserRepository,
    user_id: i32,
    input: &str,
) -> Result<bool, String> {
    let display_name = input.trim();
    if display_name.is_empty() {
        return Err("表示名を入力してください".to_string());
    }
    if display_name.chars().count() > DISPLAY_NAME_MAX_CHARS {
        return Err("表示名は50文字以内にしてください".to_string());
    }
    let user = repo.find_by_id(user_id).await?;
    let Some(user) = user else { return Ok(false) };
    if user.display_name.as_deref() == Some(display_name) {
        return Ok(true);
    }
    repo.update_display_name(user_id, display_name).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use common_type::models::user::User;
    use std::sync::Mutex;

    #[derive(Debug, PartialEq)]
    enum Call {
        Find(i32),
        Update(i32, String),
    }

    struct MockUserRepository {
        find_result: Result<Option<User>, String>,
        update_result: Result<bool, String>,
        calls: Mutex<Vec<Call>>,
    }

    impl MockUserRepository {
        fn with_user(display_name: Option<&str>) -> Self {
            Self {
                find_result: Ok(Some(User {
                    id: Some(42),
                    username: "alice".to_string(),
                    email_hash: "email-hash".to_string(),
                    password_hash: "password-hash".to_string(),
                    display_name: display_name.map(str::to_string),
                    created_at: None,
                    terminated_at: None,
                    updated_at: None,
                })),
                update_result: Ok(true),
                calls: Mutex::new(Vec::new()),
            }
        }

        fn calls(&self) -> Vec<Call> {
            std::mem::take(&mut self.calls.lock().unwrap())
        }
    }

    #[async_trait::async_trait]
    impl UserRepository for MockUserRepository {
        async fn insert_user(&self, _: User) -> Result<i64, String> {
            panic!("insert_user must not be called")
        }

        async fn find_by_username(&self, _: &str) -> Result<Option<User>, String> {
            panic!("find_by_username must not be called")
        }

        async fn find_by_id(&self, id: i32) -> Result<Option<User>, String> {
            self.calls.lock().unwrap().push(Call::Find(id));
            self.find_result.clone()
        }

        async fn exists_by_email_hash(&self, _: &str) -> Result<bool, String> {
            panic!("exists_by_email_hash must not be called")
        }

        async fn update_display_name(&self, id: i32, display_name: &str) -> Result<bool, String> {
            self.calls
                .lock()
                .unwrap()
                .push(Call::Update(id, display_name.to_string()));
            self.update_result.clone()
        }
    }

    #[tokio::test]
    async fn rejects_empty_and_whitespace_without_accessing_repository() {
        for input in ["", "  \t\n  "] {
            let repo = MockUserRepository::with_user(Some("Alice"));
            assert_eq!(
                update_display_name(&repo, 42, input).await,
                Err("表示名を入力してください".to_string())
            );
            assert!(repo.calls().is_empty());
        }
    }

    #[tokio::test]
    async fn rejects_more_than_50_characters_after_trimming_without_accessing_repository() {
        let repo = MockUserRepository::with_user(Some("Alice"));
        assert_eq!(
            update_display_name(&repo, 42, &format!("  {}  ", "あ".repeat(51))).await,
            Err("表示名は50文字以内にしてください".to_string())
        );
        assert!(repo.calls().is_empty());
    }

    #[tokio::test]
    async fn accepts_50_characters_and_sends_trimmed_name_to_the_session_user_only() {
        let repo = MockUserRepository::with_user(Some("Alice"));
        let name = "あ".repeat(50);
        assert_eq!(
            update_display_name(&repo, 42, &format!(" \t{name}\n ")).await,
            Ok(true)
        );
        assert_eq!(repo.calls(), vec![Call::Find(42), Call::Update(42, name)]);
    }

    #[tokio::test]
    async fn missing_user_returns_false_without_updating() {
        let mut repo = MockUserRepository::with_user(None);
        repo.find_result = Ok(None);
        assert_eq!(update_display_name(&repo, 42, "New name").await, Ok(false));
        assert_eq!(repo.calls(), vec![Call::Find(42)]);
    }

    #[tokio::test]
    async fn unchanged_name_returns_true_without_updating() {
        let repo = MockUserRepository::with_user(Some("Alice"));
        assert_eq!(update_display_name(&repo, 42, "  Alice  ").await, Ok(true));
        assert_eq!(repo.calls(), vec![Call::Find(42)]);
    }

    #[tokio::test]
    async fn missing_stored_name_can_be_updated() {
        let repo = MockUserRepository::with_user(None);
        assert_eq!(update_display_name(&repo, 42, "New name").await, Ok(true));
        assert_eq!(
            repo.calls(),
            vec![Call::Find(42), Call::Update(42, "New name".to_string())]
        );
    }

    #[tokio::test]
    async fn find_error_is_returned_without_updating() {
        let mut repo = MockUserRepository::with_user(None);
        repo.find_result = Err("find failed".to_string());
        assert_eq!(
            update_display_name(&repo, 42, "New name").await,
            Err("find failed".to_string())
        );
        assert_eq!(repo.calls(), vec![Call::Find(42)]);
    }

    #[tokio::test]
    async fn update_false_is_returned_without_claiming_success() {
        let mut repo = MockUserRepository::with_user(Some("Alice"));
        repo.update_result = Ok(false);
        assert_eq!(update_display_name(&repo, 42, "New name").await, Ok(false));
        assert_eq!(
            repo.calls(),
            vec![Call::Find(42), Call::Update(42, "New name".to_string())]
        );
    }

    #[tokio::test]
    async fn update_error_is_returned_without_claiming_success() {
        let mut repo = MockUserRepository::with_user(Some("Alice"));
        repo.update_result = Err("update failed".to_string());
        assert_eq!(
            update_display_name(&repo, 42, "New name").await,
            Err("update failed".to_string())
        );
        assert_eq!(
            repo.calls(),
            vec![Call::Find(42), Call::Update(42, "New name".to_string())]
        );
    }
}
