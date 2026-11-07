use actix_session::SessionExt;
use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use common::repository::users::{UserRepository, UserRepositorySqlite};
use common_type::models::session::SessionData;
use futures_util::future::LocalBoxFuture;
use sqlx::SqlitePool;

/// 認証済みユーザー情報を抽出するための抽出子
///
/// Controller の引数に `AuthenticatedUser` を指定することで、
/// セッションから自動的にユーザー情報を取得できます。
/// 未ログインの場合は 401 Unauthorized を返します。
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: i32,
    pub username: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let session = req.get_session();
        let pool = req.app_data::<actix_web::web::Data<SqlitePool>>().cloned();

        Box::pin(async move {
            let data = match session.get::<SessionData>("user") {
                Ok(Some(data)) => data,
                Ok(None) => {
                    return Err(actix_web::error::ErrorUnauthorized("ログインが必要です"));
                }
                Err(error) => return Err(actix_web::error::ErrorInternalServerError(error)),
            };
            let Some(pool) = pool else {
                return Err(actix_web::error::ErrorInternalServerError(
                    "データベース接続が設定されていません",
                ));
            };
            let repo = UserRepositorySqlite {
                pool: pool.get_ref().clone(),
            };
            match repo.find_by_id(data.user_id).await {
                Ok(Some(_)) => Ok(AuthenticatedUser {
                    user_id: data.user_id,
                    username: data.username,
                }),
                Ok(None) => {
                    session.purge();
                    Err(actix_web::error::ErrorUnauthorized("ログインが必要です"))
                }
                Err(error) => Err(actix_web::error::ErrorInternalServerError(error)),
            }
        })
    }
}
