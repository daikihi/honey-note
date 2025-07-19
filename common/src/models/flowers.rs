use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Flower {
    pub id: Option<i128>,
    pub name_jp: String,
    pub name_en: Option<String>,
    pub scientific_name: Option<String>,
    pub short_note: Option<String>,
    pub flower_type: Option<String>,
    pub image_path: Option<String>,
    pub note: Option<String>,
}

// for inserting flower from master data
// master data only contains name_jp
pub fn create_model_flower_from_name(name: &str) -> Flower {
    Flower {
        id: None,
        name_jp: name.to_string(),
        name_en: None,
        scientific_name: None,
        short_note: None,
        flower_type: None,
        image_path: None,
        note: None,
    }
}
