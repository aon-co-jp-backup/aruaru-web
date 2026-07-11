//! DOM 操作の共通ヘルパー。

use wasm_bindgen::JsValue;
use web_sys::{Document, Element};

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn document() -> Document {
    window().document().expect("window should have a document")
}

pub fn by_id(id: &str) -> Element {
    document()
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("missing #{id} element"))
}

pub fn try_by_id(id: &str) -> Option<Element> {
    document().get_element_by_id(id)
}

pub fn log(msg: &str) {
    web_sys::console::log_1(&JsValue::from_str(msg));
}

pub fn set_status(msg: &str) {
    by_id("status").set_text_content(Some(msg));
}

/// 最低限の HTML エスケープ(ユーザー入力/サーバー応答をそのまま
/// `inner_html` に差し込むための保護)。
pub fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
