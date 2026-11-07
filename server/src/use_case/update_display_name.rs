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
