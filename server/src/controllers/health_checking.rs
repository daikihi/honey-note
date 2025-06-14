use actix_web::{get, Responder};
use log::info;

#[get("/health")]
pub async fn health_check() -> impl Responder {
    info!("Health check endpoint hit");
    "OK"
}
