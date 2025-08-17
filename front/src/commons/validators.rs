use crate::commons;

pub fn is_valid_path(required_path: &str) -> bool {
    let path: String = commons::documents::get_document_path();
    web_sys::console::log_1(&format!("Document path: {}", path).into());
    path == required_path
}
