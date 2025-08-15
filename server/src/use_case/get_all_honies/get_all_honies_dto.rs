use common_type::models::honey::Honey;

pub struct GetAllHoneysRequestDto {
    pub pool: sqlx::SqlitePool,
}

pub struct GetAllHoneysResponseDto {
    pub honeys: Vec<Honey>,
}
