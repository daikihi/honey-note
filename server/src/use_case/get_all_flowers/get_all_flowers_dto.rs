use common::models::flowers::Flower;

pub struct get_all_flowers_request_dto {
    pub pool: sqlx::SqlitePool,
}

#[derive()]
pub struct get_all_flowers_response_dto {
    pub flowers: Vec<Flower>,
}
