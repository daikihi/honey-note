use front::signup_page_main;
use front::commons::browser_adapter::WebBrowserAdapter;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use web_sys::{js_sys, HtmlElement, HtmlInputElement};
use std::rc::Rc;

/// JavaScript側でのスタブ実装
/// window.fetch と window.location.assign をテスト用に上書きし、
/// 呼び出し履歴の保存や、ダミーレスポンスの返却を行えるようにします。
#[wasm_bindgen(inline_js = r#"
// 各スタブの状態をリセットまたは初期化
export function setup_stubs() {
    window.last_fetch = null;
    window.last_redirect = null;
    window.next_responses = [];

    // Fetchのスタブ化：実際のネットワークリクエストを飛ばさず、
    // 呼ばれた引数を記録し、キューから次のレスポンスを取り出して返します。
    window.original_fetch = window.fetch;
    window.fetch = async (url, options) => {
        window.last_fetch = { url, options };
        const response = window.next_responses.shift() || { status: 200, body: '{}' };
        return {
            status: response.status,
            ok: response.status < 400,
            json: async () => {
                if (typeof response.body === 'string') {
                    return JSON.parse(response.body);
                }
                return response.body;
            }
        };
    };

    // Locationのスタブ化：ページ遷移が発生するとテストが中断されるため、
    // window.location.assign を上書きして、渡されたURLを記録するだけに留めます。
    try {
        // locationオブジェクト自体は書き換えられないブラウザが多いため、
        // assignメソッドのみを上書きします。
        window.location.assign = (url) => {
            window.last_redirect = url;
        };
    } catch (e) {
        console.log("Could not stub window.location.assign directly:", e);
    }
}

// スタブを元の状態に戻し、使用したメモリをクリーンアップ
export function restore_stubs() {
    window.fetch = window.original_fetch;
    delete window.last_fetch;
    delete window.last_redirect;
    delete window.next_responses;
}

// 次に fetch が呼ばれた際に返すべきステータスコードとJSONをキューに追加
export function set_next_response(status, json) {
    const body = typeof json === 'string' ? json : JSON.stringify(json);
    window.next_responses.push({ status, body });
}

// 最後に呼ばれたリダイレクト先URLを取得
export function get_last_redirect() {
    return window.last_redirect;
}

// 最後に呼ばれたFetchリクエストの内容を取得
export function get_last_fetch() {
    return window.last_fetch;
}
"#)]
extern "C" {
    fn setup_stubs();
    fn restore_stubs();
    fn set_next_response(status: u16, json: JsValue);
    fn get_last_redirect() -> JsValue;
    fn get_last_fetch() -> JsValue;
}

/// テスト用のDOMを構築
/// signup_test.html を読み込み、テスト環境の body に追加します。
fn setup_dom() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let container = document.create_element("div").unwrap();
    container.set_id("test-root");
    // テスト専用のHTMLファイルを文字列として埋め込み
    container.set_inner_html(include_str!("signup_test.html"));
    body.append_child(&container).unwrap();
}

/// テストで使用したDOMを削除
fn cleanup_dom() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    if let Some(root) = document.get_element_by_id("test-root") {
        root.remove();
    }
}

/// テストケース：パスワード不一致時にエラーが表示されるか
#[wasm_bindgen_test]
async fn test_signup_password_mismatch() {
    setup_dom();
    setup_stubs();

    let document = web_sys::window().unwrap().document().unwrap();

    // 入力フォームにテストデータをセット
    document
        .get_element_by_id("username")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .set_value("testuser");
    document
        .get_element_by_id("email")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .set_value("test@example.com");
    document
        .get_element_by_id("password")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .set_value("password123");
    document
        .get_element_by_id("confirm_password")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .set_value("mismatch");

    // アプリケーションの初期化（イベントリスナーの登録）を実行
    let adapter = Rc::new(WebBrowserAdapter);
    signup_page_main::run(adapter).await;

    // submitイベントを手動で発火
    let form = document.get_element_by_id("signup_form").unwrap();
    let event = web_sys::Event::new("submit").unwrap();
    form.dispatch_event(&event).unwrap();

    // 内部の非同期処理（spawn_local）が完了するまで待機
    wasm_bindgen_futures::JsFuture::from(js_sys::Promise::resolve(&JsValue::NULL))
        .await
        .unwrap();

    // 検証：エラーメッセージが正しく設定され、表示されているか
    let error_div = document
        .get_element_by_id("error_message")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    assert_eq!(
        error_div.text_content().unwrap(),
        "エラーが発生しました"
    );
    assert_eq!(
        error_div.style().get_property_value("display").unwrap(),
        "block"
    );

    cleanup_dom();
    restore_stubs();
}

