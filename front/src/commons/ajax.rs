use wasm_bindgen::JsValue;
use web_sys::XmlHttpRequest;

pub async fn request_get_httprequest_get_as_json(url: &str) -> Result<String, JsValue> {
    let response = XmlHttpRequest::new()?;
    response.open("GET", url)?;
    response.set_request_header("Accept", "application/json")?;
    response.send()?;
    if response.status() == Ok(200) {
        let response_text_result = response.response_text();
        match response_text_result {
            Ok(text_opt) => match text_opt {
                Some(text) => {
                    let json: Result<String, serde_json::Error> = serde_json::from_str(&text);
                    match json {
                        Ok(json) => Ok(json),
                        Err(_) => Err(JsValue::from_str("Failed to parse JSON")),
                    }
                }
                None => return Err(JsValue::from_str("Response text is empty")),
            },
            Err(_) => return Err(JsValue::from_str("Failed to get response text")),
        }
    } else {
        Err(JsValue::from_str(&format!(
            "Request failed with status: {}",
            response.status()?
        )))
    }
}
