use crate::edit_and_new::edit_mode;

pub fn run() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let location = window.location();
    let pathname = location.pathname().expect("failed to get pathname");

    if pathname.contains("/edit.html") {
        web_sys::console::log_1(&"Honey Edit Mode".into());
        edit_mode::setup_edit_mode(&window, &document);
    } else if pathname.contains("/new.html") {
        web_sys::console::log_1(&"Honey New Mode".into());
        // New mode is out of scope for now
    }
}