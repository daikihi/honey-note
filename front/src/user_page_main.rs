use common_type::models::user::DISPLAY_NAME_MAX_CHARS;
use std::{cell::Cell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    js_sys::{JsString, Object, Reflect, JSON},
    Document, HtmlElement, HtmlFormElement, HtmlInputElement, Request, RequestInit, Response,
    Window,
};

const SHOW_PATH: &str = "/honey_note/users/show.html";
const LOGIN_PATH: &str = "/honey_note/login.html";

fn field(json: &JsValue, name: &str) -> Result<Option<String>, JsValue> {
    Ok(Reflect::get(json, &JsValue::from_str(name))?.as_string())
}

fn element(document: &Document, id: &str) -> Result<web_sys::Element, JsValue> {
    document
        .get_element_by_id(id)
        .ok_or_else(|| JsValue::from_str(id))
}

fn error(document: &Document, message: &str) {
    if let Some(node) = document.get_element_by_id("error_message") {
        node.set_text_content(Some(message));
        let _ = node
            .dyn_into::<HtmlElement>()
            .map(|node| node.style().set_property("display", "block"));
    }
}

async fn current_user(window: &Window) -> Result<JsValue, JsValue> {
    let resp = JsFuture::from(window.fetch_with_str("/api/auth/me"))
        .await?
        .dyn_into::<Response>()?;
    if resp.status() == 401 {
        window.location().assign(LOGIN_PATH)?;
        return Err(JsValue::from_str("login required"));
    }
    if !resp.ok() {
        return Err(JsValue::from_str("ユーザー情報を取得できませんでした"));
    }
    let json = JsFuture::from(resp.json()?).await?;
    if Reflect::get(&json, &JsValue::from_str("logged_in"))?.as_bool() != Some(true) {
        window.location().assign(LOGIN_PATH)?;
        return Err(JsValue::from_str("login required"));
    }
    Ok(json)
}

async fn initialize() -> Result<(Window, Document, JsValue), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("window unavailable"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("document unavailable"))?;
    let json = current_user(&window).await?;
    if let Some(name) = crate::commons::ajax::header_display_name(&json)? {
        crate::commons::ajax::setup_header_user_info(&document, &name).await?;
    }
    Ok((window, document, json))
}

pub async fn show() {
    match initialize().await {
        Ok((_, document, json)) => {
            let result = (|| -> Result<(), JsValue> {
                element(&document, "username")?
                    .set_text_content(field(&json, "username")?.as_deref());
                element(&document, "display_name")?.set_text_content(Some(
                    field(&json, "display_name")?.as_deref().unwrap_or("-"),
                ));
                Ok(())
            })();
            if result.is_err() {
                error(&document, "ユーザー情報を表示できませんでした");
            }
        }
        Err(_) => {
            if let Some(document) = web_sys::window().and_then(|window| window.document()) {
                error(&document, "ユーザー情報を取得できませんでした");
            }
        }
    }
}

pub async fn edit() {
    let (window, document, json) = match initialize().await {
        Ok(value) => value,
        Err(_) => {
            if let Some(document) = web_sys::window().and_then(|window| window.document()) {
                error(&document, "ユーザー情報を取得できませんでした");
            }
            return;
        }
    };
    let result = (|| -> Result<(), JsValue> {
        let input = element(&document, "display_name")?.dyn_into::<HtmlInputElement>()?;
        let original = field(&json, "display_name")?.unwrap_or_default();
        input.set_value(&original);
        let form = element(&document, "display_name_form")?.dyn_into::<HtmlFormElement>()?;
        let save = element(&document, "btn_save")?.dyn_into::<HtmlElement>()?;
        let busy = Rc::new(Cell::new(false));
        let event_document = document.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            event.prevent_default();
            if busy.replace(true) {
                return;
            }
            let input = input.clone();
            let original = original.clone();
            let save = save.clone();
            let busy = busy.clone();
            let window = window.clone();
            let document = event_document.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let value = input.value().trim().to_string();
                if value.is_empty() {
                    error(&document, "表示名を入力してください");
                } else if value.chars().count() > DISPLAY_NAME_MAX_CHARS {
                    error(&document, "表示名は50文字以内にしてください");
                } else if value == original {
                    let _ = window.location().assign(SHOW_PATH);
                } else {
                    let _ = save.set_attribute("disabled", "");
                    match save_display_name(&window, &value).await {
                        Ok(()) => {
                            let _ = window.location().assign(SHOW_PATH);
                        }
                        Err(message) => error(&document, &message),
                    }
                    let _ = save.remove_attribute("disabled");
                }
                busy.set(false);
            });
        }) as Box<dyn FnMut(_)>);
        form.add_event_listener_with_callback("submit", closure.as_ref().unchecked_ref())?;
        closure.forget();
        Ok(())
    })();
    if result.is_err() {
        error(&document, "編集画面を表示できませんでした");
    }
}

async fn save_display_name(window: &Window, name: &str) -> Result<(), String> {
    let body = Object::new();
    Reflect::set(
        &body,
        &JsValue::from_str("display_name"),
        &JsValue::from_str(name),
    )
    .map_err(|_| "表示名の送信に失敗しました".to_string())?;
    let body = JSON::stringify(&body).map_err(|_| "表示名の送信に失敗しました".to_string())?;
    let opts = RequestInit::new();
    Reflect::set(&opts, &JsString::from("method"), &JsString::from("PUT"))
        .map_err(|_| "表示名の送信に失敗しました".to_string())?;
    Reflect::set(&opts, &JsString::from("body"), &body)
        .map_err(|_| "表示名の送信に失敗しました".to_string())?;
    let request = Request::new_with_str_and_init("/api/auth/me/display-name", &opts)
        .map_err(|_| "表示名の送信に失敗しました".to_string())?;
    request
        .headers()
        .set("Content-Type", "application/json")
        .map_err(|_| "表示名の送信に失敗しました".to_string())?;
    let resp = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| "通信に失敗しました。再度お試しください".to_string())?
        .dyn_into::<Response>()
        .map_err(|_| "通信に失敗しました".to_string())?;
    if resp.ok() {
        return Ok(());
    }
    if resp.status() == 401 {
        return Err("ログインが必要です。ログイン画面から再度お試しください".to_string());
    }
    let message = match resp.json() {
        Ok(json) => JsFuture::from(json)
            .await
            .ok()
            .and_then(|json| field(&json, "message").ok().flatten()),
        Err(_) => None,
    };
    Err(message.unwrap_or_else(|| "表示名の保存に失敗しました".to_string()))
}
