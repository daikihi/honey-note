use crate::models::{flowers::Flower, honey::Honey};

use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HoneyDetail {
    pub honey: Honey,
    pub flowers: Vec<Flower>, // 花のリスト
}
