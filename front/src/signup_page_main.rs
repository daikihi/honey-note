use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    js_sys::{JsString, Reflect},
    Document, Window, HtmlFormElement, HtmlInputElement, Request, RequestInit, Response, HtmlElement
};

pub async fn run() {
    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");

    // すでにログインしているか確認
    if let Ok(_) = check_already_logged_in(&window).await {
        // ログイン済みならトップへ
        let _ = window.location().assign("/honey_note/index.html");
        return;
    }

    let form = document.get_element_by_id("signup_form").unwrap().dyn_into::<HtmlFormElement>().unwrap();
    let error_div = document.get_element_by_id("error_message").unwrap().dyn_into::<HtmlElement>().unwrap();

    let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
        event.prevent_default();
        let form = form.clone();
        let error_div = error_div.clone();
        wasm_bindgen_futures::spawn_local(async move {
            if let Err(e) = handle_signup(&form, &error_div).await {
                error_div.set_text_content(Some(&format!("エラーが発生しました: {:?}", e)));
                let _ = error_div.style().set_property("display", "block");
            }
        });
    }) as Box<dyn FnMut(_)>);

    document.get_element_by_id("signup_form").unwrap().add_event_listener_with_callback("submit", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

async fn check_already_logged_in(window: &Window) -> Result<bool, JsValue> {
    let opts = RequestInit::new();
    Reflect::set(&opts, &JsString::from("method"), &JsString::from("GET"))?;

    let request = Request::new_with_str_and_init("/api/auth/me", &opts)?;
    let response_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = response_value.dyn_into()?;

    if resp.status() == 200 {
        let json = JsFuture::from(resp.json()?).await?;
        let logged_in = Reflect::get(&json, &JsValue::from_str("logged_in"))?.as_bool().unwrap_or(false);
        if logged_in {
            return Ok(true);
        }
    }
    Err(JsValue::from_str("not logged in"))
}

async fn handle_signup(form: &HtmlFormElement, error_div: &HtmlElement) -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = web_sys::window().unwrap().document().unwrap();

    let username = document.get_element_by_id("username").unwrap().dyn_into::<HtmlInputElement>().unwrap().value();
    let email = document.get_element_by_id("email").unwrap().dyn_into::<HtmlInputElement>().unwrap().value();
    let password = document.get_element_by_id("password").unwrap().dyn_into::<HtmlInputElement>().unwrap().value();
    let confirm_password = document.get_element_by_id("confirm_password").unwrap().dyn_into::<HtmlInputElement>().unwrap().value();

    // パスワードが一致するか確認
    if password != confirm_password {
        error_div.set_text_content(Some("パスワードが一致しません"));
        let _ = error_div.style().set_property("display", "block");
        return Err(JsValue::from_str("パスワードが一致しません"));
    }

    let body_obj = web_sys::js_sys::Object::new();
    Reflect::set(&body_obj, &JsValue::from_str("username"), &JsValue::from_str(&username))?;
    Reflect::set(&body_obj, &JsValue::from_str("email"), &JsValue::from_str(&email))?;
    Reflect::set(&body_obj, &JsValue::from_str("password"), &JsValue::from_str(&password))?;
    let body_str = web_sys::js_sys::JSON::stringify(&body_obj)?;

    let opts = RequestInit::new();
    Reflect::set(&opts, &JsString::from("method"), &JsString::from("POST"))?;
    Reflect::set(&opts, &JsString::from("body"), &body_str)?;

    let request = Request::new_with_str_and_init("/api/auth/signup", &opts)?;
    request.headers().set("Content-Type", "application/json")?;

    let response_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = response_value.dyn_into()?;

    if resp.status() == 200 || resp.status() == 201 {
        // 新規登録成功後、ログインページへリダイレクト
        window.location().assign("/honey_note/login.html")?;
    } else {
        let json = JsFuture::from(resp.json()?).await?;
        let message = Reflect::get(&json, &JsValue::from_str("message"))?.as_string().unwrap_or_else(|| "新規登録に失敗しました".to_string());
        error_div.set_text_content(Some(&message));
        let _ = error_div.style().set_property("display", "block");
    }

    Ok(())
}

