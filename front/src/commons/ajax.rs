// use wasm_bindgen::{JsCast, JsValue};
// use web_sys::{Request, RequestInit, RequestMode, Response};
// use wasm_bindgen_futures::JsFuture;

// pub async fn request_get_httprequest_get_as_json(url: &str) -> Result<JsValue, JsValue> {
//     let mut opts = RequestInit::new();
//     opts.method("GET");
//     opts.mode(RequestMode::Cors);

//     let request = Request::new_with_str_and_init(url, &opts)?;
//     request.headers().set("Accept", "application/json")?;

//     let window = web_sys::window().unwrap();
//     let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

//     let resp: Response = resp_value.dyn_into().unwrap();
//     if resp.ok() {
//         let json = JsFuture::from(resp.json()?).await?;
//         Ok(json)
//     } else {
//         Err(JsValue::from_str(&format!(
//             "Request failed with status: {}",
//             resp.status()
//         )))
//     }
// }
