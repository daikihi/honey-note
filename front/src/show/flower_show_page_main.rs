use common_type::models::flowers::Flower;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Document, Response, Window};

use crate::commons::ajax::{check_authentication, get_list_data};

pub async fn run() {
    if check_authentication().await.is_err() {
        return;
    }

    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };

    let id = match get_flower_id(&window) {
        Ok(id) => id,
        Err(message) => {
            show_error(&document, message);
            return;
        }
    };

    let api_path = format!("/honey-note/api/flower/{id}");
    let response = match get_list_data(&api_path).await {
        Ok(value) => match value.dyn_into::<Response>() {
            Ok(response) => response,
            Err(_) => {
                show_error(&document, "蜜源の詳細を取得できませんでした。");
                return;
            }
        },
        Err(_) => {
            show_error(&document, "蜜源の詳細を取得できませんでした。");
            return;
        }
    };

    if response.status() == 404 {
        show_error(
            &document,
            "蜜源が見つからないか、閲覧する権限がありません。",
        );
        return;
    }
    if !response.ok() {
        show_error(&document, "蜜源の詳細を取得できませんでした。");
        return;
    }

    let json = match response.json() {
        Ok(json) => match JsFuture::from(json).await {
            Ok(value) => value,
            Err(_) => {
                show_error(&document, "蜜源の詳細を読み込めませんでした。");
                return;
            }
        },
        Err(_) => {
            show_error(&document, "蜜源の詳細を読み込めませんでした。");
            return;
        }
    };

    match serde_wasm_bindgen::from_value::<Flower>(json) {
        Ok(flower) => render_detail(&document, id, &flower),
        Err(_) => show_error(&document, "蜜源の詳細データを読み込めませんでした。"),
    }
}

fn get_flower_id(window: &Window) -> Result<i32, &'static str> {
    let href = window
        .location()
        .href()
        .map_err(|_| "URLを読み取れませんでした。")?;
    let url = web_sys::Url::new(&href).map_err(|_| "URLを読み取れませんでした。")?;
    let id = url
        .search_params()
        .get("id")
        .ok_or("蜜源IDが指定されていません。")?;
    let id = id
        .parse::<i32>()
        .map_err(|_| "蜜源IDが正しくありません。")?;
    if id <= 0 {
        return Err("蜜源IDが正しくありません。");
    }
    Ok(id)
}

fn set_text(document: &Document, element_id: &str, text: &str) {
    if let Some(element) = document.get_element_by_id(element_id) {
        element.set_text_content(Some(text));
    }
}

fn render_detail(document: &Document, id: i32, flower: &Flower) {
    let title = format!("{}の詳細", flower.name_jp);
    document.set_title(&format!("HoneyNote | {title}"));
    set_text(document, "page_title", &title);
    set_text(document, "field_name_jp", &flower.name_jp);
    set_optional_text(document, "field_name_en", flower.name_en.as_deref());
    set_optional_text(
        document,
        "field_scientific_name",
        flower.scientific_name.as_deref(),
    );
    set_optional_text(document, "field_short_note", flower.short_note.as_deref());
    set_optional_text(document, "field_flower_type", flower.flower_type.as_deref());
    set_optional_text(document, "field_note", flower.note.as_deref());
    set_image(document, flower.image_path.as_deref(), &flower.name_jp);

    if let Some(edit_link) = document.get_element_by_id("btn_edit_link") {
        let href = format!("/honey_note/flowers/edit.html?id={id}");
        let _ = edit_link.set_attribute("href", &href);
    }

    if let Some(details) = document.get_element_by_id("flower_detail") {
        let _ = details.remove_attribute("hidden");
    }
}

fn set_optional_text(document: &Document, element_id: &str, value: Option<&str>) {
    set_text(
        document,
        element_id,
        value.filter(|text| !text.trim().is_empty()).unwrap_or("-"),
    );
}

fn set_image(document: &Document, image_path: Option<&str>, alt: &str) {
    let Some(image) = document
        .get_element_by_id("field_image")
        .and_then(|element| element.dyn_into::<web_sys::HtmlElement>().ok())
    else {
        return;
    };

    if let Some(path) = image_path.filter(|path| !path.trim().is_empty()) {
        let _ = image.set_attribute("src", path);
        let _ = image.set_attribute("alt", alt);
        let _ = image.remove_attribute("hidden");
    } else {
        let _ = image.set_attribute("hidden", "");
        let _ = image.remove_attribute("src");
    }
}

fn show_error(document: &Document, message: &str) {
    if let Some(details) = document.get_element_by_id("flower_detail") {
        let _ = details.set_attribute("hidden", "");
    }
    set_text(document, "error_message", message);
    if let Some(error) = document.get_element_by_id("error_panel") {
        let _ = error.remove_attribute("hidden");
    }
}
