use serde::{Serialize, Deserialize};
use chrono::{DateTime, FixedOffset};
use super::basic::HoneyEditBasicRequest;
use super::dynamic::HoneyEditDynamicRequest;
use crate::models::honey_input::HoneyInput;

/// はちみつ新規作成リクエストDTO
///
/// クライアントからサーバーへ、新規はちみつデータの作成内容を送信する際のJsonリクエスト型。
/// - basic: 基本情報部分
/// - dynamic: 動的情報部分（複数）
/// - created_at: 作成リクエスト時刻
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoneyNewRequest {
    /// 基本情報部分
    pub basic: HoneyEditBasicRequest,
    /// 動的情報部分（複数）
    pub dynamic: Vec<HoneyEditDynamicRequest>,
    /// 作成リクエスト時刻
    pub created_at: Option<DateTime<FixedOffset>>,
}

impl HoneyNewRequest {
    /// HoneyNewRequest → モデル型への変換
    pub fn to_honey_input(&self) -> HoneyInput {
        HoneyInput {
            basic: self.basic.to_honey_input_basic(),
            dynamic: self.dynamic.iter().map(|d| d.to_honey_input_dynamic()).collect(),
            created_at: self.created_at,
            updated_at: None,
        }
    }
    /// モデル型 → HoneyNewRequest への変換
    pub fn from_honey_input(input: &HoneyInput) -> Self {
        HoneyNewRequest {
            basic: HoneyEditBasicRequest::from_honey_input_basic(&input.basic),
            dynamic: input.dynamic.iter().map(|d| HoneyEditDynamicRequest::from_honey_input_dynamic(d)).collect(),
            created_at: input.created_at,
        }
    }
}
