//! aruaru-web: aruaru-db 用の Web UI。
//!
//! Rust を `wasm32-unknown-unknown` へコンパイルし、`wasm-bindgen` + `web-sys`
//! で DOM 操作と `fetch()` を行う。TypeScript/Node.js のビルドチェーンは
//! 使用しない(詳細は CLAUDE.md / README.md 参照)。
//!
//! ## できること
//! - `aruaru-db`(`aruaru-graphql` クレート、`/graphql` エンドポイント)に
//!   実際に `fetch()` で GraphQL リクエストを送る:
//!   - `sql(query: String!): QueryResultGql` — SQL 実行
//!   - `registrySummary: RegistrySummaryGql` — 対応DBレジストリの集計
//! - サーバーに接続できない場合は、実スキーマと同じ形のサンプルデータを
//!   その場で描画し、「オフラインサンプル」である旨を明示する。
//! - 「サイト管理」タブで、aruaru-web用・他プロジェクト用の複数の接続先
//!   (IPアドレス/ドメイン/サブドメイン)を登録し、`localStorage` に保存
//!   してワンクリックで切り替えられる(KUSANAGIのサイト一覧に相当する
//!   最小限の管理UI)。実際のドメイン取得・DNS登録はここでは行わない
//!   (`deploy/` の vhost テンプレート・`scripts/gen-vhost.sh` を参照)。

mod dom;
mod graphql;
mod history;
mod profiles;
mod render;
mod shell;

use dom::{by_id, document, log, set_status};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlButtonElement, HtmlInputElement, HtmlTextAreaElement, KeyboardEvent};

const DEFAULT_SQL: &str = "SELECT * FROM aruaru_log LIMIT 10;";

/// エントリポイント。ページ読み込み時に一度だけ呼ばれる。
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook_set();
    log("aruaru-web starting");

    let root = by_id("app-root");
    root.set_inner_html(shell::SHELL_HTML);

    let sql_input: HtmlTextAreaElement = by_id("sql-input").dyn_into()?;
    sql_input.set_value(DEFAULT_SQL);

    // 「SQL実行」ボタン
    let run_sql_btn: HtmlButtonElement = by_id("run-sql").dyn_into()?;
    let run_sql_closure = Closure::<dyn FnMut(Event)>::new(move |_evt: Event| {
        on_run_sql();
    });
    run_sql_btn.set_onclick(Some(run_sql_closure.as_ref().unchecked_ref()));
    run_sql_closure.forget();

    // 「レジストリ集計」ボタン
    let registry_btn: HtmlButtonElement = by_id("run-registry").dyn_into()?;
    let registry_closure = Closure::<dyn FnMut(Event)>::new(move |_evt: Event| {
        on_run_registry_summary();
    });
    registry_btn.set_onclick(Some(registry_closure.as_ref().unchecked_ref()));
    registry_closure.forget();

    // 「サイトを保存」ボタン
    let save_site_btn: HtmlButtonElement = by_id("save-site").dyn_into()?;
    let save_site_closure = Closure::<dyn FnMut(Event)>::new(move |_evt: Event| {
        profiles::on_save_site();
    });
    save_site_btn.set_onclick(Some(save_site_closure.as_ref().unchecked_ref()));
    save_site_closure.forget();

    // 「クリア」ボタン
    let clear_btn: HtmlButtonElement = by_id("clear-site-form").dyn_into()?;
    let clear_closure = Closure::<dyn FnMut(Event)>::new(move |_evt: Event| {
        profiles::clear_form();
    });
    clear_btn.set_onclick(Some(clear_closure.as_ref().unchecked_ref()));
    clear_closure.forget();

    // 「CSVでエクスポート」ボタン
    let export_btn: HtmlButtonElement = by_id("export-csv").dyn_into()?;
    let export_closure = Closure::<dyn FnMut(Event)>::new(move |_evt: Event| {
        render::download_csv();
    });
    export_btn.set_onclick(Some(export_closure.as_ref().unchecked_ref()));
    export_closure.forget();

    // 「サイト一覧をエクスポート(JSON)」ボタン
    let site_export_btn: HtmlButtonElement = by_id("site-export").dyn_into()?;
    let site_export_closure = Closure::<dyn FnMut(Event)>::new(move |_evt: Event| {
        profiles::export_profiles_json();
    });
    site_export_btn.set_onclick(Some(site_export_closure.as_ref().unchecked_ref()));
    site_export_closure.forget();

    // 「サイト一覧をインポート(JSON)」ボタン → 隠しファイル入力をクリック
    let site_import_trigger: HtmlButtonElement = by_id("site-import-trigger").dyn_into()?;
    let site_import_file: web_sys::HtmlInputElement = by_id("site-import-file").dyn_into()?;
    let import_file_for_trigger = site_import_file.clone();
    let site_import_trigger_closure = Closure::<dyn FnMut(Event)>::new(move |_evt: Event| {
        import_file_for_trigger.click();
    });
    site_import_trigger.set_onclick(Some(site_import_trigger_closure.as_ref().unchecked_ref()));
    site_import_trigger_closure.forget();

    let import_file_for_change = site_import_file.clone();
    let site_import_change_closure = Closure::<dyn FnMut(Event)>::new(move |_evt: Event| {
        if let Some(files) = import_file_for_change.files() {
            if let Some(file) = files.get(0) {
                profiles::import_profiles_from_file(file);
            }
        }
        import_file_for_change.set_value("");
    });
    site_import_file
        .set_onchange(Some(site_import_change_closure.as_ref().unchecked_ref()));
    site_import_change_closure.forget();

    // SQL欄での Ctrl+Enter / Cmd+Enter ショートカット
    let keydown_closure = Closure::<dyn FnMut(KeyboardEvent)>::new(move |evt: KeyboardEvent| {
        if evt.key() == "Enter" && (evt.ctrl_key() || evt.meta_key()) {
            evt.prevent_default();
            on_run_sql();
        }
    });
    sql_input
        .add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref())?;
    keydown_closure.forget();

    wire_tabs()?;

    profiles::render_site_manager();
    profiles::sync_endpoint_field();
    history::render();
    show_tab("sql");

    set_status("準備完了。SQLを実行するか、レジストリ集計を取得してください。");
    Ok(())
}

