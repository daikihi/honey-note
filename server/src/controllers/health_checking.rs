use actix_web::{get, Responder};

#[get("/health")]
pub async fn health_check() -> impl Responder {
    "OK"
}