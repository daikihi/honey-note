use web_sys::{Document, Location, Window};
use crate::edit_and_new::flower_edit_mode;
use crate::edit_and_new::flower_new_mode;

pub async fn run() {
    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");
    let location: Location = window.location();
    let pathname: String = location.pathname().expect("failed to get pathname");

    if pathname.contains("/flowers/edit.html") {
        web_sys::console::log_1(&"Flower Edit Mode".into());
        flower_edit_mode::setup_edit_mode(&window, &document).await;
    } else if pathname.contains("/flowers/new.html") {
        web_sys::console::log_1(&"Flower New Mode".into());
        flower_new_mode::setup_new_mode(&window, &document).await;
    }
}
