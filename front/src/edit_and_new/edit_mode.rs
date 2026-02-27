use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement, Window};
use common_type::request::honey::edit::HoneyEditRequest;
use common_type::request::honey::new::HoneyNewRequest;
use common_type::request::honey::basic::HoneyEditBasicRequest;
use common_type::request::honey::dynamic::{HoneyEditDynamicRequest, ColorFeature, ObservationInputRequest};
use crate::edit_and_new::new_mode;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

pub fn setup_edit_mode(_window: &Window, document: &web_sys::Document) {
    let pathname = _window.location().pathname().unwrap_or_default();
    let is_edit = pathname.contains("/edit.html");

    if is_edit {
        if let Ok(url) = web_sys::Url::new(&_window.location().href().expect("failed to get href")) {
            let params = url.search_params();
            if let Some(id_str) = params.get("id") {
                if let Some(h1) = document.get_elements_by_tag_name("h1").item(0) {
                    h1.set_inner_html(&format!("蜂蜜の編集 (ID: {})", id_str));
                }
            }
        }

        // 編集モードでもマスタデータの取得と検索・新規追加機能をセットアップし、元データでフォームを初期化
        let document_clone = document.clone();
        let window_clone = _window.clone();
        wasm_bindgen_futures::spawn_local(async move {
            // 1) マスタの読み込み（セレクトの選択肢を先に用意）
            let _ = new_mode::fetch_and_populate_masters(&document_clone).await;
            // 2) 既存データでフォーム初期化
            if let Err(e) = prefill_existing_honey(&window_clone, &document_clone).await {
                web_sys::console::error_1(&JsValue::from_str(&format!("prefill failed: {:?}", e)));
            }
        });
        new_mode::setup_beekeeper_search(document);
        new_mode::setup_flower_search(document);
        new_mode::setup_add_new_handlers(document);
    }

    let save_button = document.get_element_by_id("btn_save")
        .or_else(|| {
            // Check if there is a button with class btn-save
            let buttons = document.get_elements_by_class_name("btn-save");
            buttons.item(0)
        });

    if let Some(button) = save_button {
        let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            wasm_bindgen_futures::spawn_local(async move {
                let window = web_sys::window().expect("no global `window` exists");
                match handle_save().await {
                    Ok(_) => {
                        let _ = window.alert_with_message("保存しました");
                    },
                    Err(e) => {
                        web_sys::console::error_1(&e);
                        let _ = window.alert_with_message(&format!("保存に失敗しました: {:?}", e));
                    }
                }
            });
        }) as Box<dyn FnMut(_)>);

        button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).expect("failed to add event listener");
        closure.forget();
    } else {
        web_sys::console::warn_1(&"Save button not found".into());
    }
}

async fn prefill_existing_honey(window: &Window, document: &web_sys::Document) -> Result<(), JsValue> {
    // URL から id を取得
    let url = web_sys::Url::new(&window.location().href().expect("failed to get href"))?;
    let params = url.search_params();
    let id_str = params.get("id").ok_or_else(|| JsValue::from_str("Missing ID in URL"))?;
    let target_id: i32 = id_str.parse().map_err(|_| JsValue::from_str("Invalid ID format"))?;

    // 単品取得 API からデータ取得
    let api = format!("/honey-note/api/honey/{}", target_id);
    let resp_val = crate::commons::ajax::get_list_data(&api).await?;
    use wasm_bindgen::JsCast;
    use web_sys::Response;
    let resp: Response = resp_val.dyn_into().map_err(|_| JsValue::from_str("Expected Response"))?;
    
    if !resp.ok() {
        return Err(JsValue::from_str(&format!("Failed to fetch honey detail: {}", resp.status())));
    }

    let json = JsFuture::from(resp.json()?).await?;
    let h: common_type::models::honey_detail::HoneyDetail = serde_wasm_bindgen::from_value(json)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse honey detail: {:?}", e)))?;

    // 基本情報の初期化
    set_input_value(document, "name_jp", &h.basic.name_jp.0);
    if let Some(country) = &h.basic.country { set_input_value(document, "country", &country.0); }
    if let Some(region) = &h.basic.region { set_input_value(document, "prefecture", &region.0); }
    if let Some(year) = h.basic.harvest_year { set_input_value(document, "harvest_year", &year.to_string()); }
    if let Some(pd) = &h.basic.purchase_date {
        // date input 用に YYYY-MM-DD へ整形
        let date_str = pd.to_rfc3339()[0..10].to_string();
        set_input_value(document, "purchase_date", &date_str);
    }
    if let Some(bk) = &h.basic.beekeeper_name { 
        set_select_value(document, "beekeeper_name", &bk.0); 
    }

    // 蜜源植物（複数選択）の初期化
    let flower_names: Vec<String> = h.basic.flower_names.iter().map(|f| f.0.clone()).collect();
    set_multi_select_values(document, "flower_name", &flower_names);

    Ok(())
}

