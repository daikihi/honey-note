#[derive(Debug)]
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
    pub fn from_string_csv(line: &str) -> Self {
        Beekeeper {
            id: None, // 新規作成時はIDはNone
            name_jp: line.to_string(),
            name_en: None,
            founding_year: None,
            location_prefecture_id: None,
            location_city: None,
            website_url: None,
            note: None,
        }
    }
}
