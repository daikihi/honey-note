//! はちみつ編集リクエスト用の動的情報DTO
//!
//! このファイルは、クライアントからサーバーへのはちみつ編集リクエストのうち、
//! 色・香り・味・観察記録などの動的情報部分をJsonで受け取るための型を定義します。
//!
//! - 用途: HTTPリクエストのJsonボディのバリデーション・型安全化
//! - 変換: to_honey_input_dynamic, from_honey_input_dynamic で内部モデルと相互変換

use serde::{Serialize, Deserialize};
use crate::models::honey_input_dynamic::HoneyInputDynamic;
use crate::models::color_feature as model_color_feature;
use crate::models::observation_input as model_observation_input;

/// 色特徴（カテゴリ・16進色・補足説明をStringで保持）
///
/// - category: 色カテゴリ（例: 明色/中間色/暗色など）
/// - hex: 色の16進表現（例: #ffeeaa）
/// - note: 色に関する補足説明
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorFeature {
    /// 色カテゴリ（例: 明色/中間色/暗色など）
    pub category: Option<String>,
    /// 色の16進表現（例: #ffeeaa）
    pub hex: Option<String>,
    /// 色に関する補足説明
    pub note: Option<String>,
}

/// 観察記録リクエストDTO（すべてString型で保持）
///
/// - date: 観察日（文字列）
/// - state: 観察時の状態（文字列）
/// - taste: 観察時の味（文字列）
/// - note: 観察時の補足ノート（文字列）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationInputRequest {
    /// 観察日（例: "2024-01-01" など）
    pub date: Option<String>,
    /// 観察時の状態
    pub state: Option<String>,
    /// 観察時の味
    pub taste: Option<String>,
    /// 観察時の補足ノート
    pub note: Option<String>,
}

/// はちみつ編集リクエストの動的情報DTO
///
/// - color_feature: 色特徴
/// - aroma_intensity: 香りの強さ
/// - aroma_type: 香りのタイプ
/// - aroma_note: 香りの詳細ノート
/// - sweetness_intensity: 甘さの強さ
/// - acidity: 酸味の強さ
/// - mouthfeel: 口当たり
/// - finish: 余韻
/// - taste_note: 味の詳細ノート
/// - crystallization_level: 結晶化の度合い
/// - crystal_texture: 結晶の質感
/// - preference: 好み（0-10など）
/// - usage: 主な用途
/// - tags: タグ
/// - observations: 観察記録リスト（ObservationInputRequest型）
/// - memo: メモ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoneyEditDynamicRequest {
    /// 色特徴
    pub color_feature: Option<ColorFeature>,
    /// 香りの強さ
    pub aroma_intensity: Option<String>,
    /// 香りのタイプ
    pub aroma_type: Option<String>,
    /// 香りの詳細ノート
    pub aroma_note: Option<String>,
    /// 甘さの強さ
    pub sweetness_intensity: Option<String>,
    /// 酸味の強さ
    pub acidity: Option<String>,
    /// 口当たり
    pub mouthfeel: Option<String>,
    /// 余韻
    pub finish: Option<String>,
    /// 味の詳細ノート
    pub taste_note: Option<String>,
    /// 結晶化の度合い
    pub crystallization_level: Option<String>,
    /// 結晶の質感
    pub crystal_texture: Option<String>,
    /// 好み（0-10など）
    pub preference: Option<u8>,
    /// 主な用途
    pub usage: Option<String>,
    /// タグ
    pub tags: Option<String>,
    /// 観察記録リスト（ObservationInputRequest型）
    pub observations: Vec<ObservationInputRequest>,
    /// メモ
    pub memo: Option<String>,
}

impl ColorFeature {
    /// ColorFeature (String型) → モデル型への変換
    pub fn to_model(&self) -> model_color_feature::ColorFeature {
        model_color_feature::ColorFeature {
            category: self.category.clone().map(|s| model_color_feature::Category(s)),
            hex: self.hex.clone().map(|s| model_color_feature::Hex(s)),
            note: self.note.clone().map(|s| model_color_feature::ColorNote(s)),
        }
    }
    /// モデル型 → ColorFeature (String型) への変換
    pub fn from_model(model: &model_color_feature::ColorFeature) -> Self {
        ColorFeature {
            category: model.category.as_ref().map(|c| c.0.clone()),
            hex: model.hex.as_ref().map(|h| h.0.clone()),
            note: model.note.as_ref().map(|n| n.0.clone()),
        }
    }
}

