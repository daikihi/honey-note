use wasm_bindgen::prelude::*;

mod commons;
mod lists;
mod models;

#[wasm_bindgen]
pub fn top_page_main() {
    web_sys::console::log_1(&"Hello, Honey Note!".into());
}

#[wasm_bindgen]
pub fn honey_list_main() {
    web_sys::console::log_1(&"Hello, Honey List!".into());
}

#[wasm_bindgen]
pub fn flower_list_main() {
    web_sys::console::log_1(&"Hello, Flower List!".into());
    lists::flower_list_page_main::run();
}
