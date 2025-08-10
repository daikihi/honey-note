use serde::Deserialize;

/**
 * JsonHoney is a struct to map with Json data as flowwing data
 * {"id": 1, "name": "", "beekeeper": "", "prefecture": "", "city": "", "country": "japan", "flowers": [], "year": 0, "expired_at_year": 0, "expired_at_month": 0, "bouhgt_in": ""}
 *
 */
#[derive(Deserialize, Debug, Clone)]
pub struct JsonHoney {
    pub id: i32,
    pub name: Option<String>,
    pub beekeeper: Option<String>,
    pub prefecture: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub flowers: Vec<String>,
    pub year: Option<i32>,
    pub expired_at_year: Option<i32>,
    pub expired_at_month: Option<i32>,
    pub bought_in: Option<String>,
}

impl JsonHoney {
    pub fn to_model_honey(&self) -> common_type::models::honey::Honey {
        common_type::models::honey::Honey {
            id: Some(self.id),
            name_jp: self.name.clone().unwrap_or_default(),
            name_en: None,    // Assuming no English name in JSON
            beekkeeper: None, // @todo if needed
            origin_country: self.country.clone(),
            origin_region: self.prefecture.clone(),
            harvest_year: self.year,
            purchase_date: self.bought_in.clone(),
            note: None, // Assuming no note in JSON
        }
    }
}
