# aruaru-web

**aruaru-db 用の最小限の Web UI(Rust → WebAssembly、フレームワーク不使用)**

`aruaru-db`(分散 Git-on-SQL データベース)が GraphQL(`/graphql`)で公開する
`sql` クエリと `registrySummary`(対応DBレジストリ集計)を、ブラウザから
実際に叩いて結果を表示する、最初のブートストラップ版です。**まだ本格的な
管理画面ではありません** — SQL入力欄・レジストリ集計ボタン・結果テーブルの
3点のみの、意図的に小さいスコープです。

📖 他の言語: [日本語](README-Japan.md) / [English](README-English.md) /
[中文](README-Chinese.md) / [한국어](README-Korea.md) / [Español](README-Spain.md) /
[Français](README-France.md) / [Deutsch](README-Germany.md) / [Italiano](README-Italy.md) /
[Русский](README-Russia.md) / [العربية](README-Arabic.md)

---

## いまできること

- `aruaru-graphql`(`aruaru-db/crates/aruaru-graphql`)の `/graphql` エンドポイントへ、
  実際に `fetch()` で GraphQL リクエストを送信:
  - `sql(query: String!): QueryResultGql` — 任意のSQLを実行し `columns`/`rows`/`commandTag` を
    テーブル表示
  - `registrySummary: RegistrySummaryGql` — 対応DBレジストリ(150件超)の集計を
    カード表示
- `aruaru-server` が起動していない/接続できない場合は、**実スキーマと同じ形の
  サンプルデータ**をその場で描画し、「オフラインサンプル」である旨を明示する
  (この挙動は実ブラウザで検証済み — 下記「動作確認」参照)。
- エンドポイントURLはUI上の入力欄で変更可能(デフォルト `http://localhost:4000/graphql`)。

## いまできないこと(正直な範囲)

- GraphQL Mutation(ブランチ作成・マージ・レジストリcrawl等)は未実装。
- 認証・ページネーション・エラー時の自動リトライは未実装。
- Tauriのようなネイティブアプリ体験は提供しない(ブラウザで動くWASMのみ)。

## ビルド方法

Node.js・npm・TypeScriptは使わない。Rustツールチェーンのみで完結する。

```bash
rustup target add wasm32-unknown-unknown        # 初回のみ
cargo install wasm-bindgen-cli --version 0.2.126 # 初回のみ(Cargo.lockのバージョンと一致させること)

cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg \
  target/wasm32-unknown-unknown/debug/aruaru_web.wasm

# 静的サーバーで配信して開く(何でもよい。例:)
python -m http.server 8080
# ブラウザで http://localhost:8080/index.html を開く
```

`aruaru-db` を実際に動かして試す場合:

```bash
cd ../aruaru-db
cargo run -p aruaru-server -- --data ./data --raft-id 1   # :4000 に GraphQL が立つ
```

## 動作確認(このパスで実施)

- `cargo check --target wasm32-unknown-unknown` / `cargo build --target wasm32-unknown-unknown`
  ともに成功(警告0件)。
- `wasm-bindgen --target web` で `pkg/aruaru_web.js` / `pkg/aruaru_web_bg.wasm` を生成し、
  実ブラウザ(Chromium)で `index.html` を読み込み、WASMモジュールの起動ログ・
  DOM構築・「SQLを実行」「レジストリ集計を取得」両ボタンのクリック→実際の
  `fetch()` 発火→接続失敗時のオフラインサンプル描画までを確認済み。

## 構成

```text
aruaru-web/
├── Cargo.toml       # crate-type = ["cdylib", "rlib"]、wasm-bindgen/web-sys依存
├── src/lib.rs        # DOM構築・fetch・GraphQLレスポンス整形(唯一のソースファイル)
├── index.html         # pkg/ を読み込むだけの薄いローダー
├── pkg/                 # wasm-bindgen生成物(.gitignore対象、ビルドで再生成)
└── CLAUDE.md
```

## 関連プロジェクト

- **aruaru-db**(このUIが接続する対象): https://github.com/aon-co-jp/aruaru-db
- **open-runo**: https://github.com/aon-co-jp/open-runo
- **open-web-server**: https://github.com/aon-co-jp/open-web-server
- **poem-cosmo-tauri**: https://github.com/aon-co-jp/poem-cosmo-tauri
- **open-raid-z**(開発ルールの正本): https://github.com/aon-co-jp/open-raid-z
- **rs-to-readme**: https://github.com/aon-co-jp/rs-to-readme

## License

Apache-2.0

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
