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

fn str_field(v: &serde_json::Value, key: &str) -> String {
    v.get(key).and_then(|x| x.as_str()).unwrap_or_default().to_string()
}

/// `currentBranch` + `[BranchGql]` をテーブル表示する。
pub fn render_branches(current_branch: &str, branches: &serde_json::Value, offline: bool) {
    let rows: Vec<Vec<String>> = branches
        .as_array()
        .map(|arr| {
            arr.iter()
                .map(|b| {
                    let is_current = b.get("isCurrent").and_then(|v| v.as_bool()).unwrap_or(false);
                    vec![
                        str_field(b, "name"),
                        str_field(b, "headCommitId"),
                        if is_current { "✓".to_string() } else { String::new() },
                    ]
                })
                .collect()
        })
        .unwrap_or_default();

    let mut preamble = String::new();
    if offline {
        preamble.push_str(offline_banner());
    }
    preamble.push_str(&format!(
        "<p class=\"tag\">現在のブランチ: {}</p>",
        esc(current_branch)
    ));
    let headers = ["名前".to_string(), "HEADコミット".to_string(), "現在".to_string()];
    render_table(&headers, rows, &preamble);
}

pub fn render_branches_offline_sample() {
    let sample = serde_json::json!([
        { "name": "main", "headCommitId": "a1b2c3d4e5f6", "isCurrent": true },
        { "name": "feature/offline-sample", "headCommitId": "9f8e7d6c5b4a", "isCurrent": false },
    ]);
    render_branches("main", &sample, true);
}

/// `[CommitGql]` (コミットログ) をテーブル表示する。
pub fn render_log(commits: &serde_json::Value, offline: bool) {
    let rows: Vec<Vec<String>> = commits
        .as_array()
        .map(|arr| {
            arr.iter()
                .map(|c| {
                    vec![
                        str_field(c, "shortId"),
                        str_field(c, "author"),
                        str_field(c, "message"),
                        str_field(c, "timestamp"),
                    ]
                })
                .collect()
        })
        .unwrap_or_default();

    let preamble = if offline { offline_banner().to_string() } else { String::new() };
    let headers = ["コミット".to_string(), "作成者".to_string(), "メッセージ".to_string(), "日時".to_string()];
    render_table(&headers, rows, &preamble);
}

pub fn render_log_offline_sample() {
    let sample = serde_json::json!([
        { "shortId": "a1b2c3d", "author": "offline", "message": "sample commit (aruaru-server unreachable)", "timestamp": "2026-07-11T00:00:00Z" },
    ]);
    render_log(&sample, true);
}

/// `DiffGql` を表示する。
pub fn render_diff(data: &serde_json::Value, offline: bool) {
    let field = |key: &str| data.get(key).and_then(|v| v.as_i64()).unwrap_or(0);
    let mut html = String::new();
    if offline {
        html.push_str(offline_banner());
    }
    html.push_str(&format!(
        "<p class=\"tag\">{} → {}</p>",
        esc(&str_field(data, "fromCommit")),
        esc(&str_field(data, "toCommit"))
    ));
    html.push_str("<dl class=\"summary\">");
    for (label, key) in [("追加", "added"), ("削除", "removed"), ("変更", "modified")] {
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

pub fn render_diff_offline_sample(from: &str, to: &str) {
    let sample = serde_json::json!({
        "fromCommit": from,
        "toCommit": to,
        "added": 3,
        "removed": 1,
        "modified": 2,
    });
    render_diff(&sample, true);
}

/// `[DbEntryGql]` (登録DB一覧) をテーブル表示する。
pub fn render_registry_list(entries: &serde_json::Value, offline: bool) {
    let rows: Vec<Vec<String>> = entries
        .as_array()
        .map(|arr| {
            arr.iter()
                .map(|e| {
                    let rank = e
                        .get("rank")
                        .and_then(|v| v.as_i64())
                        .map(|v| v.to_string())
                        .unwrap_or_default();
                    let score = e
                        .get("score")
                        .and_then(|v| v.as_f64())
                        .map(|v| format!("{v:.1}"))
                        .unwrap_or_default();
                    vec![
                        str_field(e, "name"),
                        str_field(e, "category"),
                        str_field(e, "wire"),
                        str_field(e, "status"),
                        rank,
                        score,
                        str_field(e, "updatedAt"),
                    ]
                })
                .collect()
        })
        .unwrap_or_default();

    let preamble = if offline { offline_banner().to_string() } else { String::new() };
    let headers = [
        "名前".to_string(),
        "カテゴリ".to_string(),
        "ワイヤー互換".to_string(),
        "状態".to_string(),
        "順位".to_string(),
        "スコア".to_string(),
        "更新日時".to_string(),
    ];
    render_table(&headers, rows, &preamble);
}

pub fn render_registry_list_offline_sample() {
    let sample = serde_json::json!([
        { "name": "PostgreSQL", "category": "Rdbms", "wire": "Postgres", "status": "GA", "rank": 1, "score": 98.5, "updatedAt": "2026-07-01T00:00:00Z" },
        { "name": "aruaru-db (offline sample)", "category": "Distributed", "wire": "Postgres", "status": "GA", "rank": 2, "score": 91.2, "updatedAt": "2026-07-11T00:00:00Z" },
    ]);
    render_registry_list(&sample, true);
}
