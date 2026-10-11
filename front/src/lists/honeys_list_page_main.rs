use wasm_bindgen::JsValue;
use web_sys::{Document, Window};

use crate::commons::{ajax::{get_list_data, check_authentication}, validators::is_valid_path};

use common_type::models::honey::Honey as ModelHoney;

pub async fn run() {
    // 認証チェック
    if let Err(_) = check_authentication().await {
        return;
    }
    
    web_sys::console::log_1(&"run main".into());
    if is_valid_path("/honey_note/honeys/lists.html") {
        web_sys::console::log_1(&"Flower List Page is running.".into());
        let _ = main_work().await;
    } else {
        web_sys::console::error_1(&format!("Unexpected path").into());
        panic!("Unexpected path");
    }
}

async fn main_work() {
    let _flower_api_path: &str = "/honey-note/api/honeys";
    let result: Result<JsValue, JsValue> = get_list_data(_flower_api_path).await;
    match result {
        Ok(values) => match convert_js_value_to_honey_list_data(values).await {
            Ok(honeys) => write_honeys_to_table(honeys),
            Err(e) => {
                web_sys::console::error_1(
                    &format!("Error converting honeys list data: {:?}", e).into(),
                );
            }
        },
        Err(e) => {
            web_sys::console::error_1(
                &format!("Error converting flower list data: {:?}", e).into(),
            );
        }
    }
}

async fn convert_js_value_to_honey_list_data(value: JsValue) -> Result<Vec<ModelHoney>, JsValue> {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::Response;
    let resp: Response = value
        .dyn_into()
        .map_err(|_| JsValue::from_str("Expected Response"))?;
    let json = JsFuture::from(resp.json()?).await?;

    serde_wasm_bindgen::from_value(json).map_err(|err| {
        JsValue::from_str(&format!(
            "Failed to convert JsValue to Vec<ModelHoney>: {:?}",
            err
        ))
    })
}

fn write_honeys_to_table(honeys: Vec<ModelHoney>) {
    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");
    let tbody = document
        .get_element_by_id("honeys_list_table_body")
        .unwrap();

    for honey in honeys {
        let row = document.create_element("tr").unwrap();
        let id = honey.id.unwrap_or(0);
        let link = format!("/honey_note/honeys/show.html?id={}", id);
        let beekeeper_name = honey
            .beekkeeper
            .map(|beekeeper| beekeeper.name_jp)
            .unwrap_or_default();
        let flower_names = honey.flower_names.join("、");
        let harvest_year = honey
            .harvest_year
            .map(|year| format!("{year}年"))
            .unwrap_or_else(|| "-".to_string());

        append_link_cell(&document, &row, &link, &id.to_string());
        append_link_cell(&document, &row, &link, &honey.name_jp);
        append_link_cell(&document, &row, &link, "");
        append_link_cell(&document, &row, &link, &beekeeper_name);
        append_link_cell(&document, &row, &link, &flower_names);
        append_link_cell(&document, &row, &link, &harvest_year);
        tbody.append_child(&row).unwrap();
    }
}

fn append_link_cell(document: &Document, row: &web_sys::Element, href: &str, text: &str) {
    let cell = document.create_element("td").unwrap();
    let link = document.create_element("a").unwrap();
    link.set_attribute("href", href).unwrap();
    link.set_text_content(Some(text));
    cell.append_child(&link).unwrap();
    row.append_child(&cell).unwrap();
}
