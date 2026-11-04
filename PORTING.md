# PORTING.md — aruaru-web お引越しファイル

> このファイル 1 枚で、**どのプロジェクトにも `aruaru-web` を導入・移設**
> できます。新しいフロントエンド用リポジトリにこのファイルをコピーして、
> 上から順に進めてください。
>
> 最終更新: 2026-07-12

---

## 0. aruaru-web とは何か

`aruaru-db`(分散 Git-on-SQL データベース)が公開する GraphQL エンドポイント
(`/graphql`)向けの、Rust → WebAssembly 製のブラウザ UI。単一クレート構成
(`wasm-bindgen`/`web-sys`のみ、重量級 Rust Web フレームワーク不使用)。
SQL入力欄からの`sql`クエリ実行、`registrySummary`(対応DBレジストリ集計)、
`branches`(ブランチ一覧)、`log`(コミットログ)を実`fetch()`で呼び出す。
接続失敗時は実スキーマと同形のオフラインサンプルデータへ自動フォールバック。

## 1. 持っていくもの(ファイル一覧)

```
aruaru-web/
├── Cargo.toml         ← 単一crate定義(crate-type = ["cdylib", "rlib"])
├── src/lib.rs          ← 全ロジック(単一ファイル)
├── index.html          ← 静的シェル(pkg/を読み込む)
└── CLAUDE.md           ← 開発ルール(参考)
```

## 2. 依存パッケージ

```toml
[dependencies]
wasm-bindgen = "..."
wasm-bindgen-futures = "..."
js-sys = "..."
web-sys = { version = "...", features = [...] }
serde = { version = "...", features = ["derive"] }
serde_json = "..."
```

Yew/Leptos/Dioxus 等の重量級 Rust Web フレームワークは強い理由がない限り
採用しない(エコシステム全体の方針: 依存最小・自前実装優先)。

## 3. ビルド手順

```bash
rustup target add wasm32-unknown-unknown         # 初回のみ
cargo install wasm-bindgen-cli --version 0.2.126 # 初回のみ(Cargo.lockと一致させる)
cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg \
  target/wasm32-unknown-unknown/debug/aruaru_web.wasm
python -m http.server 8080   # index.html + pkg/ を配信
```

## 4. 接続先を変える場合

`src/lib.rs`の`current_endpoint()`が UI 入力欄(`#endpoint`)の値を読む形に
なっている。別の GraphQL サーバーに向ける場合は、この入力欄のデフォルト値
(`index.html`内)を変更するだけでよく、Rust コード自体は変更不要。

## 5. 使っている GraphQL クエリ

- `sql(query: String!) -> QueryResultGql { columns rows commandTag }`
- `registrySummary -> RegistrySummaryGql { total connectable ga beta pgCompatible planned }`
- `branches { name headCommitId isCurrent }`
- `log(limit: Int!) { shortId author message timestamp }`

接続先が`aruaru-db`と異なるサーバーの場合、これらのクエリ/フィールド名が
一致するようスキーマを合わせるか、`src/lib.rs`側のクエリ文字列を変更する。

## 6. 開発ルール

このプロジェクトのコーディング方針・関連プロジェクトは
[`CLAUDE.md`](CLAUDE.md)、開発ルールの正本は
[`open-raid-z`](https://github.com/aon-co-jp/open-raid-z)の`CLAUDE.md`を参照。

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

### Deutsch (KI-Übersetzung, redaktionell überarbeitet; noch nicht von Muttersprachlern geprüft)
- **REST wird vollständig abgeschafft; neue APIs entstehen nicht mehr als REST.** APIs, die aus dem Browser oder per `curl` aufgerufen werden, verwenden **unäre Connect-Aufrufe** (`POST /<Paket>.<Service>/<Methode>`, `content-type: application/json`, JSON-Body). Zwischen Diensten kommen **gRPC bzw. gRPC-web** zum Einsatz, im selben Stil wie die Admin-API von RPoem (`open_runo.admin.v1.Admin`). Streaming wird erst ergänzt, wenn es tatsächlich gebraucht wird.
- **Bestehende REST-Endpunkte werden schrittweise umgestellt, und zwar immer dann, wenn man ohnehin an ihnen arbeitet.** Sobald ein Endpunkt umgestellt ist, wird die alte REST-Route entfernt. Bis zum Abschluss der Migration bleiben die übrigen bestehen, es werden aber **keine neuen mehr hinzugefügt**.
- **JSON wird einheitlich über RS-JSON verarbeitet**: Gelesen wird mit `from_str_secure` bzw. `parse_secure` (mit Obergrenzen für Größe, Verschachtelungstiefe und doppelte Schlüssel), geschrieben mit `rust_json::fast`. `prost` wird nicht in das Browser-WASM aufgenommen, da es die Datei vergrößern würde.
- **Ausnahmen (Schnittstellen, deren Form ein fremder Standard vorgibt und die nicht zu unserer eigenen API gehören)**: ACME, OAuth/OIDC-Callbacks, Webhooks von Drittanbietern (z. B. GitHub), Health-Checks, die Auslieferung statischer Dateien, OTLP und Ähnliches. Diese nehmen, wie vom jeweiligen Standard verlangt, gewöhnliches HTTP entgegen.
- Referenzimplementierungen: `RPoem` (`open_runo.admin.v1.Admin`) und `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Hinweis: Ob der Code eines einzelnen Repositorys diese Richtlinie bereits erfüllt, hängt vom jeweiligen Repository ab. Dieser Abschnitt dokumentiert lediglich die Richtlinie; er besagt nicht, dass der Code schon angepasst wurde.

### Русский (перевод выполнен с помощью ИИ и отредактирован; носителями языка пока не проверен)
- **От REST мы полностью отказываемся; новые API больше не создаются в стиле REST.** API, к которым обращаются из браузера или через `curl`, используют **унарные вызовы Connect** (`POST /<пакет>.<Service>/<Method>`, `content-type: application/json`, тело запроса — JSON). Для взаимодействия между сервисами применяются **gRPC и gRPC-web** — в том же стиле, что и административный API RPoem (`open_runo.admin.v1.Admin`). Потоковая передача данных добавляется лишь тогда, когда в ней действительно возникает необходимость.
- **Существующие REST-эндпоинты переводятся поэтапно — по мере того, как с ними приходится работать.** После перевода старый REST-маршрут удаляется. Пока миграция не завершена, оставшиеся эндпоинты сохраняются, но **новые не добавляются**.
- **Работа с JSON унифицирована на основе RS-JSON**: чтение — через `from_str_secure` или `parse_secure` (с ограничениями на размер, глубину вложенности и повторяющиеся ключи), запись — через `rust_json::fast`. `prost` в браузерный WASM не включается, поскольку это увеличило бы размер файла.
- **Исключения (интерфейсы, форму которых определяет чужой стандарт, а не наш собственный API)**: ACME, обратные вызовы OAuth/OIDC, вебхуки сторонних сервисов (например, GitHub), проверки работоспособности, раздача статических файлов, OTLP и тому подобное. Такие интерфейсы принимают обычный HTTP, как того требует соответствующий стандарт.
- Эталонные реализации: `RPoem` (`open_runo.admin.v1.Admin`) и `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Примечание: соответствует ли код конкретного репозитория этой политике, зависит от самого репозитория. Данный раздел лишь фиксирует политику и не означает, что код уже изменён.
