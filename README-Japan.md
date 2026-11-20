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
