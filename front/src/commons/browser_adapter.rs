use async_trait::async_trait;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response, Window};

#[async_trait(?Send)]
pub trait BrowserAdapter {
    /// 通信（Fetch）を抽象化
    async fn fetch(
        &self,
        url: &str,
        method: &str,
        body: Option<String>,
    ) -> Result<Response, JsValue>;

    /// 遷移（Location）を抽象化
    fn redirect(&self, url: &str) -> Result<(), JsValue>;
}

pub struct WebBrowserAdapter;

#[async_trait(?Send)]
impl BrowserAdapter for WebBrowserAdapter {
    async fn fetch(
        &self,
        url: &str,
        method: &str,
        body: Option<String>,
    ) -> Result<Response, JsValue> {
        let window: Window =
            web_sys::window().ok_or_else(|| JsValue::from_str("no global `window` exists"))?;

        let opts: RequestInit = RequestInit::new();
        web_sys::js_sys::Reflect::set(
            &opts,
            &JsValue::from_str("method"),
            &JsValue::from_str(method),
        )?;

        if let Some(b) = body {
            web_sys::js_sys::Reflect::set(
                &opts,
                &JsValue::from_str("body"),
                &JsValue::from_str(&b),
            )?;
        }

        let request: Request = Request::new_with_str_and_init(url, &opts)?;
        if method == "POST" || method == "PUT" {
            request.headers().set("Content-Type", "application/json")?;
        }

        let response_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: Response = response_value.dyn_into()?;

        Ok(resp)
    }

    fn redirect(&self, url: &str) -> Result<(), JsValue> {
        let window: Window =
            web_sys::window().ok_or_else(|| JsValue::from_str("no global `window` exists"))?;
        window.location().assign(url)
    }
}
