use web_sys::{Document, Location, Window};
use crate::edit_and_new::edit_mode;
use crate::edit_and_new::new_mode;

pub fn run() {
    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");
    let location: Location = window.location();
    let pathname: String = location.pathname().expect("failed to get pathname");

    if pathname.contains("/edit.html") {
        web_sys::console::log_1(&"Honey Edit Mode".into());
        edit_mode::setup_edit_mode(&window, &document);
    } else if pathname.contains("/new.html") {
        web_sys::console::log_1(&"Honey New Mode".into());
        new_mode::setup_new_mode(&window, &document);
    }
}