/// テストケース：新規登録成功時にログインページへリダイレクトされるか
#[wasm_bindgen_test]
async fn test_signup_success() {
    setup_dom();
    setup_stubs();
    // 成功レスポンス（201 Created）をシミュレート
    set_next_response(201, JsValue::from_str("{}"));

    let document = web_sys::window().unwrap().document().unwrap();

    document
        .get_element_by_id("username")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .set_value("testuser");
    document
        .get_element_by_id("email")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .set_value("test@example.com");
    document
        .get_element_by_id("password")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .set_value("password123");
    document
        .get_element_by_id("confirm_password")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .set_value("password123");

    // アプリケーションの初期化（イベントリスナーの登録）を実行
    let adapter = Rc::new(WebBrowserAdapter);
    signup_page_main::run(adapter).await;

    let form = document.get_element_by_id("signup_form").unwrap();
    let event = web_sys::Event::new("submit").unwrap();
    form.dispatch_event(&event).unwrap();

    // ネットワークリクエストと非同期処理の完了を待機
    for _ in 0..5 {
        wasm_bindgen_futures::JsFuture::from(js_sys::Promise::resolve(&JsValue::NULL))
            .await
            .unwrap();
    }

    // 検証：正しいURLにリダイレクトしようとしたか
    let redirect = get_last_redirect();
    assert_eq!(redirect.as_string().unwrap(), "/honey_note/login.html");

    cleanup_dom();
    restore_stubs();
}

