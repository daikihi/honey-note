use web_sys::{Document, Location, Window};
use crate::edit_and_new::beekeeper_edit_mode;
use crate::edit_and_new::beekeeper_new_mode;

pub async fn run() {
    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");
    let location: Location = window.location();
    let pathname: String = location.pathname().expect("failed to get pathname");

    if pathname.contains("/beekeepers/edit.html") {
        web_sys::console::log_1(&"Beekeeper Edit Mode".into());
        beekeeper_edit_mode::setup_edit_mode(&window, &document).await;
    } else if pathname.contains("/beekeepers/new.html") {
        web_sys::console::log_1(&"Beekeeper New Mode".into());
        beekeeper_new_mode::setup_new_mode(&window, &document).await;
    }
}
