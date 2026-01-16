// HoneyInput モデル（新規登録・編集用）
// このファイルはフォーム入力値を受け取るためのモデルです。

use serde::{Deserialize, Serialize};
use chrono::{DateTime, FixedOffset};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoneyNameJp(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorFeature {
    pub category: Option<String>,
    pub hex: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoneyInputBasic {
    pub name_jp: HoneyNameJp,
    pub beekeeper_name: Option<String>,
    pub harvest_year: Option<i32>,
    pub country: Option<String>,
    pub region: Option<String>,
    pub flower_names: Vec<String>,
    pub honey_type: Option<String>, // "単花蜜" or "百花蜜"
    pub volume: Option<String>,
    pub purchase_date: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoneyInputDynamic {
    pub color_feature: Option<ColorFeature>,
    pub aroma_intensity: Option<String>,
    pub aroma_type: Option<String>,
    pub aroma_note: Option<String>,
    pub sweetness_intensity: Option<String>,
    pub acidity: Option<String>,
    pub mouthfeel: Option<String>,
    pub finish: Option<String>,
    pub taste_note: Option<String>,
    pub crystallization_level: Option<String>,
    pub crystal_texture: Option<String>,
    // 管理・評価
    pub preference: Option<u8>,
    pub usage: Option<String>,
    pub tags: Option<String>,
    // 観測記録
    pub observations: Vec<ObservationInput>,
    // メモ
    pub memo: Option<String>,
    pub created_at: Option<DateTime<FixedOffset>>,
    pub updated_at: Option<DateTime<FixedOffset>>,
    // 風味・特徴

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationInput {
    pub date: Option<DateTime<FixedOffset>>,
    pub state: Option<String>,
    pub taste: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoneyInput {
    pub basic: HoneyInputBasic,
    pub dynamic: Vec<HoneyInputDynamic>,
    pub created_at: Option<DateTime<FixedOffset>>,
    pub updated_at: Option<DateTime<FixedOffset>>,
}
