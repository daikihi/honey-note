use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Document, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement, Window, Response};
use common_type::models::beekeeper::Beekeeper;
use common_type::models::prefectures::Prefecture;
use crate::commons::ajax::get_list_data;

pub async fn setup_new_mode(_window: &Window, document: &Document) {
    let _ = fetch_and_populate_prefectures(document).await;
    setup_save_handler(document);
}

async fn fetch_and_populate_prefectures(document: &Document) -> Result<(), JsValue> {
    let api_path = "/honey-note/api/prefectures";
    let result = get_list_data(api_path).await?;
    let resp: Response = result.dyn_into().map_err(|_| JsValue::from_str("Expected Response"))?;
    let json = JsFuture::from(resp.json()?).await?;
    let prefectures: Vec<Prefecture> = serde_wasm_bindgen::from_value(json)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse prefectures: {:?}", e)))?;

    if let Some(select) = document.get_element_by_id("location_prefecture_id").and_then(|el| el.dyn_into::<HtmlSelectElement>().ok()) {
        for pref in prefectures {
            let option = document.create_element("option")?.dyn_into::<web_sys::HtmlOptionElement>()?;
            option.set_value(&pref.id.to_string());
            option.set_text(&pref.name_jp);
            select.add_with_html_option_element(&option)?;
        }
    }
    Ok(())
}

fn setup_save_handler(document: &Document) {
    if let Some(btn) = document.get_element_by_id("btn_save") {
        let document_clone = document.clone();
        let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            let doc = document_clone.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Err(e) = handle_save(&doc).await {
                    web_sys::console::error_1(&e);
                    web_sys::window().unwrap().alert_with_message("保存に失敗しました").unwrap();
                }
            });
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }
}

async fn handle_save(document: &Document) -> Result<(), JsValue> {
    let beekeeper = collect_form_data(document)?;
    let api_url = "/honey-note/api/beekeeper/new";
    
    let mut opts = web_sys::RequestInit::new();
    opts.method("PUT");
    opts.body(Some(&JsValue::from_str(&serde_json::to_string(&beekeeper).unwrap())));
    
    let request = web_sys::Request::new_with_str_and_init(api_url, &opts)?;
    request.headers().set("Content-Type", "application/json")?;
    
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    
    if resp.ok() {
        window.alert_with_message("保存しました")?;
        window.location().set_href("/honey_note/beekeepers/lists.html")?;
    } else {
        let text = JsFuture::from(resp.text()?).await?;
        return Err(text);
    }
    Ok(())
}

pub fn collect_form_data(document: &Document) -> Result<Beekeeper, JsValue> {
    let name_jp = get_input_value(document, "name_jp");
    let name_en = get_input_value(document, "name_en");
    let founding_year = get_input_value(document, "founding_year").parse::<i32>().ok();
    let location_prefecture_id = get_select_value(document, "location_prefecture_id").parse::<i32>().ok();
    let location_city = get_input_value(document, "location_city");
    let website_url = get_input_value(document, "website_url");
    let note = get_textarea_value(document, "note");

    Ok(Beekeeper {
        id: None,
        name_jp,
        name_en: if name_en.is_empty() { None } else { Some(name_en) },
        founding_year,
        location_prefecture_id,
        location_city: if location_city.is_empty() { None } else { Some(location_city) },
        website_url: if website_url.is_empty() { None } else { Some(website_url) },
        note: if note.is_empty() { None } else { Some(note) },
    })
}

pub fn get_input_value(document: &Document, id: &str) -> String {
    document.get_element_by_id(id)
        .and_then(|el| el.dyn_into::<HtmlInputElement>().ok())
        .map(|el| el.value())
        .unwrap_or_default()
}

pub fn get_select_value(document: &Document, id: &str) -> String {
    document.get_element_by_id(id)
        .and_then(|el| el.dyn_into::<HtmlSelectElement>().ok())
        .map(|el| el.value())
        .unwrap_or_default()
}

pub fn get_textarea_value(document: &Document, id: &str) -> String {
    document.get_element_by_id(id)
        .and_then(|el| el.dyn_into::<HtmlTextAreaElement>().ok())
        .map(|el| el.value())
        .unwrap_or_default()
}
