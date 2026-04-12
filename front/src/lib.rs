use crate::commons::browser_adapter::WebBrowserAdapter;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

pub mod commons;
mod edit_and_new;
mod lists;
mod login_page_main;
mod models;
mod show;
pub mod signup_page_main;

#[wasm_bindgen]
pub async fn login_main() {
    login_page_main::run().await;
}

#[wasm_bindgen]
pub async fn signup_main() {
    let adapter = Rc::new(WebBrowserAdapter);
    if let Err(e) = signup_page_main::run(adapter).await {
        web_sys::console::error_1(&format!("Signup initialization failed: {:?}", e).into());
    }
}

#[wasm_bindgen]
pub async fn top_page_main() {
    // 認証チェック
    if let Err(_) = commons::ajax::check_authentication().await {
        return;
    }
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
pub async fn honey_edit_and_new_main() {
    web_sys::console::log_1(&"Hello, Honey Edit and New!".into());
    edit_and_new::honey_edit_and_new_main::run().await;
}

#[wasm_bindgen]
pub async fn beekeeper_edit_and_new_main() {
    web_sys::console::log_1(&"Hello, Beekeeper Edit and New!".into());
    edit_and_new::beekeeper_edit_and_new_main::run().await;
}

#[wasm_bindgen]
pub async fn flower_edit_and_new_main() {
    web_sys::console::log_1(&"Hello, Flower Edit and New!".into());
    edit_and_new::flower_edit_and_new_main::run().await;
}

#[wasm_bindgen]
pub async fn honey_show_main() {
    web_sys::console::log_1(&"Hello, Honey Show!".into());
    show::honey_show_page_main::run().await;
}

#[wasm_bindgen]
pub async fn beekeeper_show_main() {
    web_sys::console::log_1(&"Hello, Beekeeper Show!".into());
    show::beekeeper_show_page_main::run().await;
}

/**
 *  For Common Functions as library
 */

//  For Filtering
#[wasm_bindgen]
pub fn filter_rows(keyword: &str, rows: Vec<String>) -> Vec<usize> {
    commons::filters::filter_rows(keyword, rows)
}
