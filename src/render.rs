//! SQL 実行結果 / レジストリ集計のレンダリング。

use crate::dom::{by_id, esc, try_by_id};
use std::cell::RefCell;
use wasm_bindgen::JsCast;

type QueryResult = (Vec<String>, Vec<Vec<String>>);

thread_local! {
    static LAST_RESULT: RefCell<Option<QueryResult>> = const { RefCell::new(None) };
}

pub fn set_result_html(html: &str) {
    by_id("result").set_inner_html(html);
}

fn set_export_enabled(enabled: bool) {
    if let Some(el) = try_by_id("export-csv") {
        if let Ok(btn) = el.dyn_into::<web_sys::HtmlButtonElement>() {
            btn.set_disabled(!enabled);
        }
    }
}

/// 直近のSQL実行結果をCSV文字列として取り出す(エクスポートボタン用)。
pub fn last_result_as_csv() -> Option<String> {
    LAST_RESULT.with(|cell| {
        cell.borrow().as_ref().map(|(columns, rows)| {
            let mut csv = String::new();
            csv.push_str(&csv_row(columns));
            csv.push('\n');
            for row in rows {
                csv.push_str(&csv_row(row));
                csv.push('\n');
            }
            csv
        })
    })
}

fn csv_row(fields: &[String]) -> String {
    fields
        .iter()
        .map(|f| format!("\"{}\"", f.replace('"', "\"\"")))
        .collect::<Vec<_>>()
        .join(",")
}

/// 直近のSQL実行結果をCSVファイルとしてダウンロードする。
pub fn download_csv() {
    let Some(csv) = last_result_as_csv() else {
        return;
    };
    if crate::dom::trigger_download("aruaru-web-result.csv", &csv, "text/csv;charset=utf-8;")
        .is_none()
    {
        crate::dom::set_status("CSVのダウンロードに失敗しました。");
    }
}

/// ヘッダー行付きのテーブルを常に描画し(0行でもヘッダーは表示)、
/// CSVエクスポート用に結果を保持する。`preamble_html` は表の直前に挿入する
/// 追加HTML(オフラインバナー・注記等)。
fn render_table(headers: &[String], rows: Vec<Vec<String>>, preamble_html: &str) {
    let mut html = String::from(preamble_html);
    html.push_str(&format!(
        "<p class=\"tag\">{} 行 &times; {} 列</p>",
        rows.len(),
        headers.len()
    ));
    html.push_str("<div class=\"table-scroll\"><table><thead><tr>");
    for h in headers {
        html.push_str(&format!("<th>{}</th>", esc(h)));
    }
    html.push_str("</tr></thead><tbody>");
    for row in &rows {
        html.push_str("<tr>");
        for cell in row {
            html.push_str(&format!("<td>{}</td>", esc(cell)));
        }
        html.push_str("</tr>");
    }
    html.push_str("</tbody></table></div>");
    set_result_html(&html);

    let has_rows = !rows.is_empty();
    LAST_RESULT.with(|cell| *cell.borrow_mut() = Some((headers.to_vec(), rows)));
    set_export_enabled(has_rows);
}

/// `QueryResultGql { columns, rows, commandTag }` をテーブルとして描画する。
pub fn render_query_result(data: &serde_json::Value, offline: bool) {
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

    let mut preamble = String::new();
    if offline {
        preamble.push_str(offline_banner());
    }
    if let Some(tag) = command_tag {
        preamble.push_str(&format!("<p class=\"tag\">command: {}</p>", esc(tag)));
    }
    let has_content = !columns.is_empty() || !rows.is_empty();
    if !has_content {
        preamble.push_str("<p class=\"muted\">結果はありません(0行)。</p>");
        set_result_html(&preamble);
        LAST_RESULT.with(|cell| *cell.borrow_mut() = None);
        set_export_enabled(false);
        return;
    }
    render_table(&columns, rows, &preamble);
}

/// `RegistrySummaryGql` を簡易カード表示する。
pub fn render_registry_summary(data: &serde_json::Value, offline: bool) {
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

    LAST_RESULT.with(|cell| *cell.borrow_mut() = None);
    set_export_enabled(false);
}

fn offline_banner() -> &'static str {
    "<p class=\"offline\">⚠ aruaru-db に接続できなかったため、実スキーマと同じ形の\
     オフラインサンプルデータを表示しています。<code>cargo run -p aruaru-server</code>\
     を起動し、「サイト管理」タブで接続先を合わせると実データが表示されます。</p>"
}

pub fn render_query_result_offline_sample(sql: &str) {
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

pub fn render_registry_summary_offline_sample() {
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