/// panic 時にブラウザの console に stacktrace 相当を出す(デバッグ用)。
fn console_error_panic_hook_set() {
    std::panic::set_hook(Box::new(|info| {
        web_sys::console::error_1(&JsValue::from_str(&info.to_string()));
    }));
}

fn wire_tabs() -> Result<(), JsValue> {
    let doc = document();
    let nodes = doc.query_selector_all(".tab-btn")?;
    for i in 0..nodes.length() {
        if let Some(node) = nodes.get(i) {
            if let Ok(btn) = node.dyn_into::<HtmlButtonElement>() {
                let tab = btn.get_attribute("data-tab").unwrap_or_default();
                let closure = Closure::<dyn FnMut(Event)>::new(move |_evt: Event| {
                    show_tab(&tab);
                });
                btn.set_onclick(Some(closure.as_ref().unchecked_ref()));
                closure.forget();
            }
        }
    }
    Ok(())
}

fn show_tab(tab: &str) {
    let doc = document();
    for (id, name) in [("tab-sql", "sql"), ("tab-registry", "registry"), ("tab-sites", "sites")] {
        if let Some(el) = doc.get_element_by_id(id) {
            let html_el: Result<web_sys::HtmlElement, _> = el.dyn_into();
            if let Ok(html_el) = html_el {
                html_el
                    .style()
                    .set_property("display", if name == tab { "block" } else { "none" })
                    .ok();
            }
        }
    }
    if let Ok(nodes) = doc.query_selector_all(".tab-btn") {
        for i in 0..nodes.length() {
            if let Some(node) = nodes.get(i) {
                if let Ok(btn) = node.dyn_into::<HtmlButtonElement>() {
                    let this_tab = btn.get_attribute("data-tab").unwrap_or_default();
                    if this_tab == tab {
                        btn.class_list().add_1("active").ok();
                    } else {
                        btn.class_list().remove_1("active").ok();
                    }
                }
            }
        }
    }
}

fn on_run_sql() {
    let endpoint = current_endpoint();
    let sql = current_sql();
    if sql.trim().is_empty() {
        set_status("SQLを入力してください。");
        return;
    }
    history::push(&sql);
    history::render();
    set_status("SQL実行中…");
    set_button_busy("run-sql", true, "実行中…");
    wasm_bindgen_futures::spawn_local(async move {
        let variables = serde_json::json!({ "sql": sql });
        match graphql::post_graphql(&endpoint, graphql::SQL_QUERY, variables).await {
            Ok(body) => match graphql::extract_data(&body, "sql") {
                Ok(data) => {
                    render::render_query_result(&data, false);
                    set_status("実行完了(aruaru-dbから取得)。");
                }
                Err(e) => {
                    render::render_query_result_offline_sample(&sql);
                    set_status(&format!(
                        "サーバー応答を解釈できませんでした({e})。オフラインサンプルを表示しています。"
                    ));
                }
            },
            Err(e) => {
                render::render_query_result_offline_sample(&sql);
                set_status(&format!(
                    "aruaru-db ({endpoint}) に接続できませんでした: {e}。オフラインサンプルを表示しています。"
                ));
            }
        }
        set_button_busy("run-sql", false, "SQLを実行");
    });
}

fn set_button_busy(id: &str, busy: bool, label: &str) {
    if let Ok(btn) = by_id(id).dyn_into::<HtmlButtonElement>() {
        btn.set_disabled(busy);
        btn.set_text_content(Some(label));
    }
}

fn on_run_registry_summary() {
    let endpoint = current_endpoint();
    set_status("レジストリ集計を取得中…");
    set_button_busy("run-registry", true, "取得中…");
    wasm_bindgen_futures::spawn_local(async move {
        match graphql::post_graphql(&endpoint, graphql::REGISTRY_SUMMARY_QUERY, serde_json::Value::Null).await {
            Ok(body) => match graphql::extract_data(&body, "registrySummary") {
                Ok(data) => {
                    render::render_registry_summary(&data, false);
                    set_status("実行完了(aruaru-dbから取得)。");
                }
                Err(e) => {
                    render::render_registry_summary_offline_sample();
                    set_status(&format!(
                        "サーバー応答を解釈できませんでした({e})。オフラインサンプルを表示しています。"
                    ));
                }
            },
            Err(e) => {
                render::render_registry_summary_offline_sample();
                set_status(&format!(
                    "aruaru-db ({endpoint}) に接続できませんでした: {e}。オフラインサンプルを表示しています。"
                ));
            }
        }
        set_button_busy("run-registry", false, "レジストリ集計を取得");
    });
}

fn current_endpoint() -> String {
    let el: HtmlInputElement = by_id("endpoint").dyn_into().expect("endpoint input");
    let v = el.value();
    if v.trim().is_empty() {
        profiles::active_endpoint()
    } else {
        v
    }
}

fn current_sql() -> String {
    let el: HtmlTextAreaElement = by_id("sql-input").dyn_into().expect("sql textarea");
    el.value()
}
