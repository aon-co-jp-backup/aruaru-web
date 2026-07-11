//! 接続プロファイル(サイト)管理。
//!
//! KUSANAGI の「サイト追加/一覧」ダッシュボードのように、aruaru-web 自身の
//! 接続先(aruaru-db の GraphQL エンドポイント)や、他プロジェクト用の
//! エンドポイントを複数登録し、ブラウザの `localStorage` に保存して
//! GUI から切り替えられるようにする。
//!
//! 実際のドメイン取得・DNS登録(レジストラ操作)はここでは行わない
//! (`deploy/` 以下の vhost テンプレート・`scripts/gen-vhost.sh` を参照)。
//! ここで管理するのはあくまで「ブラウザがどの接続先を叩くか」という設定。

use crate::dom::{by_id, document, esc, try_by_id};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement};

const STORAGE_KEY: &str = "aruaru_web_site_profiles_v1";
const ACTIVE_KEY: &str = "aruaru_web_active_site_id_v1";

#[derive(Serialize, Deserialize, Clone)]
pub struct SiteProfile {
    pub id: String,
    pub name: String,
    /// "aruaru-web" | "other"
    pub purpose: String,
    /// "http" | "https"
    pub protocol: String,
    /// IPアドレス、ドメイン、またはサブドメイン。
    pub host: String,
    pub port: u16,
    pub path: String,
    /// 自由記述のバックエンドスタック(例: "Rust + Poem", "PHP + Laravel",
    /// "Python + FastAPI")。実際の scaffolding は行わず、表示用ラベルのみ。
    pub backend_stack: String,
}

impl SiteProfile {
    pub fn endpoint(&self) -> String {
        format!(
            "{}://{}:{}{}",
            self.protocol, self.host, self.port, self.path
        )
    }
}

fn local_storage() -> Option<web_sys::Storage> {
    crate::dom::window().local_storage().ok().flatten()
}

fn default_profiles() -> Vec<SiteProfile> {
    vec![
        SiteProfile {
            id: "seed-aruaru-web".to_string(),
            name: "aruaru-db (ローカル)".to_string(),
            purpose: "aruaru-web".to_string(),
            protocol: "http".to_string(),
            host: "localhost".to_string(),
            port: 4000,
            path: "/graphql".to_string(),
            backend_stack: "Rust + async-graphql (aruaru-db)".to_string(),
        },
        SiteProfile {
            id: "seed-other-example".to_string(),
            name: "他プロジェクト用(例)".to_string(),
            purpose: "other".to_string(),
            protocol: "http".to_string(),
            host: "tool.example.local".to_string(),
            port: 9000,
            path: "/graphql".to_string(),
            backend_stack: "任意(PHP + Laravel / Python + FastAPI / Rust + Poem など)"
                .to_string(),
        },
    ]
}

pub fn load_profiles() -> Vec<SiteProfile> {
    if let Some(storage) = local_storage() {
        if let Ok(Some(raw)) = storage.get_item(STORAGE_KEY) {
            if let Ok(profiles) = serde_json::from_str::<Vec<SiteProfile>>(&raw) {
                if !profiles.is_empty() {
                    return profiles;
                }
            }
        }
    }
    let seeded = default_profiles();
    save_profiles(&seeded);
    seeded
}

pub fn save_profiles(profiles: &[SiteProfile]) {
    if let Some(storage) = local_storage() {
        if let Ok(raw) = serde_json::to_string(profiles) {
            let _ = storage.set_item(STORAGE_KEY, &raw);
        }
    }
}

pub fn active_profile_id() -> Option<String> {
    local_storage().and_then(|s| s.get_item(ACTIVE_KEY).ok().flatten())
}

pub fn set_active_profile_id(id: &str) {
    if let Some(storage) = local_storage() {
        let _ = storage.set_item(ACTIVE_KEY, id);
    }
}

/// アクティブなプロファイルのエンドポイントURL。未設定なら先頭のプロファイル。
pub fn active_endpoint() -> String {
    let profiles = load_profiles();
    let active_id = active_profile_id();
    let chosen = active_id
        .as_deref()
        .and_then(|id| profiles.iter().find(|p| p.id == id))
        .or_else(|| profiles.first());
    chosen
        .map(|p| p.endpoint())
        .unwrap_or_else(|| "http://localhost:4000/graphql".to_string())
}

pub fn active_profile_name() -> String {
    let profiles = load_profiles();
    let active_id = active_profile_id();
    active_id
        .as_deref()
        .and_then(|id| profiles.iter().find(|p| p.id == id))
        .or_else(|| profiles.first())
        .map(|p| p.name.clone())
        .unwrap_or_else(|| "(未設定)".to_string())
}