async fn handle_save() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");


    // Collect Basic Info
    let name_jp = get_input_value(&document, "name_jp");
    let beekeeper_name = get_select_value(&document, "beekeeper_name");
    let harvest_year = get_input_value(&document, "harvest_year");
    let country = get_input_value(&document, "country");
    let region = get_input_value(&document, "prefecture"); // ID is prefecture in HTML
    
    // flower_names (multiple select)
    let flower_names = get_multi_select_values(&document, "flower_name");

    let honey_type = get_select_value(&document, "honey_flower_type");
    let volume = get_input_value(&document, "honey_volume");
    let purchase_date = get_input_value(&document, "purchase_date");

    let basic = HoneyEditBasicRequest {
        name_jp: Some(name_jp),
        beekeeper_name: Some(beekeeper_name),
        harvest_year: Some(harvest_year),
        country: Some(country),
        region: Some(region),
        flower_names,
        honey_type: Some(honey_type),
        volume: Some(volume),
        purchase_date: if purchase_date.is_empty() { None } else { Some(purchase_date) },
    };

    // Collect Dynamic Info
    let color_feature = Some(ColorFeature {
        category: Some(get_select_value(&document, "color_category")),
        hex: Some(get_input_value(&document, "color_hex")),
        note: Some(get_input_value(&document, "color_note")),
    });

    let aroma_intensity = Some(get_select_value(&document, "aroma_intensity"));
    let aroma_type = Some(get_select_value(&document, "aroma_type"));
    let aroma_note = Some(get_textarea_value(&document, "aroma_note"));

    let sweetness_intensity = Some(get_select_value(&document, "sweetness_intensity"));
    let acidity = Some(get_select_value(&document, "acidity"));
    let mouthfeel = Some(get_select_value(&document, "mouthfeel"));
    let finish = Some(get_select_value(&document, "finish"));
    let taste_note = Some(get_textarea_value(&document, "taste_note"));

    let crystallization_level = Some(get_select_value(&document, "crystallization_level"));
    let crystal_texture = Some(get_select_value(&document, "crystal_texture"));

    let preference = get_select_value(&document, "preference").parse::<u8>().ok();
    let usage = Some(get_input_value(&document, "usage"));
    let tags = Some(get_input_value(&document, "tags"));
    
    // Observations (Simplified: just one for now as per HTML)
    let mut observations = Vec::new();
    let obs_date = get_input_value(&document, "observation_date_1");
    if !obs_date.is_empty() {
        observations.push(ObservationInputRequest {
            date: Some(obs_date),
            state: Some(get_select_value(&document, "observation_state_1")),
            taste: Some(get_input_value(&document, "observation_taste_1")),
            note: Some(get_input_value(&document, "observation_note_1")),
        });
    }

    let memo = Some(get_textarea_value(&document, "memo"));

    let dynamic = vec![HoneyEditDynamicRequest {
        color_feature,
        aroma_intensity,
        aroma_type,
        aroma_note,
        sweetness_intensity,
        acidity,
        mouthfeel,
        finish,
        taste_note,
        crystallization_level,
        crystal_texture,
        preference,
        usage,
        tags,
        observations,
        memo,
    }];

    let pathname = window.location().pathname().unwrap_or_default();
    let is_new = pathname.contains("/new.html");

    let (api_url, method, json_body) = if is_new {
        let request_payload = HoneyNewRequest {
            basic,
            dynamic,
            created_at: None,
        };
        (
            "/honey-note/api/honey/new",
            "PUT",
            serde_json::to_string(&request_payload).map_err(|e| JsValue::from_str(&e.to_string()))?
        )
    } else {
        // Extract ID from URL (e.g., /edit.html?id=1)
        let url = web_sys::Url::new(&window.location().href().expect("failed to get href"))?;
        let params = url.search_params();
        let id_str = params.get("id").ok_or_else(|| JsValue::from_str("Missing ID in URL"))?;
        let id: i64 = id_str.parse().map_err(|_| JsValue::from_str("Invalid ID format"))?;

        let request_payload = HoneyEditRequest {
            id,
            basic,
            dynamic,
            updated_at: None,
        };
        (
            "/honey-note/api/honey/edit",
            "PUT",
            serde_json::to_string(&request_payload).map_err(|e| JsValue::from_str(&e.to_string()))?
        )
    };

    // Send request
    let opts = RequestInit::new();
    opts.set_method(method);
    opts.set_mode(web_sys::RequestMode::Cors);
    opts.set_body(&JsValue::from_str(&json_body));

    let request = Request::new_with_str_and_init(api_url, &opts)?;
    request.headers().set("Content-Type", "application/json")?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;

    if !resp.ok() {
        let text = JsFuture::from(resp.text()?).await?;
        return Err(JsValue::from_str(&format!("Server Error: {:?}", text)));
    }

    Ok(())
}

