//! aruaru-web: aruaru-db 用の最小限の Web UI。
//!
//! Rust を `wasm32-unknown-unknown` へコンパイルし、`wasm-bindgen` + `web-sys`
//! で DOM 操作と `fetch()` を行う。TypeScript/Node.js のビルドチェーンは
//! 使用しない(詳細は CLAUDE.md / README.md 参照)。
//!
//! ## この最初のバージョンが実際にできること
//! - `aruaru-db`(`aruaru-graphql` クレート、`/graphql` エンドポイント)に
//!   実際に `fetch()` で GraphQL リクエストを送る:
//!   - `sql(query: String!): QueryResultGql` — SQL 実行
//!   - `registrySummary: RegistrySummaryGql` — 対応DBレジストリの集計
//! - サーバーに接続できない場合(このリポジトリ単体のビルド検証環境など、
//!   `aruaru-server` が起動していない場合)は、実スキーマと同じ形の
//!   サンプルデータをその場で描画し、「オフラインサンプル」である旨を
//!   明示する。GraphQL のレスポンス形状は `aruaru-db/crates/aruaru-graphql`
//!   の `QueryResultGql` / `RegistrySummaryGql` に合わせてある。

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    Document, Element, Event, HtmlButtonElement, HtmlInputElement, HtmlTextAreaElement, Request,
    RequestInit, RequestMode, Response,
};

const DEFAULT_ENDPOINT: &str = "http://localhost:4000/graphql";
const DEFAULT_SQL: &str = "SELECT * FROM aruaru_log LIMIT 10;";

const SQL_QUERY: &str = "query($sql: String!) { sql(query: $sql) { columns rows commandTag } }";
const REGISTRY_SUMMARY_QUERY: &str =
    "{ registrySummary { total connectable ga beta pgCompatible planned } }";

/// `console.log` へのショートカット。
fn log(msg: &str) {
    web_sys::console::log_1(&JsValue::from_str(msg));
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> Document {
    window().document().expect("window should have a document")
}

fn by_id(id: &str) -> Element {
    document()
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("missing #{id} element"))
}

fn set_status(msg: &str) {
    by_id("status").set_text_content(Some(msg));
}

fn set_result_html(html: &str) {
    by_id("result").set_inner_html(html);
}

/// 最低限の HTML エスケープ(ユーザー入力/サーバー応答をそのまま
/// `inner_html` に差し込むための保護)。
fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// エントリポイント。ページ読み込み時に一度だけ呼ばれる。
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook_set();
    log("aruaru-web starting");

    let document = document();
    let root = by_id("app-root");
    root.set_inner_html(SHELL_HTML);

    // エンドポイント入力にデフォルト値をセット。
    let endpoint_input: HtmlInputElement = by_id("endpoint").dyn_into()?;
    endpoint_input.set_value(DEFAULT_ENDPOINT);

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

    let _ = document; // keep binding (readability); avoids unused warning if features change
    set_status("準備完了。SQLを実行するか、レジストリ集計を取得してください。");
    Ok(())
}

/// panic 時にブラウザの console に stacktrace 相当を出す(デバッグ用)。
/// `console_error_panic_hook` crate を追加せず、標準の panic hook を
/// `console.error` へ橋渡しする最小実装。
fn console_error_panic_hook_set() {
    std::panic::set_hook(Box::new(|info| {
        web_sys::console::error_1(&JsValue::from_str(&info.to_string()));
    }));
}

fn on_run_sql() {
    let endpoint = current_endpoint();
    let sql = current_sql();
    set_status("SQL実行中…");
    wasm_bindgen_futures::spawn_local(async move {
        let variables = serde_json::json!({ "sql": sql });
        match post_graphql(&endpoint, SQL_QUERY, variables).await {
            Ok(body) => match extract_data(&body, "sql") {
                Ok(data) => {
                    render_query_result(&data, false);
                    set_status("実行完了(aruaru-dbから取得)。");
                }
                Err(e) => {
                    render_query_result_offline_sample(&sql);
                    set_status(&format!("サーバー応答を解釈できませんでした({e})。オフラインサンプルを表示しています。"));
                }
            },
            Err(e) => {
                render_query_result_offline_sample(&sql);
                set_status(&format!(
                    "aruaru-db ({endpoint}) に接続できませんでした: {e}。オフラインサンプルを表示しています。"
                ));
            }
        }
    });
}