impl ObservationInputRequest {
    /// ObservationInputRequest (String型) → モデル型への変換
    pub fn to_model(&self) -> model_observation_input::ObservationInput {
        model_observation_input::ObservationInput {
            date: self.date.as_ref().and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok()),
            state: self.state.clone().map(|s| model_observation_input::State(s)),
            taste: self.taste.clone().map(|s| model_observation_input::Taste(s)),
            note: self.note.clone().map(|s| model_observation_input::ObservationNote(s)),
        }
    }
    /// モデル型 → ObservationInputRequest (String型) への変換
    pub fn from_model(model: &model_observation_input::ObservationInput) -> Self {
        ObservationInputRequest {
            date: model.date.as_ref().map(|d| d.to_rfc3339()),
            state: model.state.as_ref().map(|s| s.0.clone()),
            taste: model.taste.as_ref().map(|t| t.0.clone()),
            note: model.note.as_ref().map(|n| n.0.clone()),
        }
    }
}

impl HoneyEditDynamicRequest {
    /// HoneyEditDynamicRequest → モデル型への変換
    pub fn to_honey_input_dynamic(&self) -> HoneyInputDynamic {
        HoneyInputDynamic {
            color_feature: self.color_feature.as_ref().map(|cf| cf.to_model()),
            aroma_intensity: self.aroma_intensity.clone().map(|s| crate::models::honey_input_types::AromaIntensity(s)),
            aroma_type: self.aroma_type.clone().map(|s| crate::models::honey_input_types::AromaType(s)),
            aroma_note: self.aroma_note.clone().map(|s| crate::models::honey_input_types::AromaNote(s)),
            sweetness_intensity: self.sweetness_intensity.clone().map(|s| crate::models::honey_input_types::SweetnessIntensity(s)),
            acidity: self.acidity.clone().map(|s| crate::models::honey_input_types::Acidity(s)),
            mouthfeel: self.mouthfeel.clone().map(|s| crate::models::honey_input_types::Mouthfeel(s)),
            finish: self.finish.clone().map(|s| crate::models::honey_input_types::Finish(s)),
            taste_note: self.taste_note.clone().map(|s| crate::models::honey_input_types::TasteNote(s)),
            crystallization_level: self.crystallization_level.clone().map(|s| crate::models::honey_input_types::CrystallizationLevel(s)),
            crystal_texture: self.crystal_texture.clone().map(|s| crate::models::honey_input_types::CrystalTexture(s)),
            preference: self.preference,
            usage: self.usage.clone().map(|s| crate::models::honey_input_types::Usage(s)),
            tags: self.tags.clone().map(|s| crate::models::honey_input_types::Tags(s)),
            observations: self.observations.iter().map(|o| o.to_model()).collect(),
            memo: self.memo.clone().map(|s| crate::models::honey_input_types::Memo(s)),
            created_at: None,
            updated_at: None,
        }
    }
    /// モデル型 → HoneyEditDynamicRequest への変換
    pub fn from_honey_input_dynamic(model: &HoneyInputDynamic) -> Self {
        HoneyEditDynamicRequest {
            color_feature: model.color_feature.as_ref().map(|cf| ColorFeature::from_model(cf)),
            aroma_intensity: model.aroma_intensity.as_ref().map(|a| a.0.clone()),
            aroma_type: model.aroma_type.as_ref().map(|a| a.0.clone()),
            aroma_note: model.aroma_note.as_ref().map(|a| a.0.clone()),
            sweetness_intensity: model.sweetness_intensity.as_ref().map(|s| s.0.clone()),
            acidity: model.acidity.as_ref().map(|a| a.0.clone()),
            mouthfeel: model.mouthfeel.as_ref().map(|m| m.0.clone()),
            finish: model.finish.as_ref().map(|f| f.0.clone()),
            taste_note: model.taste_note.as_ref().map(|t| t.0.clone()),
            crystallization_level: model.crystallization_level.as_ref().map(|c| c.0.clone()),
            crystal_texture: model.crystal_texture.as_ref().map(|c| c.0.clone()),
            preference: model.preference,
            usage: model.usage.as_ref().map(|u| u.0.clone()),
            tags: model.tags.as_ref().map(|t| t.0.clone()),
            observations: model.observations.iter().map(|o| ObservationInputRequest::from_model(o)).collect(),
            memo: model.memo.as_ref().map(|m| m.0.clone()),
        }
    }
}

