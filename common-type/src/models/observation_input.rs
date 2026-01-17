// 観察記録の入力値を表す構造体
// 1つのはちみつに対する観察（テイスティングや状態記録など）を格納する用途

use serde::{Serialize, Deserialize};
use chrono::{DateTime, FixedOffset};
pub(crate) use super::honey_detail_types::{State, Taste, ObservationNote};

/// 観察記録の入力値
///
/// 1回の観察（テイスティングや状態記録）を表す。
/// - date: 観察日（任意）
/// - state: 観察時の状態（例: 液状/結晶化など）
/// - taste: 観察時の味（例: まろやか/さっぱり等）
/// - note: 観察時の補足ノート
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationInput {
    /// 観察日（任意）
    pub date: Option<DateTime<FixedOffset>>,
    /// 観察時の状態（例: 液状/結晶化など）
    pub state: Option<State>,
    /// 観察時の味（例: まろやか/さっぱり等）
    pub taste: Option<Taste>,
    /// 観察時の補足ノート
    pub note: Option<ObservationNote>,
}
