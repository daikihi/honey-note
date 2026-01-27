use common_type::models::honey::Honey;

pub struct GetAllHoneysRequestDto {}

pub struct GetAllHoneysResponseDto {
    pub honeys: Vec<Honey>,
}
