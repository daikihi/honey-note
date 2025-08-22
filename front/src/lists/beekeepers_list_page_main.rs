use crate::commons::{ajax::get_list_data, validators::is_valid_path};
use common_type::models::beekeeper::Beekeeper as ModelBeekeeper;
use common_type::models::prefectures::Prefecture as ModelPrefecture;
use wasm_bindgen::JsValue;
use web_sys::{Document, Window};

pub async fn run() {
    if is_valid_path("/honey_note/beekeepers/lists.html") {
        web_sys::console::log_1(&"Beekeepers List Page is running.".into());
        let _ = main_work().await;
    } else {
        web_sys::console::error_1(&format!("Unexpected path").into());
        panic!("Unexpected path");
    }
}

async fn main_work() {
    let _beekeepers_api_path: &str = "/honey-note/api/beekeepers";
    let _prefectures_api_path: &str = "/honey-note/api/prefectures";

    let prefectures_result: Result<JsValue, JsValue> = get_list_data(_prefectures_api_path).await;
    let prefecture_list: Vec<ModelPrefecture> = match prefectures_result {
        Ok(value) => {
            match convert_js_value_to_prefecture_list_data(value).await {
            Ok(data) => data,
            Err(err) => {
                // web_sys::console::error_1(&format!("Error fetching prefectures: {:?}", err).into());
                vec![]
            },
        }},
        Err(err) => {
            web_sys::console::error_1(&format!("Error fetching prefectures: {:?}", err).into());
            vec![]
        }
    };

    let result: Result<JsValue, JsValue> = get_list_data(_beekeepers_api_path).await;
    match result {
        Ok(value) => {
            web_sys::console::log_1(&format!("Beekeepers List Data: {:?}", value).into());
            let beekeepers_list: Result<Vec<ModelBeekeeper>, JsValue> =
                convert_js_value_to_beekeepers_list_data(value).await;
            match beekeepers_list {
                Ok(data) => {
                    web_sys::console::log_1(
                        &format!("Converted Beekeepers List Data: {:?}", data).into(),
                    );
                    write_beekeepers_to_table(data, prefecture_list.clone());
                }
                Err(err) => {
                    web_sys::console::error_1(
                        &format!("Error converting beekeepers list data: {:?}", err).into(),
                    );
                }
            }
        }
        Err(err) => {
            web_sys::console::error_1(&format!("Error fetching beekeepers list: {:?}", err).into());
            return;
        }
    }
}

async fn convert_js_value_to_prefecture_list_data(
    value: JsValue,
) -> Result<Vec<ModelPrefecture>, JsValue> {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::Response;

    let resp: Response = value
        .dyn_into()
        .map_err(|_| JsValue::from_str("Expected Response"))?;

    let json = JsFuture::from(resp.json()?).await?;

    serde_wasm_bindgen::from_value(json).map_err(|err| {
        JsValue::from_str(&format!(
            "Failed to convert JsValue to Vec<Prefecture>: {:?}",
            err
        ))
    })
}

async fn convert_js_value_to_beekeepers_list_data(
    value: JsValue,
) -> Result<Vec<ModelBeekeeper>, JsValue> {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::Response;
    let resp: Response = value
        .dyn_into()
        .map_err(|_| JsValue::from_str("Expected Response"))?;
    let json = JsFuture::from(resp.json()?).await?;

    serde_wasm_bindgen::from_value(json).map_err(|err| {
        JsValue::from_str(&format!(
            "Failed to convert JsValue to Vec<ModelBeekeeper>: {:?}",
            err
        ))
    })
}

fn write_beekeepers_to_table(beekeepers: Vec<ModelBeekeeper>, prefectures: Vec<ModelPrefecture>) {
    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");
    let tbody = document
        .get_element_by_id("beekeepers_list_table_body")
        .unwrap();

    // This function would contain the logic to write the beekeepers data to a table in the UI.
    // For now, we will just log the data.
    for beekeeper in beekeepers {
        let matched_prefecture: Option<&ModelPrefecture> = prefectures.iter().find(|p| {
            match beekeeper.location_prefecture_id {
                Some(id) => p.id == id,
                None => false,
            }
        });
        let row = document.create_element("tr").unwrap();
        row.set_inner_html(&format!(
            "<td>{}</td><td>{}</td><td>{}</td>",
            beekeeper.id.unwrap_or_default(),
            beekeeper.name_jp,
            matched_prefecture.map_or("Unknown".to_string(), |p| p.name_jp.clone()),
        ));
        tbody.append_child(&row).unwrap();
    }
}
