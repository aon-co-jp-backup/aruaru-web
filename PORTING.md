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
