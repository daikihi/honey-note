use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Document, HtmlInputElement, HtmlSelectElement, Window, Response};
use common_type::models::beekeeper::Beekeeper;
use common_type::models::flowers::Flower;
use crate::commons::ajax::get_list_data;
use crate::edit_and_new::edit_mode;

pub fn setup_new_mode(window: &Window, document: &Document) {
    web_sys::console::log_1(&"Setting up Honey New Mode".into());
    
    // 基本的な保存処理のセットアップ（edit_modeのロジックを流用）
    edit_mode::setup_edit_mode(window, document);

    // 養蜂場一覧と蜜源一覧の取得
    let document_clone = document.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let _ = fetch_and_populate_masters(&document_clone).await;
    });

    // 検索機能のセットアップ
    setup_beekeeper_search(document);
    setup_flower_search(document);
    setup_add_new_handlers(document);
}

pub async fn fetch_and_populate_masters(document: &Document) -> Result<(), JsValue> {
    // 養蜂場一覧の取得
    let beekeeper_api_path = "/honey-note/api/beekeepers";
    if let Ok(js_val) = get_list_data(beekeeper_api_path).await {
        let beekeepers = convert_to_beekeepers(js_val).await?;
        populate_beekeepers(document, &beekeepers);
    }

    // 蜜源一覧の取得
    let flower_api_path = "/honey-note/api/flowers";
    if let Ok(js_val) = get_list_data(flower_api_path).await {
        let flowers = convert_to_flowers(js_val).await?;
        populate_flowers(document, &flowers);
    }

    Ok(())
}

async fn convert_to_beekeepers(value: JsValue) -> Result<Vec<Beekeeper>, JsValue> {
    let resp: Response = value.dyn_into().map_err(|_| JsValue::from_str("Expected Response"))?;
    let json = JsFuture::from(resp.json()?).await?;
    serde_wasm_bindgen::from_value(json).map_err(|e| JsValue::from_str(&format!("{:?}", e)))
}

async fn convert_to_flowers(value: JsValue) -> Result<Vec<Flower>, JsValue> {
    let resp: Response = value.dyn_into().map_err(|_| JsValue::from_str("Expected Response"))?;
    let json = JsFuture::from(resp.json()?).await?;
    serde_wasm_bindgen::from_value(json).map_err(|e| JsValue::from_str(&format!("{:?}", e)))
}

fn populate_beekeepers(document: &Document, beekeepers: &[Beekeeper]) {
    if let Some(select) = document.get_element_by_id("beekeeper_name").and_then(|el| el.dyn_into::<HtmlSelectElement>().ok()) {
        // Clear existing options except the first one
        while select.length() > 1 {
            select.remove_with_index(1);
        }

        for beekeeper in beekeepers {
            let option = document.create_element("option").unwrap().dyn_into::<web_sys::HtmlOptionElement>().unwrap();
            option.set_value(&beekeeper.name_jp);
            option.set_text(&beekeeper.name_jp);
            select.add_with_html_option_element(&option).unwrap();
        }
    }
}

fn populate_flowers(document: &Document, flowers: &[Flower]) {
    if let Some(select) = document.get_element_by_id("flower_name").and_then(|el| el.dyn_into::<HtmlSelectElement>().ok()) {
        // Clear existing options except the first one
        while select.length() > 1 {
            select.remove_with_index(1);
        }

        for flower in flowers {
            let option = document.create_element("option").unwrap().dyn_into::<web_sys::HtmlOptionElement>().unwrap();
            option.set_value(&flower.name_jp);
            option.set_text(&flower.name_jp);
            select.add_with_html_option_element(&option).unwrap();
        }
    }
}

