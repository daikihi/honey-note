use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::HtmlInputElement;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen(inline_js = r#"
let originalFetch;
export function setup_user_fetch() {
    originalFetch = window.fetch;
    window.fetch = async (request) => {
        const path = typeof request === 'string' ? request : request.url;
        if (path.endsWith('/api/auth/me')) {
            return new Response(JSON.stringify({logged_in: true, username: 'alice', display_name: 'Alice'}), {status: 200, headers: {'Content-Type': 'application/json'}});
        }
        return new Response(JSON.stringify({message: '保存できませんでした'}), {status: 500, headers: {'Content-Type': 'application/json'}});
    };
}
export function restore_user_fetch() { window.fetch = originalFetch; }
export async function user_tick() { await new Promise(resolve => setTimeout(resolve, 0)); }
"#)]
extern "C" {
    fn setup_user_fetch();
    fn restore_user_fetch();
    async fn user_tick();
}

fn setup_dom(edit: bool) {
    let document = web_sys::window().unwrap().document().unwrap();
    let root = document.create_element("div").unwrap();
    root.set_id("user_test_root");
    root.set_inner_html(if edit {
        r#"<header><nav></nav></header><p id="error_message"></p><form id="display_name_form"><input id="display_name"><button id="btn_save" type="submit">保存</button></form><a id="cancel" href="/honey_note/users/show.html">キャンセル</a>"#
    } else {
        r#"<header><nav></nav></header><p id="error_message"></p><div id="username"></div><div id="display_name"></div>"#
    });
    document.body().unwrap().append_child(&root).unwrap();
    setup_user_fetch();
}

fn cleanup() {
    restore_user_fetch();
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("user_test_root")
        .unwrap()
        .remove();
}

#[wasm_bindgen_test]
async fn shows_current_username_and_display_name() {
    setup_dom(false);
    front::user_show_main().await;
    let document = web_sys::window().unwrap().document().unwrap();
    assert_eq!(
        document
            .get_element_by_id("username")
            .unwrap()
            .text_content()
            .as_deref(),
        Some("alice")
    );
    assert_eq!(
        document
            .get_element_by_id("display_name")
            .unwrap()
            .text_content()
            .as_deref(),
        Some("Alice")
    );
    assert_eq!(
        document
            .get_element_by_id("header_user_info")
            .unwrap()
            .query_selector("a")
            .unwrap()
            .unwrap()
            .text_content()
            .as_deref(),
        Some("ユーザー: Alice")
    );
    cleanup();
}

#[wasm_bindgen_test]
async fn prefills_name_and_keeps_input_after_failed_save() {
    setup_dom(true);
    front::user_edit_main().await;
    let document = web_sys::window().unwrap().document().unwrap();
    let input = document
        .get_element_by_id("display_name")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();
    assert_eq!(input.value(), "Alice");
    input.set_value("New Alice");
    let event = web_sys::Event::new("submit").unwrap();
    document
        .get_element_by_id("display_name_form")
        .unwrap()
        .dispatch_event(&event)
        .unwrap();
    user_tick().await;
    assert_eq!(input.value(), "New Alice");
    assert_eq!(
        document
            .get_element_by_id("error_message")
            .unwrap()
            .text_content()
            .as_deref(),
        Some("保存できませんでした")
    );
    assert!(document
        .get_element_by_id("cancel")
        .unwrap()
        .get_attribute("href")
        .unwrap()
        .ends_with("/users/show.html"));
    cleanup();
}