fn set_input_value(document: &web_sys::Document, id: &str, value: &str) {
    if let Some(el) = document.get_element_by_id(id).and_then(|el| el.dyn_into::<HtmlInputElement>().ok()) {
        el.set_value(value);
    }
}

fn set_select_value(document: &web_sys::Document, id: &str, value: &str) {
    if let Some(el) = document.get_element_by_id(id).and_then(|el| el.dyn_into::<HtmlSelectElement>().ok()) {
        el.set_value(value);
    }
}

fn set_multi_select_values(document: &web_sys::Document, id: &str, values: &[String]) {
    if let Some(el) = document.get_element_by_id(id).and_then(|el| el.dyn_into::<HtmlSelectElement>().ok()) {
        let options = el.options();
        for i in 0..options.length() {
            if let Some(option) = options.item(i).and_then(|o| o.dyn_into::<web_sys::HtmlOptionElement>().ok()) {
                let val = option.value();
                if values.contains(&val) {
                    option.set_selected(true);
                } else {
                    option.set_selected(false);
                }
            }
        }
    }
}

fn get_input_value(document: &web_sys::Document, id: &str) -> String {
    document.get_element_by_id(id)
        .and_then(|el| el.dyn_into::<HtmlInputElement>().ok())
        .map(|el| el.value())
        .unwrap_or_default()
}

fn get_select_value(document: &web_sys::Document, id: &str) -> String {
    document.get_element_by_id(id)
        .and_then(|el| el.dyn_into::<HtmlSelectElement>().ok())
        .map(|el| el.value())
        .unwrap_or_default()
}

fn get_textarea_value(document: &web_sys::Document, id: &str) -> String {
    document.get_element_by_id(id)
        .and_then(|el| el.dyn_into::<HtmlTextAreaElement>().ok())
        .map(|el| el.value())
        .unwrap_or_default()
}

fn get_multi_select_values(document: &web_sys::Document, id: &str) -> Vec<String> {
    let mut values = Vec::new();
    if let Some(el) = document.get_element_by_id(id) {
        if let Ok(select) = el.dyn_into::<HtmlSelectElement>() {
            let options = select.options();
            for i in 0..options.length() {
                if let Some(option) = options.item(i).and_then(|o| o.dyn_into::<web_sys::HtmlOptionElement>().ok()) {
                    if option.selected() {
                        values.push(option.value());
                    }
                }
            }
        }
    }
    values
}
