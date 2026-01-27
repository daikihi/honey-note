use serde::{Serialize, Deserialize};
use chrono::{DateTime, FixedOffset};
use super::color_feature::ColorFeature;
use super::observation_input::ObservationInput;
use super::honey_detail_types::{AromaIntensity, AromaType, AromaNote, SweetnessIntensity, Acidity, Mouthfeel, Finish, TasteNote, CrystallizationLevel, CrystalTexture, Usage, Tags, Memo};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoneyDetailDynamic {
    pub color_feature: Option<ColorFeature>,
    pub aroma_intensity: Option<AromaIntensity>,
    pub aroma_type: Option<AromaType>,
    pub aroma_note: Option<AromaNote>,
    pub sweetness_intensity: Option<SweetnessIntensity>,
    pub acidity: Option<Acidity>,
    pub mouthfeel: Option<Mouthfeel>,
    pub finish: Option<Finish>,
    pub taste_note: Option<TasteNote>,
    pub crystallization_level: Option<CrystallizationLevel>,
    pub crystal_texture: Option<CrystalTexture>,
    pub preference: Option<u8>,
    pub usage: Option<Usage>,
    pub tags: Option<Tags>,
    pub observations: Vec<ObservationInput>,
    pub memo: Option<Memo>,
    pub created_at: Option<DateTime<FixedOffset>>,
    pub updated_at: Option<DateTime<FixedOffset>>,
}
