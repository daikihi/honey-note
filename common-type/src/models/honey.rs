pub mod front_app_model;

use serde::{Deserialize, Serialize};

use crate::models::beekeeper::Beekeeper;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Honey {
    pub id: Option<i32>,
    pub name_jp: String,
    pub name_en: Option<String>,
    pub beekkeeper: Option<Beekeeper>, // 外部キーとしてBeekeeperを参照
    pub origin_country: Option<String>,
    pub origin_region: Option<String>,
    pub harvest_year: Option<i32>,
    pub purchase_date: Option<String>,
    pub note: Option<String>,
}
