use common_type::models::flowers::Flower as ModelFlower;
use wasm_bindgen::JsValue;
use web_sys::{
    js_sys::{JsString, Reflect},
    Request, RequestInit,
};
use web_sys::{Document, Window};

use crate::commons::{self, ajax::get_list_data, validators::is_valid_path};

pub async fn run() {
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

fn write_flowers_to_table(flowers: Vec<ModelFlower>) {
    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");
    let tbody = document
        .get_element_by_id("flower_list_table_body")
        .unwrap();

    for flower in flowers {
        let row = document.create_element("tr").unwrap();
        row.set_inner_html(&format!(
            "<td>{}</td><td>{}</td><td>{}</td><td>{}</td>",
            flower.id.unwrap_or(0.0),
            flower.name_jp,
            flower.name_en.unwrap_or_default(),
            flower.short_note.unwrap_or_default()
        ));
        tbody.append_child(&row).unwrap();
    }
}

fn print_message(message: &JsValue) {
    web_sys::console::log_2(&"Fetched data:".into(), &message);

    use web_sys::console;
    use web_sys::js_sys::JSON;

    match JSON::stringify_with_replacer_and_space(message, &JsValue::NULL, &JsValue::from_str("  "))
    {
        Ok(pretty) => console::log_1(&pretty),
        Err(_) => console::log_1(&"Failed to stringify JsValue".into()),
    }
}