/// テストケース：サーバーエラー（400 Bad Request）時のエラーメッセージ表示
#[wasm_bindgen_test]
async fn test_signup_server_error_400_with_message() {
    setup_dom();
    setup_stubs();
    // サーバーエラーレスポンスをシミュレート（400 Bad Request）
    set_next_response(400, JsValue::from_str(r#"{"message": "メールアドレスは既に登録されています"}"#));

    let document = web_sys::window().unwrap().document().unwrap();

    // フォーム入力（有効なデータ）
    document
        .get_element_by_id("username")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .set_value("testuser");
    document
        .get_element_by_id("email")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .set_value("test@example.com");
    document
        .get_element_by_id("password")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .set_value("password123");
    document
        .get_element_by_id("confirm_password")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .set_value("password123");

    // アプリケーションの初期化（イベントリスナーの登録）を実行
    let adapter = Rc::new(WebBrowserAdapter);
    signup_page_main::run(adapter).await;

    // submitイベントを手動で発火
    let form = document.get_element_by_id("signup_form").unwrap();
    let event = web_sys::Event::new("submit").unwrap();
    form.dispatch_event(&event).unwrap();

    // 内部の非同期処理（spawn_local）が完了するまで待機
    for _ in 0..5 {
        wasm_bindgen_futures::JsFuture::from(js_sys::Promise::resolve(&JsValue::NULL))
            .await
            .unwrap();
    }

    // 検証：エラーメッセージが正しく設定され、表示されているか
    let error_div = document
        .get_element_by_id("error_message")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    assert_eq!(
        error_div.text_content().unwrap(),
        "メールアドレスは既に登録されています"
    );
    assert_eq!(
        error_div.style().get_property_value("display").unwrap(),
        "block"
    );

    // リダイレクトが発生していないことを確認
    let redirect = get_last_redirect();
    assert!(redirect.is_null());

    cleanup_dom();
    restore_stubs();
}

/// テストケース：サーバーエラー（500 Internal Server Error）時のエラーメッセージ表示
#[wasm_bindgen_test]
async fn test_signup_server_error_500_with_message() {
    setup_dom();
    setup_stubs();
    // サーバーエラーレスポンスをシミュレート（500 Internal Server Error）
    set_next_response(500, JsValue::from_str(r#"{"message": "サーバー内部エラーが発生しました"}"#));

    let document = web_sys::window().unwrap().document().unwrap();

    // フォーム入力（有効なデータ）
    document
        .get_element_by_id("username")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .set_value("testuser");
    document
        .get_element_by_id("email")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .set_value("test@example.com");
    document
        .get_element_by_id("password")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .set_value("password123");
    document
        .get_element_by_id("confirm_password")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .set_value("password123");

    // アプリケーションの初期化（イベントリスナーの登録）を実行
    let adapter = Rc::new(WebBrowserAdapter);
    signup_page_main::run(adapter).await;

    // submitイベントを手動で発火
    let form = document.get_element_by_id("signup_form").unwrap();
    let event = web_sys::Event::new("submit").unwrap();
    form.dispatch_event(&event).unwrap();

    // 内部の非同期処理（spawn_local）が完了するまで待機
    for _ in 0..5 {
        wasm_bindgen_futures::JsFuture::from(js_sys::Promise::resolve(&JsValue::NULL))
            .await
            .unwrap();
    }

    // 検証：エラーメッセージが正しく設定され、表示されているか
    let error_div = document
        .get_element_by_id("error_message")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    assert_eq!(
        error_div.text_content().unwrap(),
        "サーバー内部エラーが発生しました"
    );
    assert_eq!(
        error_div.style().get_property_value("display").unwrap(),
        "block"
    );

    // リダイレクトが発生していないことを確認
    let redirect = get_last_redirect();
    assert!(redirect.is_null());

    cleanup_dom();
    restore_stubs();
}

/// テストケース：サーバーエラー（400）でメッセージフィールドなしの場合のフォールバック表示
#[wasm_bindgen_test]
async fn test_signup_server_error_400_no_message() {
    setup_dom();
    setup_stubs();
    // サーバーエラーレスポンスをシミュレート（400 Bad Request、message フィールドなし）
    set_next_response(400, JsValue::from_str("{}"));

    let document = web_sys::window().unwrap().document().unwrap();

    // フォーム入力（有効なデータ）
    document
        .get_element_by_id("username")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .set_value("testuser");
    document
        .get_element_by_id("email")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .set_value("test@example.com");
    document
        .get_element_by_id("password")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .set_value("password123");
    document
        .get_element_by_id("confirm_password")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .set_value("password123");

    // アプリケーションの初期化（イベントリスナーの登録）を実行
    let adapter = Rc::new(WebBrowserAdapter);
    signup_page_main::run(adapter).await;

    // submitイベントを手動で発火
    let form = document.get_element_by_id("signup_form").unwrap();
    let event = web_sys::Event::new("submit").unwrap();
    form.dispatch_event(&event).unwrap();

    // 内部の非同期処理（spawn_local）が完了するまで待機
    for _ in 0..5 {
        wasm_bindgen_futures::JsFuture::from(js_sys::Promise::resolve(&JsValue::NULL))
            .await
            .unwrap();
    }

    // 検証：フォールバックメッセージが表示されているか
    let error_div = document
        .get_element_by_id("error_message")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    assert_eq!(
        error_div.text_content().unwrap(),
        "新規登録に失敗しました"
    );
    assert_eq!(
        error_div.style().get_property_value("display").unwrap(),
        "block"
    );

    // リダイレクトが発生していないことを確認
    let redirect = get_last_redirect();
    assert!(redirect.is_null());

    cleanup_dom();
    restore_stubs();
}

/// テストケース：既にログイン済みの場合の自動リダイレクト
#[wasm_bindgen_test]
async fn test_already_logged_in_redirect() {
    setup_dom();
    setup_stubs();
    // /api/auth/me のレスポンスをログイン済みに設定
    set_next_response(200, JsValue::from_str(r#"{"logged_in": true}"#));

    // run() を実行（ページ初期化）
    let adapter = Rc::new(WebBrowserAdapter);
    signup_page_main::run(adapter).await;

    // 非同期処理（check_already_logged_in）の完了を待機
    for _ in 0..10 {
        wasm_bindgen_futures::JsFuture::from(js_sys::Promise::resolve(&JsValue::NULL))
            .await
            .unwrap();
    }

    // 検証: リダイレクトが発生していること
    let redirect = get_last_redirect();
    assert_eq!(redirect.as_string().unwrap(), "/honey_note/index.html");

    cleanup_dom();
    restore_stubs();
}

/// テストケース：ログインしていない場合のリダイレクトなし
#[wasm_bindgen_test]
async fn test_not_logged_in_no_redirect() {
    setup_dom();
    setup_stubs();
    // /api/auth/me のレスポンスを未ログインに設定
    set_next_response(200, JsValue::from_str(r#"{"logged_in": false}"#));

    // run() を実行
    let adapter = Rc::new(WebBrowserAdapter);
    signup_page_main::run(adapter).await;

    // 非同期処理待機
    for _ in 0..10 {
        wasm_bindgen_futures::JsFuture::from(js_sys::Promise::resolve(&JsValue::NULL))
            .await
            .unwrap();
    }

    // 検証: リダイレクトが発生していないこと
    let redirect = get_last_redirect();
    assert!(redirect.is_null());

    cleanup_dom();
    restore_stubs();
}

/// テストケース：/api/auth/me が 401 を返す場合のリダイレクトなし
#[wasm_bindgen_test]
async fn test_auth_me_unauthorized_no_redirect() {
    setup_dom();
    setup_stubs();
    // /api/auth/me のレスポンスを 401 Unauthorized に設定
    set_next_response(401, JsValue::from_str(r#"{"message": "Unauthorized"}"#));

    // run() を実行
    let adapter = Rc::new(WebBrowserAdapter);
    signup_page_main::run(adapter).await;

    // 非同期処理待機
    for _ in 0..10 {
        wasm_bindgen_futures::JsFuture::from(js_sys::Promise::resolve(&JsValue::NULL))
            .await
            .unwrap();
    }

    // 検証: リダイレクトが発生していないこと
    let redirect = get_last_redirect();
    assert!(redirect.is_null());

    cleanup_dom();
    restore_stubs();
}
