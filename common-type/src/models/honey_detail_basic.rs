use serde::{Serialize, Deserialize};
use chrono::{DateTime, FixedOffset};
use super::honey_detail_types::{HoneyNameJp, BeekeeperName, Country, Region, FlowerName, HoneyType, Volume};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoneyDetailBasic {
    pub name_jp: HoneyNameJp,
    pub beekeeper_name: Option<BeekeeperName>,
    pub harvest_year: Option<i32>,
    pub country: Option<Country>,
    pub region: Option<Region>,
    pub flower_names: Vec<FlowerName>,
    pub honey_type: Option<HoneyType>, // "単花蜜" or "百花蜜"
    pub volume: Option<Volume>,
    pub purchase_date: Option<DateTime<FixedOffset>>,
}
