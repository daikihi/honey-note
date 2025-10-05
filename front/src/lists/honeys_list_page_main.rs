use wasm_bindgen::{prelude::Closure, JsCast as _, JsValue};
use web_sys::{Document, Window};

use crate::commons::{ajax::get_list_data, validators::is_valid_path};

use common_type::models::honey::Honey as ModelHoney;

pub async fn run() {
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
        web_sys::console::log_1(&"run main".into());

        let row = document.create_element("tr").unwrap();
        let link_to = format!(
            "/honey_note/honeys/detail.html?id={}",
            honey.id.unwrap_or(0)
        );
        row.set_inner_html(&format!(
            "<td>{0}</td><td>{1}</td><td>{2}</td><td>{3}</td><td>{4}</td><td>{5}</td>",
            honey.id.unwrap_or(0),
            honey.name_jp,
            honey.name_en.unwrap_or_default(),
            honey.beekkeeper.map(|b| b.name_jp).unwrap_or_default(),
            "",
            honey.purchase_date.unwrap_or_default()
        ));
        let closure = Closure::wrap(Box::new(move || {
            web_sys::window()
                .unwrap()
                .location()
                .set_href(&link_to)
                .unwrap();
            web_sys::console::log_1(&"clicked!!!".into());
        }) as Box<dyn FnMut()>);

        row.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap();

        // ここがポイント！クロージャを忘れることでJS側に保持させる
        closure.forget();
        tbody.append_child(&row).unwrap();
    }
}
