// ラップ型定義まとめ
use serde::{Serialize, Deserialize};

/// 日本語のはちみつ名
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoneyNameJp(pub String);
/// 養蜂家の名前
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeekeeperName(pub String);
/// 国名
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country(pub String);
/// 地域名（都道府県や地方など）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region(pub String);
/// 花の名前
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowerName(pub String);
/// はちみつの種類（単花蜜/百花蜜など）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoneyType(pub String);
/// 容量（例: 100g, 200ml など）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Volume(pub String);
/// 色カテゴリ（例: 明色/中間色/暗色など）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category(pub String);
/// 色の16進表現（例: #ffeeaa）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hex(pub String);
/// 色に関する補足説明
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorNote(pub String);
/// 香りの強さ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AromaIntensity(pub String);

impl AromaIntensity {
    /// Returns the Japanese display label for this raw value, or `None` if it is unknown.
    pub fn as_label(&self) -> Option<&'static str> {
        match self.0.as_str() {
            "very_light" => Some("ほとんど感じない"),
            "light" => Some("やさしい"),
            "moderate" => Some("はっきり"),
            "strong" => Some("強い"),
            "distinctive" => Some("とても個性的"),
            _ => None,
        }
    }
}

/// 香りのタイプ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AromaType(pub String);
/// 香りの詳細ノート
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AromaNote(pub String);
/// 甘さの強さ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SweetnessIntensity(pub String);

impl SweetnessIntensity {
    /// Returns the Japanese display label for this raw value, or `None` if it is unknown.
    pub fn as_label(&self) -> Option<&'static str> {
        match self.0.as_str() {
            "very_light" => Some("ほとんど甘くない"),
            "light" => Some("控えめ"),
            "moderate" => Some("ちょうどよい"),
            "strong" => Some("かなり甘い"),
            "very_strong" => Some("喉を突くような甘さ"),
            _ => None,
        }
    }
}

/// 酸味の強さ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Acidity(pub String);

impl Acidity {
    /// Returns the Japanese display label for this raw value, or `None` if it is unknown.
    pub fn as_label(&self) -> Option<&'static str> {
        match self.0.as_str() {
            "none" => Some("ほぼ感じない"),
            "light" => Some("わずかに感じる"),
            "moderate" => Some("はっきり感じる"),
            "strong" => Some("強く感じる"),
            _ => None,
        }
    }
}

/// 口当たり
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mouthfeel(pub String);

impl Mouthfeel {
    /// Returns the Japanese display label for this raw value, or `None` if it is unknown.
    pub fn as_label(&self) -> Option<&'static str> {
        match self.0.as_str() {
            "light" => Some("さらっとしている"),
            "moderate" => Some("なめらか"),
            "thick" => Some("とろみがある"),
            "very_thick" => Some("かなり重たい"),
            _ => None,
        }
    }
}

/// 余韻
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finish(pub String);

impl Finish {
    /// Returns the Japanese display label for this raw value, or `None` if it is unknown.
    pub fn as_label(&self) -> Option<&'static str> {
        match self.0.as_str() {
            "clean" => Some("すっきり消える"),
            "gentle" => Some("やさしく残る"),
            "long" => Some("余韻が長い"),
            "distinctive" => Some("個性的な余韻"),
            _ => None,
        }
    }
}

/// 味の詳細ノート
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TasteNote(pub String);
/// 結晶化の度合い
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrystallizationLevel(pub String);

impl CrystallizationLevel {
    /// Returns the Japanese display label for this raw value, or `None` if it is unknown.
    pub fn as_label(&self) -> Option<&'static str> {
        match self.0.as_str() {
            "liquid" => Some("完全に液状"),
            "slightly" => Some("うっすら結晶化"),
            "partial" => Some("部分的に結晶化"),
            "mostly" => Some("ほぼ結晶化"),
            "solid" => Some("完全に結晶化"),
            _ => None,
        }
    }
}

/// 結晶の質感
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrystalTexture(pub String);

impl CrystalTexture {
    /// Returns the Japanese display label for this raw value, or `None` if it is unknown.
    pub fn as_label(&self) -> Option<&'static str> {
        match self.0.as_str() {
            "smooth" => Some("なめらか（微細）"),
            "creamy" => Some("クリーミー"),
            "grainy" => Some("ざらざら"),
            "hard" => Some("硬い・シャリシャリ"),
            _ => None,
        }
    }
}

/// 主な用途（例: トースト、ヨーグルト等）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage(pub String);
/// タグ（カンマ区切り等）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tags(pub String);
/// メモ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memo(pub String);
/// 観察時の状態
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State(pub String);
/// 観察時の味
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Taste(pub String);
/// 観察時の補足ノート
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationNote(pub String);
