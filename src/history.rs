//! SQL実行履歴(直近10件)。`localStorage` に保存し、クリックで再利用できる。

use crate::dom::{esc, try_by_id};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlButtonElement, HtmlTextAreaElement};

const STORAGE_KEY: &str = "aruaru_web_sql_history_v1";
const MAX_ENTRIES: usize = 10;

fn local_storage() -> Option<web_sys::Storage> {
    crate::dom::window().local_storage().ok().flatten()
}

pub fn load_history() -> Vec<String> {
    local_storage()
        .and_then(|s| s.get_item(STORAGE_KEY).ok().flatten())
        .and_then(|raw| serde_json::from_str::<Vec<String>>(&raw).ok())
        .unwrap_or_default()
}

fn save_history(history: &[String]) {
    if let Some(storage) = local_storage() {
        if let Ok(raw) = serde_json::to_string(history) {
            let _ = storage.set_item(STORAGE_KEY, &raw);
        }
    }
}

/// 実行したSQLを履歴の先頭に追加する(直前と同じ内容は積まない)。
pub fn push(sql: &str) {
    let sql = sql.trim();
    if sql.is_empty() {
        return;
    }
    let mut history = load_history();
    if history.first().map(String::as_str) == Some(sql) {
        return;
    }
    history.retain(|s| s != sql);
    history.insert(0, sql.to_string());
    history.truncate(MAX_ENTRIES);
    save_history(&history);
}

pub fn render() {
    let history = load_history();
    let Some(container) = try_by_id("sql-history") else {
        return;
    };

    if history.is_empty() {
        container.set_inner_html("<p class=\"muted\">履歴はまだありません。</p>");
        return;
    }

    let mut html = String::from("<ul class=\"history-list\">");
    for sql in &history {
        html.push_str(&format!(
            "<li><button class=\"history-item\" type=\"button\" title=\"{}\">{}</button></li>",
            esc(sql),
            esc(sql)
        ));
    }
    html.push_str("</ul>");
    container.set_inner_html(&html);

    if let Ok(nodes) = crate::dom::document().query_selector_all(".history-item") {
        for i in 0..nodes.length() {
            if let Some(node) = nodes.get(i) {
                if let Ok(btn) = node.dyn_into::<HtmlButtonElement>() {
                    let sql = btn.text_content().unwrap_or_default();
                    let closure = Closure::<dyn FnMut(Event)>::new(move |_evt: Event| {
                        if let Some(el) = try_by_id("sql-input") {
                            if let Ok(textarea) = el.dyn_into::<HtmlTextAreaElement>() {
                                textarea.set_value(&sql);
                                textarea.focus().ok();
                            }
                        }
                    });
                    btn.set_onclick(Some(closure.as_ref().unchecked_ref()));
                    closure.forget();
                }
            }
        }
    }
}