fn on_run_registry_summary() {
    let endpoint = current_endpoint();
    set_status("レジストリ集計を取得中…");
    wasm_bindgen_futures::spawn_local(async move {
        match post_graphql(&endpoint, REGISTRY_SUMMARY_QUERY, serde_json::Value::Null).await {
            Ok(body) => match extract_data(&body, "registrySummary") {
                Ok(data) => {
                    render_registry_summary(&data, false);
                    set_status("実行完了(aruaru-dbから取得)。");
                }
                Err(e) => {
                    render_registry_summary_offline_sample();
                    set_status(&format!(
                        "サーバー応答を解釈できませんでした({e})。オフラインサンプルを表示しています。"
                    ));
                }
            },
            Err(e) => {
                render_registry_summary_offline_sample();
                set_status(&format!(
                    "aruaru-db ({endpoint}) に接続できませんでした: {e}。オフラインサンプルを表示しています。"
                ));
            }
        }
    });
}

fn current_endpoint() -> String {
    let el: HtmlInputElement = by_id("endpoint").dyn_into().expect("endpoint input");
    let v = el.value();
    if v.trim().is_empty() {
        DEFAULT_ENDPOINT.to_string()
    } else {
        v
    }
}

fn current_sql() -> String {
    let el: HtmlTextAreaElement = by_id("sql-input").dyn_into().expect("sql textarea");
    el.value()
}

/// `data.<field>` を取り出す。GraphQL の `errors` があれば読みやすい形にする。
fn extract_data(body: &serde_json::Value, field: &str) -> Result<serde_json::Value, String> {
    if let Some(errors) = body.get("errors").and_then(|e| e.as_array()) {
        if !errors.is_empty() {
            let msgs: Vec<String> = errors
                .iter()
                .filter_map(|e| e.get("message").and_then(|m| m.as_str()))
                .map(|s| s.to_string())
                .collect();
            return Err(format!("GraphQL errors: {}", msgs.join("; ")));
        }
    }
    body.get("data")
        .and_then(|d| d.get(field))
        .cloned()
        .ok_or_else(|| "レスポンスに data.".to_string() + field + " が見つかりません".into())
}

/// aruaru-graphql の GraphQL エンドポイントへ POST する。
async fn post_graphql(
    endpoint: &str,
    query: &str,
    variables: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let payload = serde_json::json!({ "query": query, "variables": variables }).to_string();

    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    opts.set_body(&JsValue::from_str(&payload));

    let request = Request::new_with_str_and_init(endpoint, &opts)
        .map_err(|e| format!("request build failed: {e:?}"))?;
    request
        .headers()
        .set("Content-Type", "application/json")
        .map_err(|e| format!("header set failed: {e:?}"))?;

    let resp_value = JsFuture::from(window().fetch_with_request(&request))
        .await
        .map_err(|e| format!("fetch failed (aruaru-server may not be running): {e:?}"))?;
    let resp: Response = resp_value
        .dyn_into()
        .map_err(|_| "response cast failed".to_string())?;

    if !resp.ok() {
        return Err(format!("HTTP {}", resp.status()));
    }

    let text_js = JsFuture::from(
        resp.text()
            .map_err(|e| format!("response.text() failed: {e:?}"))?,
    )
    .await
    .map_err(|e| format!("reading body failed: {e:?}"))?;
    let text = text_js.as_string().unwrap_or_default();

    serde_json::from_str(&text).map_err(|e| format!("invalid JSON from server: {e}"))
}

// ── レンダリング ──────────────────────────────────────────────

