use common_type::models::honey::front_app_model::HoneyDetail;

#[derive()]
pub struct GetHoneyDetailsRequestDto {
    pub honey_id: i32,
    pub pool: sqlx::SqlitePool,
}

#[derive()]
pub struct GetHoneyDetailsResponseDto {
    pub honey_detail: Option<HoneyDetail>,
}
