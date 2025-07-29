pub fn get_document_path() -> String {
    let window = web_sys::window().expect("No global `window` exists");
    let location = window.location();
    let path = location.pathname().expect("Failed to get pathname");
    format!("{}", path)
}
