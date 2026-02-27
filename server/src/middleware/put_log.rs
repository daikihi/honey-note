use std::ops::Deref;

use actix_web::{FromRequest, HttpRequest, dev::Payload, Error};
use futures_util::StreamExt as _;
use serde::de::DeserializeOwned;
use bytes::BytesMut;

pub struct LoggedJson<T>(pub T);

impl<T> Deref for LoggedJson<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T> LoggedJson<T> {
    pub fn into_inner(self) -> T { self.0 }
}

impl<T> FromRequest for LoggedJson<T>
where
    T: DeserializeOwned + 'static,
{
    type Error = Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let req_info = format!("{} {}", req.method(), req.path());
        let mut pl = payload.take();
        Box::pin(async move {
            let mut body = BytesMut::new();
            while let Some(chunk) = pl.next().await {
                let chunk = chunk?;
                body.extend_from_slice(&chunk);
            }
            let body_str = String::from_utf8_lossy(&body);
            log::info!("request body captured for {} => {}", req_info, body_str);

            let value = serde_json::from_slice::<T>(&body)
                .map_err(|e| actix_web::error::ErrorBadRequest(e))?;
            Ok(LoggedJson(value))
        })
    }
}
