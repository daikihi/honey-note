use common_type::models::flowers::Flower as ModelFlower;
use wasm_bindgen::JsValue;
use web_sys::{Document, Window};

use crate::commons::{
    ajax::{check_authentication, get_list_data},
    validators::is_valid_path,
};

pub async fn run() {
    // 認証チェック
    if let Err(_) = check_authentication().await {
        return;
    }

    if is_valid_path("/honey_note/flowers/lists.html") {
        web_sys::console::log_1(&"Flower List Page is running.".into());
        let _ = main_work().await;
    } else {
        web_sys::console::error_1(&format!("Unexpected path").into());
        panic!("Unexpected path");
    }
}

async fn main_work() {
    let _flower_api_path: &str = "/honey-note/api/flowers";
    let result: Result<JsValue, JsValue> = get_list_data(_flower_api_path).await;
    match result {
        Ok(value) => {
            web_sys::console::log_1(&format!("Flower List Data: {:?}", value).into());
            let flower_list: Result<Vec<ModelFlower>, JsValue> =
                convert_js_value_to_flower_list_data(value).await;
            match flower_list {
                Ok(data) => {
                    web_sys::console::log_1(
                        &format!("Converted Flower List Data: {:?}", data).into(),
                    );
                    write_flowers_to_table(data);
                    // Here you can use the `data` as needed, e.g., rendering it in the UI
                }
                Err(err) => {
                    web_sys::console::error_1(
                        &format!("Error converting flower list data: {:?}", err).into(),
                    );
                }
            }
        }
        Err(err) => {
            web_sys::console::error_1(&format!("Error fetching flower list: {:?}", err).into());
            return;
        }
    }
}

async fn convert_js_value_to_flower_list_data(value: JsValue) -> Result<Vec<ModelFlower>, JsValue> {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::Response;
    let resp: Response = value
        .dyn_into()
        .map_err(|_| JsValue::from_str("Expected Response"))?;
    let json = JsFuture::from(resp.json()?).await?;

    serde_wasm_bindgen::from_value(json).map_err(|err| {
        JsValue::from_str(&format!(
            "Failed to convert JsValue to Vec<ModelFlower>: {:?}",
            err
        ))
    })
}

/// 蜜源の一覧を詳細・編集リンクとともに表へ追加する。
fn write_flowers_to_table(flowers: Vec<ModelFlower>) {
    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");
    let tbody = document
        .get_element_by_id("flower_list_table_body")
        .unwrap();

    for flower in flowers {
        let row = document.create_element("tr").unwrap();
        let id = flower.id.unwrap_or_default() as i64;
        let id_text = id.to_string();
        append_text_cell(&document, &row, &id_text);
        append_text_cell(&document, &row, &flower.name_jp);
        append_text_cell(&document, &row, flower.name_en.as_deref().unwrap_or(""));

        let action_cell = document.create_element("td").unwrap();
        append_action_link(
            &document,
            &action_cell,
            "詳細",
            &format!("/honey_note/flowers/show.html?id={id}"),
        );
        let separator = document.create_element("span").unwrap();
        separator.set_text_content(Some(" | "));
        action_cell.append_child(&separator).unwrap();
        append_action_link(
            &document,
            &action_cell,
            "編集",
            &format!("/honey_note/flowers/edit.html?id={id}"),
        );
        row.append_child(&action_cell).unwrap();
        tbody.append_child(&row).unwrap();
    }
}

fn append_text_cell(document: &Document, row: &web_sys::Element, text: &str) {
    let cell = document.create_element("td").unwrap();
    cell.set_text_content(Some(text));
    row.append_child(&cell).unwrap();
}

fn append_action_link(document: &Document, cell: &web_sys::Element, text: &str, href: &str) {
    let link = document.create_element("a").unwrap();
    link.set_text_content(Some(text));
    link.set_attribute("href", href).unwrap();
    cell.append_child(&link).unwrap();
}
