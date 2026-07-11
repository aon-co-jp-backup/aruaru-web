//! SQL 実行結果 / レジストリ集計のレンダリング。

use crate::dom::{by_id, esc};

pub fn set_result_html(html: &str) {
    by_id("result").set_inner_html(html);
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
