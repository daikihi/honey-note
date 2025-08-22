#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Beekeeper {
    pub id: Option<i32>,
    pub name_jp: String,
    pub name_en: Option<String>,
    pub founding_year: Option<i32>, // 創業・設立された西暦年
    pub location_prefecture_id: Option<i32>,
    pub location_city: Option<String>,
    pub website_url: Option<String>,
    pub note: Option<String>,
}

impl Beekeeper {
    pub fn from_string_csv(
        beekeeper_name: &str,
        beekeeper_name_en: Option<&str>,
        beekeeper_city: Option<&str>,
        prefecture_id: Option<i32>,
    ) -> Self {
        Beekeeper {
            id: None, // 新規作成時はIDはNone
            name_jp: beekeeper_name.to_string(),
            name_en: beekeeper_name_en.map(|s| s.to_string()),
            founding_year: None,
            location_prefecture_id: prefecture_id,
            location_city: beekeeper_city.map(|s| s.to_string()),
            website_url: None,
            note: None,
        }
    }
}
