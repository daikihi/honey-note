//! はちみつ編集リクエストDTOのエントリポイント
//!
//! このファイルは、クライアントからサーバーへのはちみつ編集リクエスト全体（基本情報＋動的情報群）をJsonで受け取るための型を定義します。
//!
//! - 用途: HTTPリクエストのJsonボディのバリデーション・型安全化
//! - 変換: to_honey_input, from_honey_input で内部モデルと相互変換
//!
//! 各フィールド・型には用途説明コメントを付与しています。

use serde::{Serialize, Deserialize};
use chrono::{DateTime, FixedOffset};
use super::basic::HoneyEditBasicRequest;
use super::dynamic::HoneyEditDynamicRequest;
use crate::models::honey_input::HoneyInput;

/// はちみつ編集リクエストDTO
///
/// クライアントからサーバーへ、既存はちみつデータの編集内容を送信する際のJsonリクエスト型。
/// - id: 編集対象のはちみつID
/// - basic: 基本情報部分
/// - dynamic: 動的情報部分（複数）
/// - updated_at: 編集リクエスト時刻
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoneyEditRequest {
    /// 編集対象のはちみつID
    pub id: i64,
    /// 基本情報部分
    pub basic: HoneyEditBasicRequest,
    /// 動的情報部分（複数）
    pub dynamic: Vec<HoneyEditDynamicRequest>,
    /// 編集リクエスト時刻
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl HoneyEditRequest {
    /// HoneyEditRequest → モデル型への変換
    pub fn to_honey_input(&self) -> HoneyInput {
        HoneyInput {
            basic: self.basic.to_honey_input_basic(),
            dynamic: self.dynamic.iter().map(|d| d.to_honey_input_dynamic()).collect(),
            created_at: None,
            updated_at: None,
        }
    }
    /// モデル型 → HoneyEditRequest への変換
    pub fn from_honey_input(input: &HoneyInput, id: i64) -> Self {
        HoneyEditRequest {
            id,
            basic: HoneyEditBasicRequest::from_honey_input_basic(&input.basic),
            dynamic: input.dynamic.iter().map(|d| HoneyEditDynamicRequest::from_honey_input_dynamic(d)).collect(),
            updated_at: None,
        }
    }
}
