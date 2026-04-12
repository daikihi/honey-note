use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct SignupRequest {
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub display_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthResponse {
    pub success: bool,
    pub message: Option<String>,
    pub user_id: Option<i32>,
    pub username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MeResponse {
    pub logged_in: bool,
    pub user_id: Option<i32>,
    pub username: Option<String>,
}
