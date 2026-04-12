use actix_session::SessionExt;
use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use common_type::models::session::SessionData;
use std::future::{ready, Ready};

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
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let session = req.get_session();

        match session.get::<SessionData>("user") {
            Ok(Some(data)) => {
                ready(Ok(AuthenticatedUser {
                    user_id: data.user_id,
                    username: data.username,
                }))
            }
            Ok(None) => {
                // セッションが存在しない場合
                ready(Err(actix_web::error::ErrorUnauthorized("ログインが必要です")))
            }
            Err(e) => {
                // セッションの読み込みに失敗した場合
                ready(Err(actix_web::error::ErrorInternalServerError(e)))
            }
        }
    }
}
