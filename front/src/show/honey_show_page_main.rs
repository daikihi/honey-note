use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Document, Window, Response};

use crate::commons::ajax::{get_list_data, check_authentication};

use common_type::models::honey_detail::HoneyDetail;

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

    // 動的情報（バッチ・風味）の表示
    if let Some(dynamic_container) = document.get_element_by_id("dynamic_info_container") {
        let doc = document.clone();
        dynamic_container.set_inner_html("");
        for (i, d) in detail.dynamic.iter().enumerate() {
            let section = doc.create_element("div").unwrap();
            section.set_class_name("section");
            let date_str = d.created_at.map(|dt| dt.format("%Y-%m-%d").to_string()).unwrap_or_else(|| format!("記録 {}", i + 1));
            
            let mut html = format!("<h2>バッチ記録: {}</h2>", date_str);
            html.push_str("<table>");
            if let Some(color) = &d.color_feature { html.push_str(&format!("<tr><th>色</th><td>{:?}</td></tr>", color)); }
            if let Some(note) = &d.memo { html.push_str(&format!("<tr><th>メモ</th><td>{}</td></tr>", note.0)); }
            html.push_str("</table>");
            section.set_inner_html(&html);
            let _ = dynamic_container.append_child(&section);
        }
    }
}
