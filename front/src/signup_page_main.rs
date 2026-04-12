use crate::commons::browser_adapter::BrowserAdapter;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys::Reflect, Document, HtmlElement, HtmlFormElement, HtmlInputElement, Window};

pub async fn run<A: BrowserAdapter + 'static>(adapter: Rc<A>) -> Result<(), JsValue> {
    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");

    // すでにログインしているか確認
    if let Ok(_) = check_already_logged_in(&window, adapter.as_ref()).await {
        // ログイン済みならトップへ
        let _ = adapter.redirect("/honey_note/index.html");
        return Ok(());
    }

    // DOM要素の取得を安全に行う
    let form_element = document
        .get_element_by_id("signup_form")
        .ok_or_else(|| JsValue::from_str("signup_form element not found"))?
        .dyn_into::<HtmlFormElement>()
        .map_err(|_| JsValue::from_str("signup_form is not a form element"))?;

    let error_div = document
        .get_element_by_id("error_message")
        .ok_or_else(|| JsValue::from_str("error_message element not found"))?
        .dyn_into::<HtmlElement>()
        .map_err(|_| JsValue::from_str("error_message is not an element"))?;

    // イベントリスナーの追加も安全に行う
    let form_clone = form_element.clone();
    let error_div_clone = error_div.clone();
    let adapter_clone = Rc::clone(&adapter);
    let document_clone = document.clone();
    let window_clone = window.clone();

    let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
        event.prevent_default();
        let form = form_clone.clone();
        let error_div = error_div_clone.clone();
        let adapter = Rc::clone(&adapter_clone);
        let document = document_clone.clone();
        let window = window_clone.clone();

        wasm_bindgen_futures::spawn_local(async move {
            if let Err(e) =
                handle_signup(&form, &error_div, &document, &window, adapter.as_ref()).await
            {
                error_div.set_text_content(Some(&format!("エラーが発生しました: {:?}", e)));
                let _ = error_div.style().set_property("display", "block");
            }
        });
    }) as Box<dyn FnMut(_)>);

    form_element.add_event_listener_with_callback("submit", closure.as_ref().unchecked_ref())?;
    closure.forget();

    Ok(())
}

async fn check_already_logged_in<A: BrowserAdapter>(
    _window: &Window,
    adapter: &A,
) -> Result<bool, JsValue> {
    let resp = adapter.fetch("/api/auth/me", "GET", None).await?;

    if resp.status() == 200 {
        let json = JsFuture::from(resp.json()?).await?;
        let logged_in = Reflect::get(&json, &JsValue::from_str("logged_in"))?
            .as_bool()
            .unwrap_or(false);
        if logged_in {
            return Ok(true);
        }
    }
    Err(JsValue::from_str("not logged in"))
}

async fn handle_signup<A: BrowserAdapter>(
    _form: &HtmlFormElement,
    error_div: &HtmlElement,
    document: &Document,
    _window: &Window,
    adapter: &A,
) -> Result<(), JsValue> {
    let username_element = document
        .get_element_by_id("username")
        .ok_or_else(|| JsValue::from_str("username element not found"))?
        .dyn_into::<HtmlInputElement>()
        .map_err(|_| JsValue::from_str("username is not an input element"))?;
    let username = username_element.value();

    let email_element = document
        .get_element_by_id("email")
        .ok_or_else(|| JsValue::from_str("email element not found"))?
        .dyn_into::<HtmlInputElement>()
        .map_err(|_| JsValue::from_str("email is not an input element"))?;
    let email = email_element.value();

    let password_element = document
        .get_element_by_id("password")
        .ok_or_else(|| JsValue::from_str("password element not found"))?
        .dyn_into::<HtmlInputElement>()
        .map_err(|_| JsValue::from_str("password is not an input element"))?;
    let password = password_element.value();

    let confirm_password_element = document
        .get_element_by_id("confirm_password")
        .ok_or_else(|| JsValue::from_str("confirm_password element not found"))?
        .dyn_into::<HtmlInputElement>()
        .map_err(|_| JsValue::from_str("confirm_password is not an input element"))?;
    let confirm_password = confirm_password_element.value();

    // パスワードが一致するか確認
    if password != confirm_password {
        error_div.set_text_content(Some("エラーが発生しました"));
        let _ = error_div.style().set_property("display", "block");
        return Ok(());
    }

    let body_obj = web_sys::js_sys::Object::new();
    Reflect::set(
        &body_obj,
        &JsValue::from_str("username"),
        &JsValue::from_str(&username),
    )?;
    Reflect::set(
        &body_obj,
        &JsValue::from_str("email"),
        &JsValue::from_str(&email),
    )?;
    Reflect::set(
        &body_obj,
        &JsValue::from_str("password"),
        &JsValue::from_str(&password),
    )?;
    let body_str = web_sys::js_sys::JSON::stringify(&body_obj)?;

    let resp = adapter
        .fetch("/api/auth/signup", "POST", Some(body_str.into()))
        .await?;

    if resp.status() == 200 || resp.status() == 201 {
        // 新規登録成功後、ログインページへリダイレクト
        adapter.redirect("/honey_note/login.html")?;
    } else {
        let json = JsFuture::from(resp.json()?).await?;
        let message = Reflect::get(&json, &JsValue::from_str("message"))?
            .as_string()
            .unwrap_or_else(|| "新規登録に失敗しました".to_string());
        error_div.set_text_content(Some(&message));
        let _ = error_div.style().set_property("display", "block");
    }

    Ok(())
}