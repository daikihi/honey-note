use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    js_sys::{JsString, Reflect},
    Request, RequestInit, Window, Response, HtmlElement
};
use wasm_bindgen::JsCast;

pub async fn check_authentication() -> Result<(), JsValue> {
    let window: Window = web_sys::window().ok_or("no global `window` exists")?;
    let document = window.document().ok_or("no document exists")?;
    let me_api_path = "/api/auth/me";

    let opts = RequestInit::new();
    Reflect::set(&opts, &JsString::from("method"), &JsString::from("GET"))?;

    let request = Request::new_with_str_and_init(me_api_path, &opts)?;
    request.headers().set("Content-Type", "application/json")?;

    let response_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = response_value.dyn_into()?;

    if resp.status() == 200 {
        let json = JsFuture::from(resp.json()?).await?;
        let logged_in = Reflect::get(&json, &JsValue::from_str("logged_in"))?
            .as_bool()
            .unwrap_or(false);

        if !logged_in {
            window.location().assign("/honey_note/login.html")?;
            return Err(JsValue::from_str("not logged in"));
        }

        // ログイン済みならヘッダーを調整
        if let Some(username) = Reflect::get(&json, &JsValue::from_str("username"))?.as_string() {
            setup_header_user_info(&document, &username).await?;
        }
    } else {
        window.location().assign("/honey_note/login.html")?;
        return Err(JsValue::from_str("failed to fetch auth info"));
    }

    Ok(())
}

async fn setup_header_user_info(document: &web_sys::Document, username: &str) -> Result<(), JsValue> {
    if let Some(nav) = document.query_selector("header nav")? {
        // すでにユーザー情報が表示されているか確認（重複防止）
        if document.get_element_by_id("header_user_info").is_some() {
            return Ok(());
        }

        let user_info_div = document.create_element("div")?;
        user_info_div.set_id("header_user_info");
        user_info_div.set_attribute("style", "display: flex; align-items: center; gap: 1rem; font-size: 0.9rem; color: #666;")?;
        
        let span = document.create_element("span")?;
        span.set_text_content(Some(&format!("ユーザー: {}", username)));
        
        let logout_btn = document.create_element("button")?;
        logout_btn.set_text_content(Some("ログアウト"));
        logout_btn.set_attribute("style", "padding: 4px 8px; border: 1px solid #ccc; border-radius: 4px; background: #eee; cursor: pointer;")?;
        
        let closure = Closure::wrap(Box::new(move || {
            wasm_bindgen_futures::spawn_local(async move {
                let _ = logout().await;
            });
        }) as Box<dyn FnMut()>);
        
        logout_btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();

        let _ = user_info_div.append_child(&span)?;
        let _ = user_info_div.append_child(&logout_btn)?;
        let _ = nav.append_child(&user_info_div)?;
    }
    Ok(())
}

pub async fn logout() -> Result<(), JsValue> {
    let window: Window = web_sys::window().ok_or("no global `window` exists")?;
    let logout_api_path = "/api/auth/logout";

    let opts = RequestInit::new();
    Reflect::set(&opts, &JsString::from("method"), &JsString::from("POST"))?;

    let request = Request::new_with_str_and_init(logout_api_path, &opts)?;
    let _ = JsFuture::from(window.fetch_with_request(&request)).await?;

    window.location().assign("/honey_note/login.html")?;
    Ok(())
}

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
