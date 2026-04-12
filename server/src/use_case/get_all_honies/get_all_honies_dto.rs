use common_type::models::honey::Honey;

pub struct GetAllHoneysRequestDto {}

#[derive(Clone)]
pub struct GetAllHoneysResponseDto {
    pub honeys: Vec<Honey>,
}
