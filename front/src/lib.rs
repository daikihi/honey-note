use wasm_bindgen::prelude::*;

mod commons;
mod lists;
mod models;
mod edit_and_new;

#[wasm_bindgen]
pub fn top_page_main() {
    web_sys::console::log_1(&"Hello, Honey Note!".into());
}

/**
 *  For List Pages
 */


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

/**
 *  For Edit and new Pages
 */
#[wasm_bindgen]
pub fn honey_edit_and_new_main() {
    web_sys::console::log_1(&"Hello, Honey Edit and New!".into());
    edit_and_new::honey_edit_and_new_main::run();
}


/**
 *  For Common Functions as library
 */


//  For Filtering
#[wasm_bindgen]
pub fn filter_rows(keyword: &str, rows: Vec<String>) -> Vec<usize> {
    commons::filters::filter_rows(keyword, rows)
}
