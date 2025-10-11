use common_type::models::honey::front_app_model::HoneyDetail;
use web_sys::{Location, UrlSearchParams};

// main behavior of showing honey detail page
pub fn run() {
    web_sys::console::log_1(&"Hello, Honey Detail Page!! main function".into());
    let honey_id_opt: Option<HoneyDetailParametersFromUrl> = get_honey_id_from_location();
    match honey_id_opt {
        Some(honey_detail_id) => {},
        None => {} // nothing todo because invalid or new id
        
    }
}

fn get_honey_id_from_location() -> Option<HoneyDetailParametersFromUrl> {
    let location: Location = web_sys::window()?.location();
    let search: String = location.search().ok()?;
    let search_params_result: Result<UrlSearchParams, wasm_bindgen::JsValue> =
        UrlSearchParams::new_with_str(&search);

    match search_params_result {
        Ok(params) => {
            let id_opt: Option<i32> = params
                .get("id")
                .and_then(|id_str| id_str.parse::<i32>().ok());
            Some(HoneyDetailParametersFromUrl { id: id_opt })
        }
        Err(e) => {
            web_sys::console::log_1(&format!("cannot take parameter : {:?}", e).into());
            None
        }
    }
}

fn get_honey_detail_by_id(id: i32) -> Option<HoneyDetail> {
    None
}

#[derive()]
struct HoneyDetailParametersFromUrl {
    pub id: Option<i32>,
}