fn new_id() -> String {
    format!("site-{}", js_sys::Date::now() as u64)
}

/// 「サイト管理」タブの一覧+フォームを再描画する。
pub fn render_site_manager() {
    let profiles = load_profiles();
    let active_id = active_profile_id().unwrap_or_default();

    let mut list_html = String::new();
    if profiles.is_empty() {
        list_html.push_str("<p class=\"muted\">登録済みサイトはありません。下のフォームから追加してください。</p>");
    }
    for p in &profiles {
        let is_active = p.id == active_id || (active_id.is_empty() && profiles.first().map(|f| f.id.clone()) == Some(p.id.clone()));
        list_html.push_str(&format!(
            r#"<div class="site-card{active_class}">
  <div class="site-card-main">
    <div class="site-card-title">{name} {badge}</div>
    <div class="site-card-meta muted">{purpose_label} ・ {endpoint}</div>
    <div class="site-card-stack muted">スタック: {stack}</div>
  </div>
  <div class="site-card-actions">
    <button class="site-select" data-id="{id}">選択</button>
    <button class="site-test" data-id="{id}">接続テスト</button>
    <button class="site-edit" data-id="{id}">編集</button>
    <button class="site-delete" data-id="{id}">削除</button>
    <span class="test-result muted" id="test-result-{id}"></span>
  </div>
</div>"#,
            active_class = if is_active { " active" } else { "" },
            name = esc(&p.name),
            badge = if is_active { "<span class=\"badge\">現在の接続</span>" } else { "" },
            purpose_label = if p.purpose == "aruaru-web" { "aruaru-web用" } else { "他の用途" },
            endpoint = esc(&p.endpoint()),
            stack = esc(&p.backend_stack),
            id = esc(&p.id),
        ));
    }

    if let Some(el) = try_by_id("site-list") {
        el.set_inner_html(&list_html);
    }
    wire_site_list_buttons();
}

fn wire_site_list_buttons() {
    use wasm_bindgen::prelude::*;
    use web_sys::{Event, HtmlButtonElement};

    let doc = document();

    type Handler = fn(String);
    let wiring: [(&str, Handler); 4] = [
        ("site-select", on_select_site),
        ("site-test", on_test_site),
        ("site-edit", on_edit_site),
        ("site-delete", on_delete_site),
    ];
    for (class, handler) in wiring {
        if let Ok(nodes) = doc.query_selector_all(&format!(".{class}")) {
            for i in 0..nodes.length() {
                if let Some(node) = nodes.get(i) {
                    if let Ok(btn) = node.dyn_into::<HtmlButtonElement>() {
                        let id = btn.get_attribute("data-id").unwrap_or_default();
                        let closure = Closure::<dyn FnMut(Event)>::new(move |_evt: Event| {
                            handler(id.clone());
                        });
                        btn.set_onclick(Some(closure.as_ref().unchecked_ref()));
                        closure.forget();
                    }
                }
            }
        }
    }
}

fn on_select_site(id: String) {
    set_active_profile_id(&id);
    render_site_manager();
    crate::dom::set_status(&format!(
        "接続先を「{}」に切り替えました。",
        active_profile_name()
    ));
    sync_endpoint_field();
}

/// カードの「接続テスト」ボタン。アクティブなサイトを変えずに疎通確認だけ行う。
fn on_test_site(id: String) {
    let profiles = load_profiles();
    let Some(profile) = profiles.iter().find(|p| p.id == id).cloned() else {
        return;
    };
    let result_id = format!("test-result-{}", profile.id);
    if let Some(el) = try_by_id(&result_id) {
        el.set_text_content(Some("確認中…"));
    }
    wasm_bindgen_futures::spawn_local(async move {
        let endpoint = profile.endpoint();
        let outcome = crate::graphql::post_graphql(
            &endpoint,
            crate::graphql::REGISTRY_SUMMARY_QUERY,
            serde_json::Value::Null,
        )
        .await;
        let message = match outcome {
            Ok(_) => "✅ 接続成功".to_string(),
            Err(e) => format!("❌ 接続失敗: {e}"),
        };
        if let Some(el) = try_by_id(&result_id) {
            el.set_text_content(Some(&message));
        }
    });
}

fn on_delete_site(id: String) {
    let mut profiles = load_profiles();
    profiles.retain(|p| p.id != id);
    save_profiles(&profiles);
    if active_profile_id().as_deref() == Some(id.as_str()) {
        if let Some(first) = profiles.first() {
            set_active_profile_id(&first.id);
        }
    }
    render_site_manager();
    sync_endpoint_field();
}