pub fn setup_beekeeper_search(document: &Document) {
    if let Some(search_input) = document.get_element_by_id("beekeeper_search").and_then(|el| el.dyn_into::<HtmlInputElement>().ok()) {
        let document_clone = document.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            let input = event.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
            let keyword = input.value().to_lowercase();
            
            if let Some(select) = document_clone.get_element_by_id("beekeeper_name").and_then(|el| el.dyn_into::<HtmlSelectElement>().ok()) {
                let options = select.options();
                for i in 1..options.length() { // Skip "Please select"
                    if let Some(option) = options.item(i).and_then(|o| o.dyn_into::<web_sys::HtmlElement>().ok()) {
                        let text = option.inner_text().to_lowercase();
                        if text.contains(&keyword) {
                            let _ = option.style().set_property("display", "");
                        } else {
                            let _ = option.style().set_property("display", "none");
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        search_input.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }
}

pub fn setup_flower_search(document: &Document) {
    if let Some(search_input) = document.get_element_by_id("flower_search").and_then(|el| el.dyn_into::<HtmlInputElement>().ok()) {
        let document_clone = document.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            let input = event.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
            let keyword = input.value().to_lowercase();
            
            if let Some(select) = document_clone.get_element_by_id("flower_name").and_then(|el| el.dyn_into::<HtmlSelectElement>().ok()) {
                let options = select.options();
                for i in 1..options.length() { // Skip "Please select"
                    if let Some(option) = options.item(i).and_then(|o| o.dyn_into::<web_sys::HtmlElement>().ok()) {
                        let text = option.inner_text().to_lowercase();
                        if text.contains(&keyword) {
                            let _ = option.style().set_property("display", "");
                        } else {
                            let _ = option.style().set_property("display", "none");
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        search_input.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }
}

pub fn setup_add_new_handlers(document: &Document) {
    // Beekeeper add handler
    if let Some(btn) = document.get_element_by_id("btn_add_beekeeper") {
        web_sys::console::log_1(&"btn_add_beekeeper found".into());
        let document_clone = document.clone();
        let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            web_sys::console::log_1(&"btn_add_beekeeper clicked".into());
            if let Some(input) = document_clone.get_element_by_id("beekeeper_search").and_then(|el| el.dyn_into::<HtmlInputElement>().ok()) {
                let name = input.value().trim().to_string();
                if name.is_empty() { return; }
                
                if let Some(select) = document_clone.get_element_by_id("beekeeper_name").and_then(|el| el.dyn_into::<HtmlSelectElement>().ok()) {
                    let option = document_clone.create_element("option").unwrap().dyn_into::<web_sys::HtmlOptionElement>().unwrap();
                    option.set_value(&name);
                    option.set_text(&name);
                    option.set_selected(true);
                    let _ = select.add_with_html_option_element(&option);
                    select.set_value(&name); // Set value to the newly added option
                    web_sys::console::log_1(&format!("Added beekeeper: {}", name).into());
                } else {
                    web_sys::console::error_1(&"beekeeper_name select not found".into());
                }
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    } else {
        web_sys::console::error_1(&"btn_add_beekeeper not found".into());
    }

    // Flower add handler
    if let Some(btn) = document.get_element_by_id("btn_add_flower") {
        web_sys::console::log_1(&"btn_add_flower found".into());
        let document_clone = document.clone();
        let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            web_sys::console::log_1(&"btn_add_flower clicked".into());
            if let Some(input) = document_clone.get_element_by_id("flower_search").and_then(|el| el.dyn_into::<HtmlInputElement>().ok()) {
                let name = input.value().trim().to_string();
                if name.is_empty() { return; }
                
                if let Some(select) = document_clone.get_element_by_id("flower_name").and_then(|el| el.dyn_into::<HtmlSelectElement>().ok()) {
                    let option = document_clone.create_element("option").unwrap().dyn_into::<web_sys::HtmlOptionElement>().unwrap();
                    option.set_value(&name);
                    option.set_text(&name);
                    option.set_selected(true);
                    let _ = select.add_with_html_option_element(&option);
                    // For multiple select, we might want to add to current selection or just select this one.
                    // Given the user expectation "select it", let's make it selected.
                    option.set_selected(true); 
                    web_sys::console::log_1(&format!("Added flower: {}", name).into());
                } else {
                    web_sys::console::error_1(&"flower_name select not found".into());
                }
            }
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    } else {
        web_sys::console::error_1(&"btn_add_flower not found".into());
    }
}
