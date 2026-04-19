use actix_session::Session;
use actix_web::{get, post, web, Error, HttpResponse};
use common::libs::auth::{hash_email, hash_password, verify_password};
use common::repository::users::{UserRepository, UserRepositorySqlite};
use common_type::models::session::SessionData;
use common_type::models::user::User;
use common_type::request::auth::auth::{AuthResponse, LoginRequest, MeResponse, SignupRequest};
use log::{debug, info, warn};
use sqlx::SqlitePool;
use validator::Validate;

/// Create a masked username string suitable for logging.
///
/// If `username` is empty, returns `"***"`. Otherwise returns the first character
/// of `username` followed by `"***"`.
///
/// # Examples
///
///

/// Handles user signup requests: validates input, ensures username/email uniqueness,
/// creates a new user with hashed password and returns an authentication response.
///
/// On validation failure or if the username/email is already in use, returns a 400 response
/// with an `AuthResponse` describing the error. On internal failures (e.g., password hashing
/// or database insertion errors) returns a 500 response. On success returns 200 with the
/// created user's ID and username.
///
/// # Examples
///
/// ```
/// use actix_web::{test, App};
/// use crate::handlers::signup;
/// use crate::models::SignupRequest;
///
/// #[actix_rt::test]
/// async fn signup_endpoint_accepts_valid_payload() {
///     let app = test::init_service(App::new().service(signup)).await;
///     let payload = SignupRequest {
///         username: "newuser".into(),
///         email: "newuser@example.com".into(),
///         password: "s3cr3tpass".into(),
///         display_name: None,
///     };
///     let req = test::TestRequest::post()
///         .uri("/api/auth/signup")
///         .set_json(&payload)
///         .to_request();
///     let resp = test::call_service(&app, req).await;
///     assert!(resp.status().is_success());
/// }
/// ```
#[post("/api/auth/signup")]
pub async fn signup(
    pool: web::Data<SqlitePool>,
    payload: web::Json<SignupRequest>,
) -> Result<HttpResponse, Error> {
    // バリデーション
    if let Err(e) = payload.validate() {
        warn!("バリデーションエラー: {:?}", e);
        return Ok(HttpResponse::BadRequest().json(AuthResponse {
            success: false,
            message: Some(
                "入力内容に誤りがあります。メールアドレスの形式を確認してください。".to_string(),
            ),
            user_id: None,
            username: None,
        }));
    }

    let repo = UserRepositorySqlite {
        pool: pool.get_ref().clone(),
    };

    // ユーザー名を小文字に正規化
    let username = payload.username.to_lowercase();
    debug!("username: {}", mask_username(&username));

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

    let new_user = User {
        id: None,
        username: username.clone(),
        email_hash,
        password_hash,
        display_name: payload
            .display_name
            .clone()
            .or_else(|| Some(username.clone())),
        created_at: None,
        terminated_at: None,
        updated_at: None,
    };

    match repo.insert_user(new_user).await {
        Ok(id) => {
            info!("user signup success: username={}", mask_username(&username));
            Ok(HttpResponse::Ok().json(AuthResponse {
                success: true,
                message: None,
                user_id: Some(id as i32),
                username: Some(username),
            }))
        }
        Err(e) => {
            warn!("user signup failed: username={}, error={}", mask_username(&username), e);
            Ok(HttpResponse::InternalServerError().json(AuthResponse {
                success: false,
                message: Some("ユーザー登録に失敗しました".to_string()),
                user_id: None,
                username: None,
            }))
        }
    }
}

