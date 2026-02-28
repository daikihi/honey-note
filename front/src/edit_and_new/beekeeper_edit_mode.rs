use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Document, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement, Window, Response};
use common_type::models::beekeeper::Beekeeper;
use crate::edit_and_new::beekeeper_new_mode::collect_form_data;

pub async fn setup_edit_mode(window: &Window, document: &Document) {
    let url = web_sys::Url::new(&window.location().href().unwrap()).unwrap();
    let params = url.search_params();
    let id_str = params.get("id").expect("Missing ID in URL");
    let id: i32 = id_str.parse().expect("Invalid ID format");

    let _ = fetch_and_populate_prefectures(document).await;
    let _ = fetch_and_prefill_beekeeper(id, document).await;
    setup_save_handler(id, document);
}

async fn fetch_and_populate_prefectures(document: &Document) -> Result<(), JsValue> {
    // Re-use logic or call beekeeper_new_mode's function if it was public/exported
    // For simplicity, let's assume it's duplicated or moved to a common place if needed.
    // In this case, I'll just re-implement it briefly.
    use common_type::models::prefectures::Prefecture;
    use crate::commons::ajax::get_list_data;

    let api_path = "/honey-note/api/prefectures";
    let result = get_list_data(api_path).await?;
    let resp: Response = result.dyn_into()?;
    let json = JsFuture::from(resp.json()?).await?;
    let prefectures: Vec<Prefecture> = serde_wasm_bindgen::from_value(json).unwrap();

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

async fn fetch_and_prefill_beekeeper(id: i32, document: &Document) -> Result<(), JsValue> {
    let api_url = format!("/honey-note/api/beekeeper/{}", id);
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_str(&api_url)).await?;
    let resp: Response = resp_value.dyn_into()?;
    
    if !resp.ok() {
        return Err(JsValue::from_str("Failed to fetch beekeeper"));
    }

    let json = JsFuture::from(resp.json()?).await?;
    let beekeeper: Beekeeper = serde_wasm_bindgen::from_value(json).unwrap();

    set_input_value(document, "name_jp", &beekeeper.name_jp);
    set_input_value(document, "name_en", &beekeeper.name_en.unwrap_or_default());
    set_input_value(document, "founding_year", &beekeeper.founding_year.map(|y| y.to_string()).unwrap_or_default());
    set_select_value(document, "location_prefecture_id", &beekeeper.location_prefecture_id.map(|id| id.to_string()).unwrap_or_default());
    set_input_value(document, "location_city", &beekeeper.location_city.unwrap_or_default());
    set_input_value(document, "website_url", &beekeeper.website_url.unwrap_or_default());
    set_textarea_value(document, "note", &beekeeper.note.unwrap_or_default());

    Ok(())
}

fn setup_save_handler(id: i32, document: &Document) {
    if let Some(btn) = document.get_element_by_id("btn_save") {
        let document_clone = document.clone();
        let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            let doc = document_clone.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Err(e) = handle_update(id, &doc).await {
                    web_sys::console::error_1(&e);
                    web_sys::window().unwrap().alert_with_message("更新に失敗しました").unwrap();
                }
            });
        }) as Box<dyn FnMut(_)>);
        btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }
}

async fn handle_update(id: i32, document: &Document) -> Result<(), JsValue> {
    let mut beekeeper = collect_form_data(document)?;
    beekeeper.id = Some(id);
    let api_url = format!("/honey-note/api/beekeeper/edit/{}", id);
    
    let mut opts = web_sys::RequestInit::new();
    opts.method("PUT");
    opts.body(Some(&JsValue::from_str(&serde_json::to_string(&beekeeper).unwrap())));
    
    let request = web_sys::Request::new_with_str_and_init(&api_url, &opts)?;
    request.headers().set("Content-Type", "application/json")?;
    
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    
    if resp.ok() {
        window.alert_with_message("更新しました")?;
        window.location().set_href("/honey_note/beekeepers/lists.html")?;
    } else {
        let text = JsFuture::from(resp.text()?).await?;
        return Err(text);
    }
    Ok(())
}

fn set_input_value(document: &Document, id: &str, value: &str) {
    if let Some(el) = document.get_element_by_id(id).and_then(|el| el.dyn_into::<HtmlInputElement>().ok()) {
        el.set_value(value);
    }
}

fn set_select_value(document: &Document, id: &str, value: &str) {
    if let Some(el) = document.get_element_by_id(id).and_then(|el| el.dyn_into::<HtmlSelectElement>().ok()) {
        el.set_value(value);
    }
}

fn set_textarea_value(document: &Document, id: &str, value: &str) {
    if let Some(el) = document.get_element_by_id(id).and_then(|el| el.dyn_into::<HtmlTextAreaElement>().ok()) {
        el.set_value(value);
    }
}
