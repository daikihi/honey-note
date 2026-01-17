//! はちみつ編集リクエスト用の基本情報DTO
//!
//! このファイルは、クライアントからサーバーへのはちみつ編集リクエストのうち、
//! 基本情報部分（名前、養蜂家、産地など）をJsonで受け取るための型を定義します。
//!
//! - 用途: HTTPリクエストのJsonボディのバリデーション・型安全化
//! - 変換: to_honey_input_basic, from_honey_input_basic で内部モデルと相互変換

use serde::{Serialize, Deserialize};
use chrono::{DateTime, FixedOffset};
use crate::models::honey_input_basic::HoneyInputBasic;
use crate::models::honey_input_types::{HoneyNameJp, BeekeeperName, Country, Region, FlowerName, HoneyType, Volume};

/// はちみつ編集リクエストの基本情報DTO
///
/// - name_jp: 日本語のはちみつ名
/// - beekeeper_name: 養蜂家の名前
/// - harvest_year: 収穫年
/// - country: 国名
/// - region: 地域名
/// - flower_names: 花の名前リスト
/// - honey_type: はちみつの種類
/// - volume: 容量
/// - purchase_date: 購入日（ISO8601文字列）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoneyEditBasicRequest {
    /// 日本語のはちみつ名
    pub name_jp: Option<String>,
    /// 養蜂家の名前
    pub beekeeper_name: Option<String>,
    /// 収穫年
    pub harvest_year: Option<String>,
    /// 国名
    pub country: Option<String>,
    /// 地域名
    pub region: Option<String>,
    /// 花の名前リスト
    pub flower_names: Vec<String>,
    /// はちみつの種類
    pub honey_type: Option<String>,
    /// 容量
    pub volume: Option<String>,
    /// 購入日（ISO8601文字列）
    pub purchase_date: Option<String>,
}

impl HoneyEditBasicRequest {
    /// HoneyEditBasicRequest → モデル型への変換
    pub fn to_honey_input_basic(&self) -> HoneyInputBasic {
        HoneyInputBasic {
            name_jp: HoneyNameJp(self.name_jp.clone().unwrap_or_default()),
            beekeeper_name: self.beekeeper_name.clone().map(BeekeeperName),
            harvest_year: self.harvest_year.as_ref().and_then(|s| s.parse::<i32>().ok()),
            country: self.country.clone().map(Country),
            region: self.region.clone().map(Region),
            flower_names: self.flower_names.iter().map(|s| FlowerName(s.clone())).collect(),
            honey_type: self.honey_type.clone().map(HoneyType),
            volume: self.volume.clone().map(Volume),
            purchase_date: self.purchase_date.as_ref().and_then(|s| DateTime::parse_from_rfc3339(s).ok()),
        }
    }
    /// モデル型 → HoneyEditBasicRequest への変換
    pub fn from_honey_input_basic(basic: &HoneyInputBasic) -> Self {
        HoneyEditBasicRequest {
            name_jp: Some(basic.name_jp.0.clone()),
            beekeeper_name: basic.beekeeper_name.as_ref().map(|b| b.0.clone()),
            harvest_year: basic.harvest_year.map(|y| y.to_string()),
            country: basic.country.as_ref().map(|c| c.0.clone()),
            region: basic.region.as_ref().map(|r| r.0.clone()),
            flower_names: basic.flower_names.iter().map(|f| f.0.clone()).collect(),
            honey_type: basic.honey_type.as_ref().map(|h| h.0.clone()),
            volume: basic.volume.as_ref().map(|v| v.0.clone()),
            purchase_date: basic.purchase_date.as_ref().map(|d| d.to_rfc3339()),
        }
    }
}
