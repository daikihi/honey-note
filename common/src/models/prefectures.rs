#[derive(Debug)]
pub struct Prefecture {
    pub id: i32,
    pub name_jp: String,
    pub name_en: String,
}

impl Prefecture {
    pub fn from_string_csv(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 3 {
            return None; // Not enough parts to form a Prefecture
        }
        Some(Self {
            id: parts[0].parse().ok()?,
            name_jp: parts[1].to_string(),
            name_en: parts[2].to_string(),
        })
    }
}