/// Handles user login by validating credentials, creating a server session on success, and returning an authentication response.
///
/// Attempts to find a user by the provided username (case-insensitive) and verify the provided password. On successful verification a new session is created and an `AuthResponse` with `success: true`, the `user_id`, and the original `username` is returned. If the username is not found or the password is incorrect, an `AuthResponse` with `success: false` and an unauthorized status is returned. On repository or verification errors, an internal server error response is returned.
///
/// # Returns
///
/// An `AuthResponse` wrapped in an `HttpResponse`:
/// - `success: true` with `user_id` and `username` when credentials are valid.
/// - `success: false` with an appropriate message and no `user_id`/`username` when credentials are invalid or on error.
pub async fn login(
    pool: web::Data<SqlitePool>,
    payload: web::Json<LoginRequest>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let repo = UserRepositorySqlite {
        pool: pool.get_ref().clone(),
    };
    let username = payload.username.to_lowercase();

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

                info!("login success username={}", mask_username(&username));
                return Ok(HttpResponse::Ok().json(AuthResponse {
                    success: true,
                    message: None,
                    user_id: Some(user_id),
                    username: Some(username),
                }));
            }
            Ok(false) => {
                warn!("login failed username={}", mask_username(&username));
                return Ok(HttpResponse::Unauthorized().json(AuthResponse {
                    success: false,
                    message: Some("ユーザー名またはパスワードが正しくありません".to_string()),
                    user_id: None,
                    username: None,
                }));
            }
            Err(e) => {
                log::error!("bcrypt error during password verification: {}", e);
                return Ok(HttpResponse::InternalServerError().json(AuthResponse {
                    success: false,
                    message: Some("サーバー内部エラーが発生しました".to_string()),
                    user_id: None,
                    username: None,
                }));
            }
        }
    }

    warn!("login failed username={}", mask_username(&username));
    Ok(HttpResponse::Unauthorized().json(AuthResponse {
        success: false,
        message: Some("ユーザー名またはパスワードが正しくありません".to_string()),
        user_id: None,
        username: None,
    }))
}

/// Logs out the current user by purging the session and returning a success response.
///
/// # Returns
/// An `HttpResponse` with a JSON `AuthResponse` indicating success and containing the message `"ログアウトしました"`.
///
/// # Examples
///
/// ```rust,no_run
/// // In an Actix handler or test context with a valid `Session`:
/// let resp = logout(session).await.unwrap();
/// assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
/// ```
#[post("/api/auth/logout")]
pub async fn logout(session: Session) -> Result<HttpResponse, Error> {
    if let Ok(Some(data)) = session.get::<SessionData>("user") {
        info!("logout username={}", mask_username(&data.username));
    }
    session.purge();
    Ok(HttpResponse::Ok().json(AuthResponse {
        success: true,
        message: Some("ログアウトしました".to_string()),
        user_id: None,
        username: None,
    }))
}

