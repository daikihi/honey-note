use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Document, HtmlInputElement, HtmlTextAreaElement, Window, Response};
use common_type::models::flowers::Flower;

pub async fn setup_new_mode(_window: &Window, document: &Document) {
    setup_save_handler(document);
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
    let flower = collect_form_data(document)?;
    let api_url = "/honey-note/api/flower/new";
    
    let mut opts = web_sys::RequestInit::new();
    opts.method("PUT");
    opts.body(Some(&JsValue::from_str(&serde_json::to_string(&flower).unwrap())));
    
    let request = web_sys::Request::new_with_str_and_init(api_url, &opts)?;
    request.headers().set("Content-Type", "application/json")?;
    
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    
    if resp.ok() {
        window.alert_with_message("保存しました")?;
        window.location().set_href("/honey_note/flowers/lists.html")?;
    } else {
        let text = JsFuture::from(resp.text()?).await?;
        return Err(text);
    }
    Ok(())
}

pub fn collect_form_data(document: &Document) -> Result<Flower, JsValue> {
    let name_jp = get_input_value(document, "name_jp");
    let name_en = get_input_value(document, "name_en");
    let scientific_name = get_input_value(document, "scientific_name");
    let short_note = get_input_value(document, "short_note");
    let flower_type = get_input_value(document, "flower_type");
    let note = get_textarea_value(document, "note");

    Ok(Flower {
        id: None,
        name_jp,
        name_en: if name_en.is_empty() { None } else { Some(name_en) },
        scientific_name: if scientific_name.is_empty() { None } else { Some(scientific_name) },
        short_note: if short_note.is_empty() { None } else { Some(short_note) },
        flower_type: if flower_type.is_empty() { None } else { Some(flower_type) },
        image_path: None,
        note: if note.is_empty() { None } else { Some(note) },
    })
}

pub fn get_input_value(document: &Document, id: &str) -> String {
    document.get_element_by_id(id)
        .and_then(|el| el.dyn_into::<HtmlInputElement>().ok())
        .map(|el| el.value())
        .unwrap_or_default()
}

pub fn get_textarea_value(document: &Document, id: &str) -> String {
    document.get_element_by_id(id)
        .and_then(|el| el.dyn_into::<HtmlTextAreaElement>().ok())
        .map(|el| el.value())
        .unwrap_or_default()
}