/// `QueryResultGql { columns, rows, commandTag }` をテーブルとして描画する。
fn render_query_result(data: &serde_json::Value, offline: bool) {
    let columns: Vec<String> = data
        .get("columns")
        .and_then(|c| c.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();
    let rows: Vec<Vec<String>> = data
        .get("rows")
        .and_then(|r| r.as_array())
        .map(|rows| {
            rows.iter()
                .filter_map(|row| row.as_array())
                .map(|row| {
                    row.iter()
                        .map(|v| v.as_str().unwrap_or_default().to_string())
                        .collect()
                })
                .collect()
        })
        .unwrap_or_default();
    let command_tag = data.get("commandTag").and_then(|t| t.as_str());

    let mut html = String::new();
    if offline {
        html.push_str(offline_banner());
    }
    if let Some(tag) = command_tag {
        html.push_str(&format!("<p class=\"tag\">command: {}</p>", esc(tag)));
    }
    if columns.is_empty() && rows.is_empty() {
        html.push_str("<p class=\"muted\">結果はありません(0行)。</p>");
    } else {
        html.push_str("<table><thead><tr>");
        for c in &columns {
            html.push_str(&format!("<th>{}</th>", esc(c)));
        }
        html.push_str("</tr></thead><tbody>");
        for row in &rows {
            html.push_str("<tr>");
            for cell in row {
                html.push_str(&format!("<td>{}</td>", esc(cell)));
            }
            html.push_str("</tr>");
        }
        html.push_str("</tbody></table>");
    }
    set_result_html(&html);
}

/// `RegistrySummaryGql` を簡易カード表示する。
fn render_registry_summary(data: &serde_json::Value, offline: bool) {
    let field = |name: &str| data.get(name).and_then(|v| v.as_i64()).unwrap_or(0);
    let mut html = String::new();
    if offline {
        html.push_str(offline_banner());
    }
    html.push_str("<dl class=\"summary\">");
    for (label, key) in [
        ("登録DB総数", "total"),
        ("接続確認可能", "connectable"),
        ("GA", "ga"),
        ("Beta", "beta"),
        ("PostgreSQL互換", "pgCompatible"),
        ("計画中", "planned"),
    ] {
        html.push_str(&format!(
            "<div class=\"stat\"><dt>{}</dt><dd>{}</dd></div>",
            esc(label),
            field(key)
        ));
    }
    html.push_str("</dl>");
    set_result_html(&html);
}

fn offline_banner() -> &'static str {
    "<p class=\"offline\">⚠ aruaru-db に接続できなかったため、実スキーマと同じ形の\
     オフラインサンプルデータを表示しています。<code>cargo run -p aruaru-server</code>\
     を起動し、上のエンドポイント欄を合わせると実データが表示されます。</p>"
}

fn render_query_result_offline_sample(sql: &str) {
    let sample = serde_json::json!({
        "columns": ["id", "short_id", "message"],
        "rows": [
            ["1", "a1b2c3d", format!("sample row for: {sql}")],
            ["2", "e4f5a6b", "aruaru-server is not reachable from this build"],
        ],
        "commandTag": serde_json::Value::Null,
    });
    render_query_result(&sample, true);
}

fn render_registry_summary_offline_sample() {
    let sample = serde_json::json!({
        "total": 159,
        "connectable": 42,
        "ga": 12,
        "beta": 9,
        "pgCompatible": 21,
        "planned": 96,
    });
    render_registry_summary(&sample, true);
}

const SHELL_HTML: &str = r#"
<header>
  <h1>aruaru-web</h1>
  <p class="muted">aruaru-db 用の最小限の管理/クエリUI(Rust &rarr; WebAssembly、フレームワーク不使用)。</p>
</header>
<section>
  <label for="endpoint">GraphQL エンドポイント</label>
  <input id="endpoint" type="text" />
</section>
<section>
  <label for="sql-input">SQL</label>
  <textarea id="sql-input" rows="4"></textarea>
  <div class="buttons">
    <button id="run-sql">SQLを実行</button>
    <button id="run-registry">レジストリ集計を取得</button>
  </div>
</section>
<p id="status" class="muted"></p>
<section id="result"></section>
"#;
