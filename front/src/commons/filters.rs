use wasm_bindgen::prelude::*;

pub fn filter_rows(keyword: &str, rows: Vec<String>) -> Vec<usize> {
    let keyword_lower = keyword.to_lowercase();
    rows.iter()
        .enumerate()
        .filter_map(|(index, row)| {
            if row.to_lowercase().contains(&keyword_lower) {
                Some(index) // Return the matching row's index
            } else {
                None
            }
        })
        .collect()
}