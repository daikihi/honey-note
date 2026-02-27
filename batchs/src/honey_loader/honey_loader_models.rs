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
        pub country: Option<String>,
    }

