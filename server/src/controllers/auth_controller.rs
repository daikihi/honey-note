use crate::middleware::auth::AuthenticatedUser;
use crate::use_case::update_display_name::update_display_name as update_name;
use actix_session::Session;
use actix_web::http::header;
use actix_web::{get, post, put, web, Error, HttpRequest, HttpResponse};
use common::libs::auth::{hash_email, hash_password, verify_password};
use common::repository::users::{UserRepository, UserRepositorySqlite};
use common_type::models::session::SessionData;
use common_type::models::user::User;
use common_type::request::auth::auth::{
    AuthResponse, LoginRequest, MeResponse, SignupRequest, UpdateDisplayNameRequest,
};
use log::{debug, info, warn};
use sqlx::SqlitePool;
use validator::Validate;

#[post("/api/auth/signup")]
/// ユーザー名の前後空白を除いて登録する。
pub async fn signup(
    pool: web::Data<SqlitePool>,
    payload: web::Json<SignupRequest>,
) -> Result<HttpResponse, Error> {
    // バリデーション
    let validation_error = payload.validate().err();
    let username = payload.username.trim().to_lowercase();
    if username.is_empty() || payload.password.is_empty() || validation_error.is_some() {
        warn!("バリデーションエラー: {:?}", validation_error);
        return Ok(HttpResponse::BadRequest().json(AuthResponse {
            success: false,
            message: Some("入力内容を確認してください。".to_string()),
            user_id: None,
            username: None,
        }));
    }

    let repo = UserRepositorySqlite {
        pool: pool.get_ref().clone(),
    };

    // ユーザー名は登録とログインで同じ規則に正規化する。
    debug!("username: {}", username);

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
    let password_hash = hash_password(&payload.password)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    let display_name = payload
        .display_name
        .clone()
        .unwrap_or_else(|| username.clone());
    let new_user = User {
        id: None,
        username: username.clone(),
        email_hash,
        password_hash,
        display_name: Some(display_name.clone()),
        created_at: None,
        terminated_at: None,
        updated_at: None,
    };

    match repo.insert_user(new_user).await {
        Ok(id) => {
            info!(
                "{}",
                serde_json::json!({
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                    "event": "user_signup",
                    "user_id": id,
                    "username": username,
                    "display_name": display_name,
                })
            );
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
/// 登録時と同じ規則でユーザー名を照合する。
pub async fn login(
    pool: web::Data<SqlitePool>,
    payload: web::Json<LoginRequest>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let repo = UserRepositorySqlite {
        pool: pool.get_ref().clone(),
    };
    let username = payload.username.trim().to_lowercase();

    let user_opt = repo
        .find_by_username(&username)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    if let Some(user) = user_opt {
        let verified = verify_password(&payload.password, &user.password_hash)
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
        if verified {
            // セッション発行
            let user_id = user.id.unwrap();
            let session_data = SessionData::new(user_id, username.clone());

            session.renew();
            session
                .insert("user", session_data)
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

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
pub async fn me(session: Session, pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error> {
    if let Ok(Some(data)) = session.get::<SessionData>("user") {
        let repo = UserRepositorySqlite {
            pool: pool.get_ref().clone(),
        };
        let user = repo
            .find_by_id(data.user_id)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;
        let Some(user) = user else {
            return Ok(HttpResponse::Unauthorized().finish());
        };
        Ok(HttpResponse::Ok().json(MeResponse {
            logged_in: true,
            user_id: Some(data.user_id),
            username: Some(data.username),
            display_name: user.display_name,
        }))
    } else {
        Ok(HttpResponse::Ok().json(MeResponse {
            logged_in: false,
            user_id: None,
            username: None,
            display_name: None,
        }))
    }
}

/// Cookie セッションで行う更新は同一オリジンからの要求に限定する。
fn same_origin(request: &HttpRequest) -> bool {
    let Some(origin) = request
        .headers()
        .get(header::ORIGIN)
        .and_then(|value| value.to_str().ok())
    else {
        return false;
    };
    let Ok(origin) = origin.parse::<actix_web::http::Uri>() else {
        return false;
    };
    let Some(host) = request
        .headers()
        .get(header::HOST)
        .and_then(|value| value.to_str().ok())
    else {
        return false;
    };
    origin.scheme_str() == Some(request.connection_info().scheme())
        && origin.authority().map(|value| value.as_str()) == Some(host)
}

#[put("/api/auth/me/display-name")]
pub async fn put_display_name(
    request: HttpRequest,
    user: AuthenticatedUser,
    pool: web::Data<SqlitePool>,
    payload: web::Json<UpdateDisplayNameRequest>,
) -> Result<HttpResponse, Error> {
    if !same_origin(&request) {
        return Ok(
            HttpResponse::Forbidden().json(serde_json::json!({"message": "不正なオリジンです"}))
        );
    }
    let repo = UserRepositorySqlite {
        pool: pool.get_ref().clone(),
    };
    match update_name(&repo, user.user_id, &payload.display_name).await {
        Ok(true) => {
            let display_name = payload.display_name.trim();
            info!(
                "{}",
                serde_json::json!({
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                    "event": "user_display_name_saved",
                    "user_id": user.user_id,
                    "display_name": display_name,
                })
            );
            Ok(HttpResponse::Ok().json(serde_json::json!({"display_name": display_name})))
        }
        Ok(false) => Ok(HttpResponse::NotFound().finish()),
        Err(message) if message == "表示名を入力してください" => {
            Ok(HttpResponse::BadRequest().json(serde_json::json!({"message": message})))
        }
        Err(error) => {
            warn!(
                "display name update failed: user_id={}, error={}",
                user.user_id, error
            );
            Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"message": "表示名の保存に失敗しました"})))
        }
    }
}

#[cfg(test)]
mod username_normalization_tests {
    use super::*;
    use actix_web::{http::StatusCode, test, App};

    /// 登録・ログイン・重複判定で同じユーザー名の正規化を使うことを確認する。
    #[actix_web::test]
    async fn signup_and_login_use_the_same_trimmed_username() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::query(
            "CREATE TABLE users (id INTEGER PRIMARY KEY, username TEXT NOT NULL UNIQUE, \
             email_hash TEXT NOT NULL UNIQUE, password_hash TEXT NOT NULL, display_name TEXT, \
             created_at TEXT, terminated_at TEXT, updated_at TEXT)",
        )
        .execute(&pool)
        .await
        .unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .wrap(
                    actix_session::SessionMiddleware::builder(
                        actix_session::storage::CookieSessionStore::default(),
                        actix_web::cookie::Key::from(&[0; 64]),
                    )
                    .build(),
                )
                .service(signup)
                .service(login),
        )
        .await;

        let signup_request = SignupRequest {
            username: "  Alice  ".to_string(),
            email: "alice@example.com".to_string(),
            password: "password123".to_string(),
            display_name: None,
        };
        let response = test::call_service(
            &app,
            test::TestRequest::post()
                .uri("/api/auth/signup")
                .set_json(&signup_request)
                .to_request(),
        )
        .await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: AuthResponse = test::read_body_json(response).await;
        assert_eq!(body.username.as_deref(), Some("alice"));
        let (stored_name,): (String,) = sqlx::query_as("SELECT username FROM users")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(stored_name, "alice");

        let login_request = LoginRequest {
            username: " ALICE ".to_string(),
            password: "password123".to_string(),
        };
        let response = test::call_service(
            &app,
            test::TestRequest::post()
                .uri("/api/auth/login")
                .set_json(&login_request)
                .to_request(),
        )
        .await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: AuthResponse = test::read_body_json(response).await;
        assert_eq!(body.username.as_deref(), Some("alice"));

        let response = test::call_service(
            &app,
            test::TestRequest::post()
                .uri("/api/auth/signup")
                .set_json(&SignupRequest {
                    username: " alice ".to_string(),
                    email: "another@example.com".to_string(),
                    password: "password123".to_string(),
                    display_name: None,
                })
                .to_request(),
        )
        .await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}

#[cfg(test)]
mod display_name_tests {
    use super::*;
    use actix_web::{http::StatusCode, test, App};

    #[actix_web::test]
    async fn updates_only_the_session_user_and_keeps_the_session() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::query("CREATE TABLE users (id INTEGER PRIMARY KEY, username TEXT NOT NULL UNIQUE, email_hash TEXT NOT NULL UNIQUE, password_hash TEXT NOT NULL, display_name TEXT, created_at TEXT, terminated_at TEXT, updated_at TEXT)")
            .execute(&pool).await.unwrap();
        let password_hash = hash_password("secret").unwrap();
        for (id, username) in [(1, "alice"), (2, "bob")] {
            sqlx::query("INSERT INTO users (id, username, email_hash, password_hash, display_name) VALUES (?, ?, ?, ?, ?)")
                .bind(id).bind(username).bind(username).bind(&password_hash).bind(username)
                .execute(&pool).await.unwrap();
        }
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .wrap(
                    actix_session::SessionMiddleware::builder(
                        actix_session::storage::CookieSessionStore::default(),
                        actix_web::cookie::Key::from(&[0; 64]),
                    )
                    .build(),
                )
                .service(login)
                .service(me)
                .service(put_display_name),
        )
        .await;

        let response = test::call_service(
            &app,
            test::TestRequest::put()
                .uri("/api/auth/me/display-name")
                .insert_header(("Host", "localhost"))
                .insert_header(("Origin", "http://localhost"))
                .set_json(serde_json::json!({"display_name": "New"}))
                .to_request(),
        )
        .await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let response = test::call_service(
            &app,
            test::TestRequest::post()
                .uri("/api/auth/login")
                .set_json(LoginRequest {
                    username: "alice".into(),
                    password: "secret".into(),
                })
                .to_request(),
        )
        .await;
        assert_eq!(response.status(), StatusCode::OK);
        let cookie = response.response().cookies().next().unwrap().to_owned();

        for value in ["", "  \t  "] {
            let response = test::call_service(
                &app,
                test::TestRequest::put()
                    .uri("/api/auth/me/display-name")
                    .cookie(cookie.clone())
                    .insert_header(("Host", "localhost"))
                    .insert_header(("Origin", "http://localhost"))
                    .set_json(serde_json::json!({"display_name": value}))
                    .to_request(),
            )
            .await;
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        }
        for body in [
            serde_json::json!({"display_name": "Other", "username": "bob"}),
            serde_json::json!({"display_name": "Other", "user_id": 2}),
        ] {
            let response = test::call_service(
                &app,
                test::TestRequest::put()
                    .uri("/api/auth/me/display-name")
                    .cookie(cookie.clone())
                    .insert_header(("Host", "localhost"))
                    .insert_header(("Origin", "http://localhost"))
                    .set_json(body)
                    .to_request(),
            )
            .await;
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        }
        let response = test::call_service(
            &app,
            test::TestRequest::put()
                .uri("/api/auth/me/display-name")
                .cookie(cookie.clone())
                .insert_header(("Host", "localhost"))
                .insert_header(("Origin", "https://evil.example"))
                .set_json(serde_json::json!({"display_name": "Other"}))
                .to_request(),
        )
        .await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        let response = test::call_service(
            &app,
            test::TestRequest::put()
                .uri("/api/auth/me/display-name")
                .cookie(cookie.clone())
                .insert_header(("Host", "localhost"))
                .set_json(serde_json::json!({"display_name": "Other"}))
                .to_request(),
        )
        .await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);

        let response = test::call_service(
            &app,
            test::TestRequest::put()
                .uri("/api/auth/me/display-name")
                .cookie(cookie.clone())
                .insert_header(("Host", "localhost"))
                .insert_header(("Origin", "http://localhost"))
                .set_json(serde_json::json!({"display_name": "  Alice New  "}))
                .to_request(),
        )
        .await;
        assert_eq!(response.status(), StatusCode::OK);

        let response = test::call_service(
            &app,
            test::TestRequest::get()
                .uri("/api/auth/me")
                .cookie(cookie)
                .to_request(),
        )
        .await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: MeResponse = test::read_body_json(response).await;
        assert!(body.logged_in);
        assert_eq!(body.username.as_deref(), Some("alice"));
        assert_eq!(body.display_name.as_deref(), Some("Alice New"));
        let rows: Vec<(String, Option<String>)> =
            sqlx::query_as("SELECT username, display_name FROM users ORDER BY id")
                .fetch_all(&pool)
                .await
                .unwrap();
        assert_eq!(
            rows,
            vec![
                ("alice".into(), Some("Alice New".into())),
                ("bob".into(), Some("bob".into()))
            ]
        );
    }
}
