use crate::commons::browser_adapter::BrowserAdapter;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys::Reflect, Document, HtmlElement, HtmlFormElement, HtmlInputElement, Window};

/// Initialize the login page: redirect already-authenticated users and attach a submit handler to the login form.
///
/// This function acquires `window` and `document`, checks authentication via the provided `adapter`,
/// redirects to `/honey_note/index.html` when the user is already logged in, locates the login form
/// and error message elements, and registers a `submit` event listener that runs the asynchronous
/// login flow. The listener sets the error message element's text and makes it visible if the login
/// handler fails.
///
/// # Errors
///
/// Returns `Err(JsValue)` when required DOM elements are missing or have the wrong type, or when
/// registering the event listener fails. Redirect errors performed for already-logged-in users are ignored.
///
/// # Examples
///
/// ```no_run
/// use std::rc::Rc;
/// use wasm_bindgen::JsValue;
///
/// // `MyAdapter` should implement `BrowserAdapter`.
/// // let adapter = Rc::new(MyAdapter::new());
/// // wasm_bindgen_futures::spawn_local(async move {
/// //     let _ = crate::login_page_main::run(adapter).await;
/// // });
/// ```
pub async fn run<A: BrowserAdapter + 'static>(adapter: Rc<A>) -> Result<(), JsValue> {
    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");

    // すでにログインしているか確認
    if let Ok(_) = check_already_logged_in(&window, adapter.as_ref()).await {
        // ログイン済みならトップへ
        let _ = adapter.redirect("/honey_note/index.html");
        return Ok(());
    }

    let form = document
        .get_element_by_id("login_form")
        .ok_or_else(|| JsValue::from_str("login_form element not found"))?
        .dyn_into::<HtmlFormElement>()
        .map_err(|_| JsValue::from_str("login_form is not a form element"))?;

    let error_div = document
        .get_element_by_id("error_message")
        .ok_or_else(|| JsValue::from_str("error_message element not found"))?
        .dyn_into::<HtmlElement>()
        .map_err(|_| JsValue::from_str("error_message is not an element"))?;

    let form_clone = form.clone();
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
                handle_login(&form, &error_div, &document, &window, adapter.as_ref()).await
            {
                error_div.set_text_content(Some(&format!("エラーが発生しました: {:?}", e)));
                let _ = error_div.style().set_property("display", "block");
            }
        });
    }) as Box<dyn FnMut(_)>);

    form.add_event_listener_with_callback("submit", closure.as_ref().unchecked_ref())?;
    closure.forget();

    Ok(())
}

/// Checks whether the current user is already authenticated.
///
/// Sends a GET request to `/api/auth/me` via the provided adapter and returns success only when the response
/// has status 200 and its top-level `logged_in` property is `true`.
///
/// # Returns
///
/// `Ok(true)` if the adapter reports the user is logged in (response status 200 and `logged_in` equals `true`),
/// `Err(JsValue)` otherwise.
///
/// # Examples
///
/// ```no_run
/// # use std::rc::Rc;
/// # async fn _example<A: BrowserAdapter + 'static>(window: web_sys::Window, adapter: Rc<A>) {
/// let result = crate::check_already_logged_in(&window, adapter.as_ref()).await;
/// match result {
///     Ok(true) => println!("already logged in"),
///     Err(_) | Ok(false) => println!("not logged in"),
/// }
/// # }
/// ```
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

/// Submits username/password from the DOM to the login API and updates the page on failure.
///
/// Reads the values of inputs with IDs "username" and "password", sends them as JSON to
/// `/api/auth/login` using the provided `adapter`. If the response has status 200 the adapter
/// is asked to redirect to `/honey_note/index.html`. On non-200 responses the function reads a
/// `message` field from the response JSON (falling back to `ログインに失敗しました`), sets that
/// text into `error_div`, and makes the `error_div` visible.
///
/// # Parameters
///
/// - `error_div`: HTML element where error messages will be written and shown.
/// - `document`: DOM `Document` used to locate the username/password input elements.
/// - `adapter`: Browser adapter used to perform the HTTP request and navigation.
///
/// # Returns
///
/// `Ok(())` on success; `Err(JsValue)` if DOM elements are missing/invalid, JSON/stringification fails,
/// the fetch via `adapter` fails, or the adapter redirect fails.
///
/// # Examples
///
/// ```no_run
/// # use std::rc::Rc;
/// # use wasm_bindgen::JsValue;
/// # async fn example<A: BrowserAdapter + 'static>(adapter: Rc<A>, document: &web_sys::Document, error_div: &web_sys::HtmlElement) -> Result<(), JsValue> {
/// // `handle_login` would be called from an async context (e.g., a submit handler).
/// handle_login::<A>(&web_sys::HtmlFormElement::new().unwrap(), error_div, document, &web_sys::window().unwrap(), adapter.as_ref()).await
/// # }
/// ```
async fn handle_login<A: BrowserAdapter>(
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

    let password_element = document
        .get_element_by_id("password")
        .ok_or_else(|| JsValue::from_str("password element not found"))?
        .dyn_into::<HtmlInputElement>()
        .map_err(|_| JsValue::from_str("password is not an input element"))?;
    let password = password_element.value();

    let body_obj = web_sys::js_sys::Object::new();
    Reflect::set(
        &body_obj,
        &JsValue::from_str("username"),
        &JsValue::from_str(&username),
    )?;
    Reflect::set(
        &body_obj,
        &JsValue::from_str("password"),
        &JsValue::from_str(&password),
    )?;
    let body_str = web_sys::js_sys::JSON::stringify(&body_obj)?;

    let resp = adapter
        .fetch("/api/auth/login", "POST", Some(body_str.into()))
        .await?;

    if resp.status() == 200 {
        adapter.redirect("/honey_note/index.html")?;
    } else {
        let json = JsFuture::from(resp.json()?).await?;
        let message = Reflect::get(&json, &JsValue::from_str("message"))?
            .as_string()
            .unwrap_or_else(|| "ログインに失敗しました".to_string());
        error_div.set_text_content(Some(&message));
        let _ = error_div.style().set_property("display", "block");
    }

    Ok(())
}