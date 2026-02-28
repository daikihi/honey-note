use std::ops::Deref;

use actix_web::{FromRequest, HttpRequest, dev::Payload, Error};
use futures_util::StreamExt;
use serde::{de::DeserializeOwned, Serialize};
use bytes::BytesMut;
use chrono::Utc;

pub struct LoggedJson<T>(pub T);

impl<T> Deref for LoggedJson<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T> LoggedJson<T> {
    pub fn into_inner(self) -> T { self.0 }
}

#[derive(Serialize)]
struct PutLog<T> {
    timestamp: String,
    method: String,
    path: String,
    body: T,
}

impl<T> FromRequest for LoggedJson<T>
where
    T: DeserializeOwned + Serialize + 'static,
{
    type Error = Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let method = req.method().to_string();
        let path = req.path().to_string();
        let mut pl = payload.take();
        Box::pin(async move {
            let mut body = BytesMut::new();
            while let Some(chunk) = pl.next().await {
                let chunk = chunk?;
                body.extend_from_slice(&chunk);
            }

            let value = serde_json::from_slice::<T>(&body)
                .map_err(|e| actix_web::error::ErrorBadRequest(e))?;

            let log_entry = PutLog {
                timestamp: Utc::now().to_rfc3339(),
                method,
                path,
                body: &value,
            };

            if let Ok(log_json) = serde_json::to_string(&log_entry) {
                log::info!("{}", log_json);
            }

            Ok(LoggedJson(value))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use actix_web::test;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestBody {
        name: String,
        age: i32,
    }

    #[actix_web::test]
    async fn test_put_log_serialization() {
        let body = TestBody {
            name: "Honey".to_string(),
            age: 5,
        };
        let log_entry = PutLog {
            timestamp: "2026-02-28T09:56:00Z".to_string(),
            method: "PUT".to_string(),
            path: "/api/test".to_string(),
            body: &body,
        };

        let json = serde_json::to_string(&log_entry).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["method"], "PUT");
        assert_eq!(parsed["path"], "/api/test");
        assert_eq!(parsed["timestamp"], "2026-02-28T09:56:00Z");
        assert_eq!(parsed["body"]["name"], "Honey");
        assert_eq!(parsed["body"]["age"], 5);
    }

    #[actix_web::test]
    async fn test_logged_json_from_request() {
        let body = TestBody {
            name: "Alice".to_string(),
            age: 30,
        };
        let payload = serde_json::to_string(&body).unwrap();
        
        let (req, mut pl) = test::TestRequest::put()
            .uri("/test/path")
            .set_payload(payload)
            .to_http_parts();
        
        let res = LoggedJson::<TestBody>::from_request(&req, &mut pl).await;

        assert!(res.is_ok());
        let logged_json = res.unwrap();
        assert_eq!(logged_json.0, body);
    }
}
