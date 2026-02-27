use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Document, Window, Response};

use crate::commons::ajax::get_list_data;

use common_type::models::honey_detail::HoneyDetail;

pub async fn run() {
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
            web_sys::console::error_1(&JsValue::from_str("show: id is missing in URL"));
            return;
        }
    };

    let api_path = format!("/honey-note/api/honey/{}", id);
    match get_list_data(&api_path).await {
        Ok(resp_js) => {
            match convert_to_honey_detail(resp_js).await {
                Ok(detail) => render_detail(&document, &detail),
                Err(e) => web_sys::console::error_1(&JsValue::from_str(&format!("parse error: {:?}", e))),
            }
        }
        Err(e) => {
            web_sys::console::error_1(&JsValue::from_str(&format!("fetch error: {:?}", e)));
        }
    }
}

async fn convert_to_honey_detail(value: JsValue) -> Result<HoneyDetail, JsValue> {
    use wasm_bindgen::JsCast;
    let resp: Response = value
        .dyn_into()
        .map_err(|_| JsValue::from_str("Expected Response"))?;
    let json = JsFuture::from(resp.json()?).await?;

    serde_wasm_bindgen::from_value(json).map_err(|err| {
        JsValue::from_str(&format!(
            "Failed to convert JsValue to HoneyDetail: {:?}",
            err
        ))
    })
}

fn set_text(document: &Document, id: &str, text: &str) {
    if let Some(el) = document.get_element_by_id(id) {
        el.set_text_content(Some(text));
    }
}

fn render_detail(document: &Document, detail: &HoneyDetail) {
    // タイトル
    set_text(document, "honey_title", &detail.basic.name_jp.0);

    // 基本情報
    set_text(document, "field_name_jp", &detail.basic.name_jp.0);
    if let Some(bk) = &detail.basic.beekeeper_name {
        set_text(document, "field_beekeeper", &bk.0);
    }
    if let Some(year) = detail.basic.harvest_year {
        set_text(document, "field_harvest_year", &format!("{}年", year));
    }
    if let Some(country) = &detail.basic.country {
        set_text(document, "field_country", &country.0);
    }
    if let Some(region) = &detail.basic.region {
        set_text(document, "field_region", &region.0);
    }
    if let Some(pd) = &detail.basic.purchase_date {
        // YYYY-MM-DD 抜粋
        let s = pd.to_rfc3339();
        let date_str = if s.len() >= 10 { &s[0..10] } else { &s };
        set_text(document, "field_purchase_date", date_str);
    }

    // 花タグ
    if let Some(container) = document.get_element_by_id("field_flowers") {
        let doc = document.clone();
        // 既存クリア
        container.set_inner_html("");
        for f in &detail.basic.flower_names {
            let span = doc.create_element("span").unwrap();
            span.set_class_name("tag");
            span.set_text_content(Some(&f.0));
            let _ = container.append_child(&span);
        }
    }
}
