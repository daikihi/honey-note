// HoneyInput モデル（新規登録・編集用）
// このファイルはフォーム入力値を受け取るためのモデルです。
//
// HoneyInput: 1件のはちみつデータの全入力情報（基本情報＋動的情報群）をまとめて管理するエントリポイント。
// - basic: 基本情報（名前、養蜂家、産地など）
// - dynamic: 動的情報（色・香り・味・観察記録など）を複数持つ
// - created_at/updated_at: レコードの作成・更新日時

use chrono::{DateTime, FixedOffset};
use serde::{Serialize, Deserialize};
use crate::models::honey_input_basic::HoneyInputBasic;
use crate::models::honey_input_dynamic::HoneyInputDynamic;

/// 1件のはちみつデータの全入力情報をまとめるモデル
///
/// - basic: 基本情報（HoneyInputBasic）
/// - dynamic: 色・香り・味・観察記録などの動的情報（HoneyInputDynamic）のリスト
/// - created_at: レコード作成日時
/// - updated_at: レコード更新日時
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoneyInput {
    /// 基本情報（名前、養蜂家、産地など）
    pub basic: HoneyInputBasic,
    /// 色・香り・味・観察記録などの動的情報（複数）
    pub dynamic: Vec<HoneyInputDynamic>,
    /// レコード作成日時
    pub created_at: Option<DateTime<FixedOffset>>,
    /// レコード更新日時
    pub updated_at: Option<DateTime<FixedOffset>>,
}

