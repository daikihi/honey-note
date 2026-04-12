use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Document, Window, Response};

use crate::commons::ajax::{get_list_data, check_authentication};
use common_type::models::beekeeper::Beekeeper;
use common_type::models::prefectures::Prefecture;

pub async fn run() {
    // 認証チェック
    if let Err(_) = check_authentication().await {
        return;
    }

    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");

    // URLからid取得
    let id_opt = (|| {
        let href = window.location().href().ok()?;
        let url = web_sys::Url::new(&href).ok()?;
        url.search_params().get("id")
    })();

    let id = match id_opt {
        Some(v) => v,
        None => {
            web_sys::console::error_1(&JsValue::from_str("beekeeper show: id is missing in URL"));
            return;
        }
    };

    // 都道府県データ取得
    let prefectures_api_path = "/honey-note/api/prefectures";
    let prefectures: Vec<Prefecture> = match get_list_data(prefectures_api_path).await {
        Ok(resp_js) => {
            match convert_to_prefecture_list(resp_js).await {
                Ok(list) => list,
                Err(_) => vec![],
            }
        }
        Err(_) => vec![],
    };

    let api_path = format!("/honey-note/api/beekeeper/{}", id);
    match get_list_data(&api_path).await {
        Ok(resp_js) => {
            match convert_to_beekeeper(resp_js).await {
                Ok(beekeeper) => render_detail(&document, &beekeeper, &prefectures),
                Err(e) => web_sys::console::error_1(&JsValue::from_str(&format!("parse error: {:?}", e))),
            }
        }
        Err(e) => {
            web_sys::console::error_1(&JsValue::from_str(&format!("fetch error: {:?}", e)));
        }
    }
}

async fn convert_to_prefecture_list(value: JsValue) -> Result<Vec<Prefecture>, JsValue> {
    use wasm_bindgen::JsCast;
    let resp: Response = value
        .dyn_into()
        .map_err(|_| JsValue::from_str("Expected Response"))?;
    let json = JsFuture::from(resp.json()?).await?;

    serde_wasm_bindgen::from_value(json).map_err(|err| {
        JsValue::from_str(&format!("Failed to convert JsValue to Vec<Prefecture>: {:?}", err))
    })
}

async fn convert_to_beekeeper(value: JsValue) -> Result<Beekeeper, JsValue> {
    use wasm_bindgen::JsCast;
    let resp: Response = value
        .dyn_into()
        .map_err(|_| JsValue::from_str("Expected Response"))?;
    let json = JsFuture::from(resp.json()?).await?;

    serde_wasm_bindgen::from_value(json).map_err(|err| {
        JsValue::from_str(&format!("Failed to convert JsValue to Beekeeper: {:?}", err))
    })
}

fn set_text(document: &Document, id: &str, text: &str) {
    if let Some(el) = document.get_element_by_id(id) {
        el.set_text_content(Some(text));
    }
}

fn render_detail(document: &Document, beekeeper: &Beekeeper, prefectures: &[Prefecture]) {
    set_text(document, "field_name_jp", &beekeeper.name_jp);
    set_text(document, "field_name_en", beekeeper.name_en.as_deref().unwrap_or("-"));
    
    if let Some(year) = beekeeper.founding_year {
        set_text(document, "field_founding_year", &format!("{}年", year));
    } else {
        set_text(document, "field_founding_year", "-");
    }

    if let Some(pref_id) = beekeeper.location_prefecture_id {
        let pref_name = prefectures.iter()
            .find(|p| p.id == pref_id)
            .map(|p| p.name_jp.as_str())
            .unwrap_or("不明");
        set_text(document, "field_location_prefecture", pref_name);
    } else {
        set_text(document, "field_location_prefecture", "-");
    }

    set_text(document, "field_location_city", beekeeper.location_city.as_deref().unwrap_or("-"));
    
    if let Some(url) = &beekeeper.website_url {
        if let Some(el) = document.get_element_by_id("field_website_url") {
            el.set_inner_html(&format!("<a href='{0}' target='_blank' rel='noopener noreferrer'>{0}</a>", url));
        }
    } else {
        set_text(document, "field_website_url", "-");
    }

    set_text(document, "field_note", beekeeper.note.as_deref().unwrap_or("-"));
}
