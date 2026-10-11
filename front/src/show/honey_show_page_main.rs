use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Document, Response, Window};

use crate::commons::ajax::{check_authentication, get_list_data};

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
        Ok(resp_js) => match convert_to_honey_detail(resp_js).await {
            Ok(detail) => render_detail(&document, &detail),
            Err(e) => {
                web_sys::console::error_1(&JsValue::from_str(&format!("parse error: {:?}", e)))
            }
        },
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
        container.set_text_content(None);
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
        dynamic_container.set_text_content(None);
        for (i, d) in detail.dynamic.iter().enumerate() {
            let section = doc.create_element("div").unwrap();
            section.set_class_name("section");
            let date_str = d
                .created_at
                .map(|dt| dt.format("%Y-%m-%d").to_string())
                .unwrap_or_else(|| format!("記録 {}", i + 1));
            let heading = doc.create_element("h2").unwrap();
            heading.set_text_content(Some(&format!("バッチ記録: {}", date_str)));
            let table = doc.create_element("table").unwrap();
            if let Some(color) = &d.color_feature {
                append_detail_row(&doc, &table, "色", &format!("{:?}", color));
            }
            if let Some(note) = &d.memo {
                append_detail_row(&doc, &table, "メモ", &note.0);
            }
            let _ = section.append_child(&heading);
            let _ = section.append_child(&table);
            let _ = dynamic_container.append_child(&section);
        }
    }
}

fn append_detail_row(document: &Document, table: &web_sys::Element, label: &str, value: &str) {
    let row = document.create_element("tr").unwrap();
    let header = document.create_element("th").unwrap();
    header.set_text_content(Some(label));
    let cell = document.create_element("td").unwrap();
    cell.set_text_content(Some(value));
    let _ = row.append_child(&header);
    let _ = row.append_child(&cell);
    let _ = table.append_child(&row);
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn renders_dynamic_branches_as_text_and_clears_previous_content() {
        let document = web_sys::window().unwrap().document().unwrap();
        let root = document.create_element("div").unwrap();
        for id in ["honey_title", "field_flowers", "dynamic_info_container"] {
            let child = document.create_element("div").unwrap();
            child.set_id(id);
            root.append_child(&child).unwrap();
        }
        document.body().unwrap().append_child(&root).unwrap();

        let detail: HoneyDetail = serde_json::from_value(serde_json::json!({
            "basic": {"name_jp":"蜜", "flower_names":["<img src=x>"],
                "beekeeper_name":null, "harvest_year":null, "country":null,
                "region":null, "honey_type":null, "volume":null, "purchase_date":null},
            "dynamic": [
                {"created_at":"2026-10-10T12:00:00+09:00",
                 "color_feature":{"category":null,"hex":null,"note":null},
                 "memo":"<img src=x onerror=alert(1)>", "observations":[]},
                {"created_at":null, "color_feature":null, "memo":null, "observations":[]}
            ],
            "created_at":null, "updated_at":null
        }))
        .unwrap();
        render_detail(&document, &detail);
        let container = document
            .get_element_by_id("dynamic_info_container")
            .unwrap();
        let headings = container.get_elements_by_tag_name("h2");
        assert_eq!(headings.length(), 2);
        assert_eq!(
            headings.item(0).unwrap().text_content().as_deref(),
            Some("バッチ記録: 2026-10-10")
        );
        assert_eq!(
            headings.item(1).unwrap().text_content().as_deref(),
            Some("バッチ記録: 記録 2")
        );
        assert_eq!(container.get_elements_by_tag_name("tr").length(), 2);
        assert!(container
            .text_content()
            .unwrap()
            .contains("<img src=x onerror=alert(1)>"));
        assert_eq!(root.get_elements_by_tag_name("img").length(), 0);

        render_detail(&document, &detail);
        assert_eq!(container.get_elements_by_tag_name("h2").length(), 2);
        assert_eq!(
            document
                .get_element_by_id("field_flowers")
                .unwrap()
                .get_elements_by_tag_name("span")
                .length(),
            1
        );
        root.remove();
    }
}
