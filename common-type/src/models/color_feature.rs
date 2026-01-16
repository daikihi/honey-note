use serde::{Serialize, Deserialize};
use super::honey_input_types::{Category, Hex, ColorNote};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorFeature {
    pub category: Option<Category>,
    pub hex: Option<Hex>,
    pub note: Option<ColorNote>,
}
