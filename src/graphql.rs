//! `aruaru-graphql` (`/graphql`) への fetch 呼び出し。

use crate::dom::window;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

pub const SQL_QUERY: &str = "query($sql: String!) { sql(query: $sql) { columns rows commandTag } }";
pub const REGISTRY_SUMMARY_QUERY: &str =
    "{ registrySummary { total connectable ga beta pgCompatible planned } }";

/// `data.<field>` を取り出す。GraphQL の `errors` があれば読みやすい形にする。
pub fn extract_data(body: &serde_json::Value, field: &str) -> Result<serde_json::Value, String> {
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
        .ok_or_else(|| "レスポンスに data.".to_string() + field + " が見つかりません")
}

/// aruaru-graphql の GraphQL エンドポイントへ POST する。
pub async fn post_graphql(
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
