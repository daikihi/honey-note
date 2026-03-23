use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Error};
use common::repository::users::{UserRepository, UserRepositorySqlite};
use common::libs::auth::{hash_password, verify_password, hash_email};
use common_type::models::user::User;
use common_type::models::session::SessionData;
use common_type::request::auth::auth::{SignupRequest, LoginRequest, AuthResponse, MeResponse};
use sqlx::SqlitePool;
use log::{info, warn};

#[post("/api/auth/signup")]
pub async fn signup(
    pool: web::Data<SqlitePool>,
    payload: web::Json<SignupRequest>,
) -> Result<HttpResponse, Error> {
    let repo = UserRepositorySqlite { pool: pool.get_ref().clone() };
    
    // ユーザー名を小文字に正規化
    let username = payload.username.to_lowercase();
    
    // ユーザー名の重複チェック
    if let Ok(Some(_)) = repo.find_by_username(&username).await {
        return Ok(HttpResponse::BadRequest().json(AuthResponse {
            success: false,
            message: Some("このユーザー名は既に使用されています".to_string()),
            user_id: None,
            username: None,
        }));
    }
    
    // メールアドレスのハッシュ化と重複チェック
    let email_hash = hash_email(&payload.email);
    if let Ok(true) = repo.exists_by_email_hash(&email_hash).await {
        return Ok(HttpResponse::BadRequest().json(AuthResponse {
            success: false,
            message: Some("このメールアドレスは既に登録されています".to_string()),
            user_id: None,
            username: None,
        }));
    }
    
    // パスワードのハッシュ化
    let password_hash = hash_password(&payload.password).map_err(|e| {
        actix_web::error::ErrorInternalServerError(e.to_string())
    })?;
    
    let new_user = User {
        id: None,
        username: username.clone(),
        email_hash,
        password_hash,
        display_name: payload.display_name.clone(),
        created_at: None,
        terminated_at: None,
        updated_at: None,
    };
    
    match repo.insert_user(new_user).await {
        Ok(id) => {
            info!("user signup success: username={}", username);
            Ok(HttpResponse::Ok().json(AuthResponse {
                success: true,
                message: None,
                user_id: Some(id as i32),
                username: Some(username),
            }))
        }
        Err(e) => {
            warn!("user signup failed: username={}, error={}", username, e);
            Ok(HttpResponse::InternalServerError().json(AuthResponse {
                success: false,
                message: Some("ユーザー登録に失敗しました".to_string()),
                user_id: None,
                username: None,
            }))
        }
    }
}

#[post("/api/auth/login")]
pub async fn login(
    pool: web::Data<SqlitePool>,
    payload: web::Json<LoginRequest>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let repo = UserRepositorySqlite { pool: pool.get_ref().clone() };
    let username = payload.username.to_lowercase();
    
    let user_opt = repo.find_by_username(&username).await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(e.to_string())
    })?;
    
    if let Some(user) = user_opt {
        if let Ok(true) = verify_password(&payload.password, &user.password_hash) {
            // セッション発行
            let user_id = user.id.unwrap();
            let session_data = SessionData::new(user_id, username.clone());
            
            session.insert("user", session_data).map_err(|e| {
                actix_web::error::ErrorInternalServerError(e.to_string())
            })?;
            
            info!("login success username={}", username);
            return Ok(HttpResponse::Ok().json(AuthResponse {
                success: true,
                message: None,
                user_id: Some(user_id),
                username: Some(username),
            }));
        }
    }
    
    warn!("login failed username={}", username);
    Ok(HttpResponse::Unauthorized().json(AuthResponse {
        success: false,
        message: Some("ユーザー名またはパスワードが正しくありません".to_string()),
        user_id: None,
        username: None,
    }))
}

#[post("/api/auth/logout")]
pub async fn logout(session: Session) -> Result<HttpResponse, Error> {
    if let Ok(Some(data)) = session.get::<SessionData>("user") {
        info!("logout username={}", data.username);
    }
    session.purge();
    Ok(HttpResponse::Ok().json(AuthResponse {
        success: true,
        message: Some("ログアウトしました".to_string()),
        user_id: None,
        username: None,
    }))
}

#[get("/api/auth/me")]
pub async fn me(session: Session) -> Result<HttpResponse, Error> {
    if let Ok(Some(data)) = session.get::<SessionData>("user") {
        Ok(HttpResponse::Ok().json(MeResponse {
            logged_in: true,
            user_id: Some(data.user_id),
            username: Some(data.username),
        }))
    } else {
        Ok(HttpResponse::Ok().json(MeResponse {
            logged_in: false,
            user_id: None,
            username: None,
        }))
    }
}
