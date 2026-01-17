use serde::{Serialize, Deserialize};
pub(crate) use super::honey_detail_types::{Category, Hex, ColorNote};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorFeature {
    pub category: Option<Category>,
    pub hex: Option<Hex>,
    pub note: Option<ColorNote>,
}