/// Returns the current authentication state based on the session.
///
/// If the session contains `SessionData` under the `"user"` key, responds with HTTP 200 and a JSON
/// `MeResponse` where `logged_in` is `true` and `user_id` / `username` are populated from the
/// session. If no session data is present, responds with HTTP 200 and a JSON `MeResponse` where
/// `logged_in` is `false` and `user_id` / `username` are `None`.
///
/// # Examples
///
/// ```
/// use crate::models::MeResponse;
///
/// // When not logged in
/// let not_logged = MeResponse { logged_in: false, user_id: None, username: None };
/// assert_eq!(not_logged.logged_in, false);
///
/// // When logged in
/// let logged = MeResponse { logged_in: true, user_id: Some(1), username: Some("alice".to_string()) };
/// assert_eq!(logged.logged_in, true);
/// assert_eq!(logged.user_id, Some(1));
/// assert_eq!(logged.username.as_deref(), Some("alice"));
/// ```
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use sqlx::SqlitePool;

    /// Creates an in-memory SQLite database prepopulated with the `users` table and returns a connection pool.
    ///
    /// The returned pool is connected to a temporary in-memory database whose `users` table matches the application's schema
    /// (columns: `id`, `username`, `email_hash`, `password_hash`, `display_name`, `created_at`, `terminated_at`, `updated_at`)
    /// with unique constraints on `username` and `email_hash`.
    ///
    /// # Returns
    ///
    /// A `SqlitePool` connected to the initialized in-memory database.
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[tokio::test]
    /// async fn example_setup_db() {
    ///     let pool = crate::setup_db().await;
    ///     // table exists and starts empty
    ///     let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
    ///         .fetch_one(&pool)
    ///         .await
    ///         .unwrap();
    ///     assert_eq!(count, 0);
    /// }
    /// ```
    async fn setup_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::query(
            "CREATE TABLE users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL,
                email_hash TEXT NOT NULL,
                password_hash TEXT NOT NULL,
                display_name TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                terminated_at DATETIME,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(username),
                UNIQUE(email_hash)
            );",
        )
        .execute(&pool)
        .await
        .unwrap();
        pool
    }

    #[actix_web::test]
    async fn test_signup_success() {
        let pool = setup_db().await;
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
                .service(signup),
        )
        .await;

        let payload = SignupRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            display_name: None,
        };

        let req = test::TestRequest::post()
            .uri("/api/auth/signup")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body: AuthResponse = test::read_body_json(resp).await;
        assert!(body.success);
        assert!(body.user_id.is_some());
        assert_eq!(body.username, Some("testuser".to_string()));
    }

    #[actix_web::test]
    async fn test_signup_invalid_email() {
        let pool = setup_db().await;
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
                .service(signup),
        )
        .await;

        let payload = SignupRequest {
            username: "testuser".to_string(),
            email: "invalid-email".to_string(),
            password: "password123".to_string(),
            display_name: None,
        };

        let req = test::TestRequest::post()
            .uri("/api/auth/signup")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);

        let body: AuthResponse = test::read_body_json(resp).await;
        assert!(!body.success);
    }

    #[actix_web::test]
    async fn test_signup_duplicate_username() {
        let pool = setup_db().await;
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
                .service(signup),
        )
        .await;

        let payload = SignupRequest {
            username: "testuser".to_string(),
            email: "test1@example.com".to_string(),
            password: "password123".to_string(),
            display_name: None,
        };

        let req1 = test::TestRequest::post()
            .uri("/api/auth/signup")
            .set_json(&payload)
            .to_request();
        let resp1 = test::call_service(&app, req1).await;
        assert!(resp1.status().is_success());

        let payload2 = SignupRequest {
            username: "testuser".to_string(),
            email: "test2@example.com".to_string(),
            password: "password123".to_string(),
            display_name: None,
        };

        let req2 = test::TestRequest::post()
            .uri("/api/auth/signup")
            .set_json(&payload2)
            .to_request();
        let resp2 = test::call_service(&app, req2).await;
        assert_eq!(resp2.status(), actix_web::http::StatusCode::BAD_REQUEST);

        let body: AuthResponse = test::read_body_json(resp2).await;
        assert!(!body.success);
        assert!(body.message.unwrap().contains("ユーザー名は既に使用されています"));
    }

    #[actix_web::test]
    async fn test_signup_duplicate_email_hash() {
        let pool = setup_db().await;
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
                .service(signup),
        )
        .await;

        let payload = SignupRequest {
            username: "testuser1".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            display_name: None,
        };

        let req1 = test::TestRequest::post()
            .uri("/api/auth/signup")
            .set_json(&payload)
            .to_request();
        let resp1 = test::call_service(&app, req1).await;
        assert!(resp1.status().is_success());

        let payload2 = SignupRequest {
            username: "testuser2".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            display_name: None,
        };

        let req2 = test::TestRequest::post()
            .uri("/api/auth/signup")
            .set_json(&payload2)
            .to_request();
        let resp2 = test::call_service(&app, req2).await;
        assert_eq!(resp2.status(), actix_web::http::StatusCode::BAD_REQUEST);

        let body: AuthResponse = test::read_body_json(resp2).await;
        assert!(!body.success);
        assert!(body.message.unwrap().contains("メールアドレスは既に登録されています"));
    }

    #[actix_web::test]
    async fn test_login_success() {
        let pool = setup_db().await;
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

        let signup_payload = SignupRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            display_name: None,
        };

        let req_signup = test::TestRequest::post()
            .uri("/api/auth/signup")
            .set_json(&signup_payload)
            .to_request();
        let resp_signup = test::call_service(&app, req_signup).await;
        assert!(resp_signup.status().is_success());

        let login_payload = LoginRequest {
            username: "testuser".to_string(),
            password: "password123".to_string(),
        };

        let req_login = test::TestRequest::post()
            .uri("/api/auth/login")
            .set_json(&login_payload)
            .to_request();
        let resp_login = test::call_service(&app, req_login).await;
        assert!(resp_login.status().is_success());

        let body: AuthResponse = test::read_body_json(resp_login).await;
        assert!(body.success);
        assert!(body.user_id.is_some());
    }

    /// Verifies that attempting to log in with an incorrect password returns 401 Unauthorized.
    ///
    /// This test signs up a user, attempts to log in with a wrong password, and asserts the response
    /// status is `UNAUTHORIZED` and the returned `AuthResponse.success` is `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// // This example mirrors the test: sign up a user, then attempt login with wrong password
    /// // and assert that authentication fails.
    /// # tokio_test::block_on(async {
    /// use actix_web::{test, App, web};
    /// use actix_session::SessionMiddleware;
    /// use actix_session::storage::CookieSessionStore;
    /// use actix_web::cookie::Key;
    ///
    /// // setup_db, signup, login, SignupRequest, LoginRequest, AuthResponse are defined in the crate
    /// let pool = setup_db().await;
    /// let app = test::init_service(
    ///     App::new()
    ///         .app_data(web::Data::new(pool.clone()))
    ///         .wrap(SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0;64])).build())
    ///         .service(signup)
    ///         .service(login),
    /// ).await;
    ///
    /// let signup_payload = SignupRequest {
    ///     username: "testuser".to_string(),
    ///     email: "test@example.com".to_string(),
    ///     password: "password123".to_string(),
    ///     display_name: None,
    /// };
    /// let req_signup = test::TestRequest::post().uri("/api/auth/signup").set_json(&signup_payload).to_request();
    /// let resp_signup = test::call_service(&app, req_signup).await;
    /// assert!(resp_signup.status().is_success());
    ///
    /// let login_payload = LoginRequest { username: "testuser".to_string(), password: "wrongpassword".to_string() };
    /// let req_login = test::TestRequest::post().uri("/api/auth/login").set_json(&login_payload).to_request();
    /// let resp_login = test::call_service(&app, req_login).await;
    /// assert_eq!(resp_login.status(), actix_web::http::StatusCode::UNAUTHORIZED);
    /// let body: AuthResponse = test::read_body_json(resp_login).await;
    /// assert!(!body.success);
    /// # });
    /// ```
    #[actix_web::test]
    async fn test_login_incorrect_password() {
        let pool = setup_db().await;
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

        let signup_payload = SignupRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            display_name: None,
        };

        let req_signup = test::TestRequest::post()
            .uri("/api/auth/signup")
            .set_json(&signup_payload)
            .to_request();
        let resp_signup = test::call_service(&app, req_signup).await;
        assert!(resp_signup.status().is_success());

        let login_payload = LoginRequest {
            username: "testuser".to_string(),
            password: "wrongpassword".to_string(),
        };

        let req_login = test::TestRequest::post()
            .uri("/api/auth/login")
            .set_json(&login_payload)
            .to_request();
        let resp_login = test::call_service(&app, req_login).await;
        assert_eq!(resp_login.status(), actix_web::http::StatusCode::UNAUTHORIZED);

        let body: AuthResponse = test::read_body_json(resp_login).await;
        assert!(!body.success);
    }

    /// Ensures the logout endpoint responds successfully and the returned `AuthResponse` indicates success.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example(pool: sqlx::SqlitePool) {
    /// use actix_web::{test, App, web};
    /// use actix_session::SessionMiddleware;
    /// use actix_session::storage::CookieSessionStore;
    /// use actix_web::cookie::Key;
    ///
    /// let app = test::init_service(
    ///     App::new()
    ///         .app_data(web::Data::new(pool))
    ///         .wrap(SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0;64])).build())
    ///         .service(crate::auth::logout),
    /// ).await;
    ///
    /// let req = test::TestRequest::post().uri("/api/auth/logout").to_request();
    /// let resp = test::call_service(&app, req).await;
    /// assert!(resp.status().is_success());
    /// let body: crate::auth::AuthResponse = test::read_body_json(resp).await;
    /// assert!(body.success);
    /// # }
    /// ```
    #[actix_web::test]
    async fn test_logout() {
        let pool = setup_db().await;
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
                .service(logout),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/auth/logout")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body: AuthResponse = test::read_body_json(resp).await;
        assert!(body.success);
    }

    #[actix_web::test]
    async fn test_me_not_logged_in() {
        let pool = setup_db().await;
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
                .service(me),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/auth/me")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body: MeResponse = test::read_body_json(resp).await;
        assert!(!body.logged_in);
        assert!(body.user_id.is_none());
        assert!(body.username.is_none());
    }
}