fn on_edit_site(id: String) {
    let profiles = load_profiles();
    if let Some(p) = profiles.iter().find(|p| p.id == id) {
        fill_form(p);
        if let Some(el) = try_by_id("site-form-id") {
            el.set_attribute("value", &p.id).ok();
        }
    }
}

fn fill_form(p: &SiteProfile) {
    let set_val = |id: &str, v: &str| {
        if let Some(el) = try_by_id(id) {
            if let Ok(input) = el.dyn_into::<HtmlInputElement>() {
                input.set_value(v);
            }
        }
    };
    set_val("site-name", &p.name);
    set_val("site-host", &p.host);
    set_val("site-port", &p.port.to_string());
    set_val("site-path", &p.path);
    set_val("site-stack", &p.backend_stack);

    if let Some(el) = try_by_id("site-purpose") {
        if let Ok(select) = el.dyn_into::<HtmlSelectElement>() {
            select.set_value(&p.purpose);
        }
    }
    if let Some(el) = try_by_id("site-protocol") {
        if let Ok(select) = el.dyn_into::<HtmlSelectElement>() {
            select.set_value(&p.protocol);
        }
    }
}

pub fn clear_form() {
    if let Some(el) = try_by_id("site-form-id") {
        el.set_attribute("value", "").ok();
    }
    let set_val = |id: &str, v: &str| {
        if let Some(el) = try_by_id(id) {
            if let Ok(input) = el.dyn_into::<HtmlInputElement>() {
                input.set_value(v);
            }
        }
    };
    set_val("site-name", "");
    set_val("site-host", "");
    set_val("site-port", "8080");
    set_val("site-path", "/graphql");
    set_val("site-stack", "");
}

/// 「保存」ボタン押下時のハンドラ。新規追加/既存編集の両方を扱う。
pub fn on_save_site() {
    let get_val = |id: &str| -> String {
        by_id(id)
            .dyn_into::<HtmlInputElement>()
            .map(|i| i.value())
            .unwrap_or_default()
    };
    let get_select = |id: &str| -> String {
        by_id(id)
            .dyn_into::<HtmlSelectElement>()
            .map(|s| s.value())
            .unwrap_or_default()
    };

    let existing_id = get_val("site-form-id");
    let name = get_val("site-name");
    let host = get_val("site-host");
    let path_raw = get_val("site-path");
    let port_raw = get_val("site-port");
    let stack = get_val("site-stack");
    let purpose = get_select("site-purpose");
    let protocol = get_select("site-protocol");

    if name.trim().is_empty() || host.trim().is_empty() {
        crate::dom::set_status("サイト名と接続先ホスト(IP/ドメイン)は必須です。");
        return;
    }
    let port: u16 = match port_raw.trim().parse::<u32>() {
        Ok(p) if (1..=65535).contains(&p) => p as u16,
        _ => {
            crate::dom::set_status(&format!(
                "ポート番号が不正です(1〜65535の数値を入力してください): \"{port_raw}\""
            ));
            return;
        }
    };
    let path = if path_raw.trim().is_empty() {
        "/graphql".to_string()
    } else if path_raw.starts_with('/') {
        path_raw
    } else {
        format!("/{path_raw}")
    };

    let mut profiles = load_profiles();
    if !existing_id.is_empty() {
        if let Some(p) = profiles.iter_mut().find(|p| p.id == existing_id) {
            p.name = name;
            p.purpose = purpose;
            p.protocol = protocol;
            p.host = host;
            p.port = port;
            p.path = path;
            p.backend_stack = stack;
        }
    } else {
        let new_profile = SiteProfile {
            id: new_id(),
            name,
            purpose,
            protocol,
            host,
            port,
            path,
            backend_stack: stack,
        };
        if active_profile_id().is_none() {
            set_active_profile_id(&new_profile.id);
        }
        profiles.push(new_profile);
    }
    save_profiles(&profiles);
    clear_form();
    render_site_manager();
    sync_endpoint_field();
    crate::dom::set_status("サイト情報を保存しました。");
}

/// SQL/レジストリタブのエンドポイント表示欄をアクティブなサイトに合わせる。
pub fn sync_endpoint_field() {
    if let Some(el) = try_by_id("endpoint") {
        if let Ok(input) = el.dyn_into::<HtmlInputElement>() {
            input.set_value(&active_endpoint());
        }
    }
    if let Some(el) = try_by_id("active-site-name") {
        el.set_text_content(Some(&active_profile_name()));
    }
}
