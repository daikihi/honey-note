use wasm_bindgen::prelude::*;

mod commons;
mod lists;
mod models;

#[wasm_bindgen]
pub fn top_page_main() {
    web_sys::console::log_1(&"Hello, Honey Note!".into());
}

#[wasm_bindgen]
pub async fn honey_list_main() {
    web_sys::console::log_1(&"Hello, Honey List!".into());
    lists::honeys_list_page_main::run().await;
}

#[wasm_bindgen]
pub async fn flower_list_main() {
    web_sys::console::log_1(&"Hello, Flower List!".into());
    lists::flower_list_page_main::run().await;
}

#[wasm_bindgen]
pub async fn beekeepers_list_main() {
    web_sys::console::log_1(&"Hello, Beekeepers List!".into());
    lists::beekeepers_list_page_main::run().await;
}

#[wasm_bindgen]
pub fn filter_rows(keyword: &str, rows: Vec<String>) -> Vec<usize> {
    commons::filters::filter_rows(keyword, rows)
}
