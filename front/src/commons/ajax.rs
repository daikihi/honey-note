use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    js_sys::{JsString, Reflect},
    Request, RequestInit, Window,
};

pub async fn get_list_data(_flower_api_path: &str) -> Result<JsValue, JsValue> {
    let opts = RequestInit::new();
    Reflect::set(&opts, &JsString::from("method"), &JsString::from("GET"))?;
    Reflect::set(&opts, &JsString::from("mode"), &JsString::from("cors"))?;
    Reflect::set(&opts, &JsString::from("keepalive"), &JsValue::from(true))?;

    let request = Request::new_with_str_and_init(_flower_api_path, &opts).unwrap();
    request.headers().set("Content-Type", "application/json")?;

    let window: Window = web_sys::window().ok_or("no global `window` exists")?;

    let response: Result<JsValue, JsValue> =
        JsFuture::from(window.fetch_with_request(&request)).await;

    match response {
        Ok(resp) => {
            // let _resp: Response = resp.clone().dyn_into().map_err(|_| JsValue::from_str("Expected Response"))?;
            // let json_value = JsFuture::from(_resp.json()?).await?;
            // web_sys::console::log_1(&json_value);

            // print_message(&json_value);
            Ok(resp)
        }
        Err(err) => {
            web_sys::console::error_1(&format!("Error fetching flowers: {:?}", err).into());
            Err(err)
        }
    }
}
