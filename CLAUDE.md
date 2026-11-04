# 技術スタック・開発ルール(aruaru-web)

このリポジトリ、および関連プロジェクト(`open-runo`/`open-web-server`/
`aruaru-db`/`poem-cosmo-tauri`/`open-raid-z`/`rs-to-readme`)で開発・保守を
行う際は、以下を基本方針とする。作業ドライブは `F:\open-runo`(E:ドライブは
2026-07-10に消失、以後Fが実体)。この節は
[`open-raid-z`](https://github.com/aon-co-jp/open-raid-z) の `CLAUDE.md`
を正本とし、各プロジェクトへコピーして同期する。

## このリポジトリの役割

`aruaru-web` は `aruaru-db`(分散 Git-on-SQL データベース)向けの Web UI。
`aruaru-db` が GraphQL(`/graphql`)で公開する `sql`/`registrySummary` 等の
クエリをブラウザから実行し、結果を表示する。**このリポジトリは
`aruaru-db`/`open-runo`/`open-web-server` とは別チームの並行作業対象であり、
それらのディレクトリには立ち入らない**(README/CLAUDE.mdの参照のみ可)。

## フロントエンド(2026-07-10、方針更新 — open-raid-zより)

- Tauriパッケージには直接依存しない。ただしTauriのデスクトップUI体験・
  `invoke()`的なコマンド呼び出しインターフェースとは互換性を保つ。
- **HTML5/CSS3・TypeScript・Bootstrap・Node.jsのスタックは廃止**。
  Rustをメイン言語としてフロントエンドを構成し、**WebAssembly (WASM)**に
  置き換える(コンパイル対象はRust → `wasm32-unknown-unknown`)。DOM操作・
  `fetch()`呼び出しはRust製WASMモジュール側(`wasm-bindgen` + `web-sys`)で
  行い、TypeScript/Node.jsのビルドチェーンには依存しない。重量級のRust製
  Webフレームワーク(Yew/Leptos/Dioxus等)も、強い理由がない限り採用しない
  (エコシステム全体の方針: 依存最小・自前実装優先)。
  https://webassembly.org/ | https://rustwasm.github.io/

## バックエンド・コア(エコシステム全体の方針、参考)

- **Rust**(メイン言語、標準ライブラリ中心): https://www.rust-lang.org/ja/
- **tokio** + **hyper**(Webフレームワークなしで直接HTTPサーバを自前実装):
  https://tokio.rs/ | https://docs.rs/hyper/latest/hyper/
- Poemパッケージには依存しないが、Poemのルーティング/ハンドラAPI形状とは
  互換性のあるインターフェースを維持する方針(open-runo/poem-cosmo-tauri側)。
- `aruaru-web` 自体はバックエンドを持たない(静的な WASM + `index.html` の
  みで、接続先の `aruaru-db` がバックエンド)。

## API設計思想(参考・概念のみ)

- **VersionLess API**という考え方を参考にする(WunderGraphのブログ/podcast参照)。
- **WunderGraph Cosmo**: パッケージとしては直接依存させない。GraphQL
  Federation / VersionlessAPI というAPI形状・コンセプトのみ参考にする。
  https://github.com/wundergraph/cosmo
- `aruaru-db` 側は `async-graphql` による Federation サブグラフを実装済み
  (`aruaru-db/crates/aruaru-graphql`)。`aruaru-web` はこのサブグラフの
  クライアントとして動作する。

## poem-cosmo-tauri と open-runo の違い(2026-07-11、ユーザー確認済み、open-raid-z正本より転記)

両者は共通コア(Cosmo有料版機能のOSS Rust再実装)を持つが**全く違う
リポジトリのプロジェクト**であり統合対象ではない。poem-cosmo-tauri は
さらに範囲が広く、Poem/Tauriの**全機能をAI駆動開発で一から自作・再現する**
という上乗せ目標を持つ(open-runoにはない)。詳細は open-raid-z の
`CLAUDE.md` を参照。

## ビルド手順(このリポジトリ固有)

```bash
rustup target add wasm32-unknown-unknown        # 初回のみ
cargo install wasm-bindgen-cli --version 0.2.126 # 初回のみ(Cargo.lockと一致させる)
cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg \
  target/wasm32-unknown-unknown/debug/aruaru_web.wasm
python -m http.server 8080   # index.html + pkg/ を配信
```

## 関連プロジェクト

- **open-runo**: https://github.com/aon-co-jp/open-runo
- **open-web-server**: https://github.com/aon-co-jp/open-web-server
- **aruaru-db**(このUIの接続先): https://github.com/aon-co-jp/aruaru-db
- **aruaru-web**(このリポジトリ): https://github.com/aon-co-jp/aruaru-web
- **poem-cosmo-tauri**: https://github.com/aon-co-jp/poem-cosmo-tauri
- **open-raid-z**(開発ルールの正本): https://github.com/aon-co-jp/open-raid-z
- **rs-to-readme**: https://github.com/aon-co-jp/rs-to-readme

## 運用ルール

- **開発中はこの`CLAUDE.md`を、コード変更のコミット/pushと必ず一緒に
  push する**(内容を更新した場合はもちろん、変更が無い場合も他の変更と
  一緒にコミット対象へ含めておくこと)。
- 実装で迷った場合や、API仕様の詳細確認が必要な場合は、学習データからの
  推測より公式ドキュメント(上記URL)、または `aruaru-db` 側の実ソース
  (`aruaru-db/crates/aruaru-graphql/src/lib.rs` 等)を優先して参照する。
- 作業ドライブが変わった場合は、この節を更新し、関連プロジェクトの
  引き継ぎ資料にも変更の経緯を記録すること。
- **無人自動開発(確認不要・自動デバッグ)のタイミングでは、20〜30分おきの
  スケジュール実行待ちにせず、1パス内でできる限り連続して作業を進める**
  こと。小さく検証可能な単位(1機能ごとに `cargo build --target
  wasm32-unknown-unknown` → 実ブラウザ確認 → commit → push)を保ちながら
  進める。
- **このリポジトリはエコシステム内で優先度が低い(他リポジトリが安定稼働中の
  ため)。過剰なスコープ拡大はせず、小さく実用的な増分を積み重ねる。**
- **ドキュメント(parity調査・ギャップ一覧・HANDOFF等)に「未調査」「未着手」
  「意図的に先送り」「将来検討」等と記録されている項目を見つけた場合、
  それは承認待ちの提案ではなく、次に着手すべき実装対象そのものである**
  (ユーザー指示、2026-07-12)。「ご指示はありますか?」のように追加の
  確認を求めて手を止めず、確認不要でそのまま実装を進めること。
  優先度・実現可能性の判断(例: ブラウザ実行という設計上の制約で
  技術的に不可能な項目は代替案を検討する)は自分で行ってよいが、
  「未着手だから今回は見送る」という判断そのものをユーザーへの
  確認なしに下してはならない——見送る場合も、まず着手を試み、
  真に不可能/著しく非現実的と判明した場合のみ、その理由をドキュメントに
  明記した上で次の項目に進む。

## 現状(このリポジトリ固有)

- 2026-07-11 に本リポジトリを空の状態からブートストラップ(初回コミット)。
- 単一クレート構成(`Cargo.toml` + `src/lib.rs`、workspaceではなく単体crate)。
  `crate-type = ["cdylib", "rlib"]`、依存は `wasm-bindgen`/`wasm-bindgen-futures`/
  `js-sys`/`web-sys`/`serde`/`serde_json` のみ。重量級フレームワーク無し。
- 実装済み機能: SQL入力欄からの `aruaru-graphql` `/graphql` への実
  `fetch()`(`sql`/`registrySummary` クエリ)、接続失敗時は実スキーマと
  同形のオフラインサンプルをレンダリングしてその旨を明示。
- `cargo check --target wasm32-unknown-unknown` / `cargo build --target
  wasm32-unknown-unknown` ともに警告0件で成功。`wasm-bindgen-cli 0.2.126`
  (Cargo.lockの`wasm-bindgen`バージョンと一致)で `pkg/` を生成し、実
  Chromiumブラウザで起動ログ・DOM構築・両ボタンのクリック→実`fetch()`→
  オフラインサンプル描画までを確認済み。
- `todo!()`/`unimplemented!()`/TODO/FIXMEマーカーは0件(実装した範囲は
  スタブなしで完結)。

## HANDOFF(直近の自動巡回ログ、上が最新)

- **2026-07-11(今回・初回パス)**: 空リポジトリからブートストラップ。
  `aruaru-db/crates/aruaru-graphql`(async-graphql、Federation サブグラフ、
  `aruaru-server` が `:4000/graphql` で配信)のスキーマを実ソースから確認し、
  `VcsQuery::sql(query: String!) -> QueryResultGql`(`columns`/`rows`/
  `commandTag`)と `AdminQuery::registrySummary -> RegistrySummaryGql`
  (`total`/`connectable`/`ga`/`beta`/`pgCompatible`/`planned`)を実際に
  呼び出すクライアントUIを実装(`src/lib.rs`、単一ファイル)。
  `poem-cosmo-tauri/apps/desktop-wasm` が既に確立していたビルド規約
  (`cargo build --target wasm32-unknown-unknown` →
  `wasm-bindgen --target web --no-typescript --out-dir pkg`、
  `wasm-bindgen-cli`はCargo.lockのバージョンにピン留め)をそのまま踏襲。
  実際に `cargo install wasm-bindgen-cli --version 0.2.126` → `wasm-bindgen`
  実行 → `python -m http.server` で配信 → 実Chromiumブラウザで
  `index.html` を開き、WASM起動ログ・DOM構築を確認、さらに「SQLを実行」
  「レジストリ集計を取得」の両ボタンを実際にクリックして
  (`aruaru-server` は本セッションでは起動していないため)接続失敗
  → オフラインサンプルデータ(実スキーマと同形)への自動フォールバックが
  正しく動作することまで確認済み。README(ルート+10言語)・このCLAUDE.md・
  `.gitignore` を作成し、初回コミット・push実施予定(このコミットハッシュは
  `git log` 参照)。
  **次回パスがすべきこと**: (1) 実際に `aruaru-server` を起動して
  (`cargo run -p aruaru-server -- --data ./data --raft-id 1`)、本物の
  `sql`/`registrySummary` レスポンスがこのUIで正しく描画されることを
  実接続で確認(今回はサーバー未起動のためオフラインフォールバック経路の
  みを検証した)、(2) 価値があれば `branches`/`log`/`diff` など他の
  VcsQueryクエリ、または `registry`(DB一覧)クエリを追加UIとして実装、
  (3) CORS設定(`aruaru-server`側は`poem::middleware::Cors::new()`で
  全許可済みなので現状は追加対応不要のはずだが、実接続確認時に問題が
  出れば調査)、(4) 結果テーブルの見た目・エラー表示の細部改善は優先度低。

## API 方針: REST 全廃・gRPC / Connect へ (2026-11-04)

### 日本語
- **REST は全廃する。新しい API は REST で作らない。** ブラウザや `curl` から呼ぶ API は **Connect の単純呼び出し**(`POST /<パッケージ>.<Service>/<Method>`、`content-type: application/json`、本文は JSON)、サービス間は **gRPC / gRPC-web**(RPoem の管理API `open_runo.admin.v1.Admin` と同じ流儀)。ストリーミングは、必要になってから。
- **既存の REST は、触るときに順に移す。** 移したら旧 REST の経路は消す。移行が終わるまで残るが、**新規には追加しない**。
- **JSON は RS-JSON に統一**する(読み込みは `from_str_secure` / `parse_secure` でサイズ・深さ・重複キーの上限つき、書き出しは `rust_json::fast`)。ブラウザの WASM に `prost` は入れない(サイズが増えるため)。
- **例外(自分たちの API ではなく、相手の規格が決めているもの)**: ACME、OAuth/OIDC のコールバック、第三者(GitHub など)からの Webhook、ヘルスチェック、静的ファイル配信、OTLP など。これらは規格どおりの HTTP を受ける。
- 参考実装: `RPoem`(`open_runo.admin.v1.Admin`)、`open-easy-web`(`server/src/connect.rs`、`proto/open_easy_web/admin/v1/admin.proto`)。
- ※ このリポジトリのコードが既に方針どおりかどうかは、各リポジトリの現状による。この節は方針の記録であり、コードの変更を意味しない。

### English
- **REST is being abolished. New APIs are not built as REST.** APIs called from browsers or `curl` use **Connect unary calls** (`POST /<package>.<Service>/<Method>`, `content-type: application/json`, JSON body); service-to-service calls use **gRPC / gRPC-web** (the same style as RPoem's admin API `open_runo.admin.v1.Admin`). Streaming only when it is actually needed.
- **Existing REST endpoints are migrated one by one, when they are touched.** The old REST route is removed once migrated. Until migration is finished they remain, but **no new ones are added**.
- **JSON is unified on RS-JSON**: reading with `from_str_secure` / `parse_secure` (size, depth and duplicate-key limits), writing with `rust_json::fast`. `prost` is not put into browser WASM (it would grow the binary).
- **Exceptions (shapes dictated by someone else's standard, not our own API)**: ACME, OAuth/OIDC callbacks, webhooks from third parties (e.g. GitHub), health checks, static file serving, OTLP, and so on. These accept plain HTTP as the standard requires.
- Reference implementations: `RPoem` (`open_runo.admin.v1.Admin`) and `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Note: whether the code in a given repository already follows this policy depends on that repository. This section records the policy; it does not mean the code has been changed.

### Deutsch (KI-Übersetzung, noch nicht von Muttersprachlern geprüft)
- **REST wird abgeschafft. Neue APIs werden nicht als REST gebaut.** APIs, die aus dem Browser oder mit `curl` aufgerufen werden, nutzen **Connect-Unary-Aufrufe** (`POST /<Paket>.<Service>/<Methode>`, `content-type: application/json`, JSON-Body); zwischen Diensten gilt **gRPC / gRPC-web** (wie bei der Admin-API von RPoem, `open_runo.admin.v1.Admin`). Streaming erst, wenn es wirklich nötig ist.
- **Bestehende REST-Endpunkte werden einzeln migriert, sobald man sie anfasst.** Nach der Migration wird die alte REST-Route entfernt. Bis dahin bleiben sie bestehen, es werden aber **keine neuen hinzugefügt**.
- **JSON wird auf RS-JSON vereinheitlicht**: Lesen mit `from_str_secure` / `parse_secure` (Grenzen für Größe, Tiefe und doppelte Schlüssel), Schreiben mit `rust_json::fast`. `prost` kommt nicht in Browser-WASM (die Datei würde größer).
- **Ausnahmen (Formate, die ein fremder Standard vorgibt, nicht unsere eigene API)**: ACME, OAuth/OIDC-Callbacks, Webhooks von Dritten (z. B. GitHub), Health-Checks, statische Dateien, OTLP usw. Diese nehmen schlichtes HTTP entgegen, wie es der Standard verlangt.
- Referenzimplementierungen: `RPoem` (`open_runo.admin.v1.Admin`) und `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Hinweis: Ob der Code eines Repositorys diese Richtlinie schon befolgt, hängt vom jeweiligen Repository ab. Dieser Abschnitt hält die Richtlinie fest; er bedeutet nicht, dass der Code bereits geändert wurde.

### Русский (машинный перевод, носителями языка ещё не проверен)
- **REST полностью отменяется. Новые API не создаются как REST.** API, вызываемые из браузера или через `curl`, используют **унарные вызовы Connect** (`POST /<пакет>.<Service>/<Method>`, `content-type: application/json`, тело — JSON); между сервисами — **gRPC / gRPC-web** (так же, как административный API RPoem, `open_runo.admin.v1.Admin`). Потоковая передача — только когда она действительно нужна.
- **Существующие REST-эндпоинты переносятся по одному, когда до них доходят руки.** После переноса старый REST-маршрут удаляется. До завершения переноса они остаются, но **новые не добавляются**.
- **JSON унифицируется на RS-JSON**: чтение через `from_str_secure` / `parse_secure` (ограничения на размер, глубину и повторяющиеся ключи), запись через `rust_json::fast`. `prost` не добавляется в браузерный WASM (размер вырос бы).
- **Исключения (форматы, заданные чужим стандартом, а не наш собственный API)**: ACME, обратные вызовы OAuth/OIDC, вебхуки сторонних сервисов (например, GitHub), проверки работоспособности, раздача статических файлов, OTLP и т. п. Они принимают обычный HTTP, как требует стандарт.
- Эталонные реализации: `RPoem` (`open_runo.admin.v1.Admin`) и `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Примечание: соответствует ли код конкретного репозитория этой политике, зависит от самого репозитория. Этот раздел фиксирует политику и не означает, что код уже изменён.
