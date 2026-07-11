# aruaru-web

**aruaru-db 用の最小限の Web UI(Rust → WebAssembly、フレームワーク不使用)**

`aruaru-db`(分散 Git-on-SQL データベース)が GraphQL(`/graphql`)で公開する
`sql` クエリと `registrySummary`(対応DBレジストリ集計)を、ブラウザから
実際に叩いて結果を表示するタブ式ダッシュボードです。SQL実行・レジストリ集計に加え、
KUSANAGIのサイト一覧のように**複数の接続先(aruaru-web用・他プロジェクト用)を
登録して切り替えられる「サイト管理」タブ**を持ちます。

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
- **サイト管理タブ**: aruaru-web用・他プロジェクト用の接続先(IPアドレス/
  ドメイン/サブドメイン/ポート/パス)を複数登録し、`localStorage` に保存して
  ワンクリックで切り替えられる。SQL/レジストリタブのエンドポイント欄は
  選択中のサイトに自動的に追従する。
- **HTTPS(TLS)の自動設定・自動監視・自動更新**: `scripts/gen-vhost.sh` で
  Nginx/Apache の vhost(HTTP→HTTPSリダイレクト込み)を生成し、
  `scripts/setup-tls.sh` で Let's Encrypt(certbot)の証明書取得、
  `deploy/systemd/install-systemd-units.sh` で「1日2回の自動更新
  (`aruaru-tls-renew.timer`)」と「1日1回の失効監視
  (`aruaru-tls-monitor.timer` → `scripts/check-all-tls.sh`)」を有効化できる。
  詳細は「HTTPS・ドメイン/サブドメインの登録」を参照。

## いまできないこと(正直な範囲)

- GraphQL Mutation(ブランチ作成・マージ・レジストリcrawl等)は未実装。
- 認証・ページネーション・エラー時の自動リトライは未実装。
- Tauriのようなネイティブアプリ体験は提供しない(ブラウザで動くWASMのみ)。
- **実際のドメイン取得・DNSレコード登録(レジストラでの操作)はこのリポジトリ
  からは行わない**(費用・外部サービスへの反映を伴うため)。ここで自動化して
  いるのは、取得済みドメインに対する「vhost設定生成」「TLS証明書の取得・
  監視・自動更新」までであり、DNS登録自体は利用者がレジストラで行う。

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

## IPアドレスから起動する

```bash
scripts/serve.sh 0.0.0.0 8080        # 全インターフェースで待受
scripts/serve.sh 192.168.1.50 8080   # 特定のIPアドレスのみで待受
```

## HTTPS・ドメイン/サブドメインの登録

このリポジトリ自体はドメイン取得やDNSレコード登録は行わない(レジストラでの
操作・費用が発生するため利用者が別途実施する)。以下は、取得済みのドメイン/
サブドメインを aruaru-web 用・他プロジェクト用に簡単に払い出すための
ローカル自動化(KUSANAGIの「サイト追加」相当)。

```bash
# 1. ドメイン+IP+バックエンドから vhost(Nginx/Apache、HTTP→HTTPSリダイレクト込み)を生成
scripts/gen-vhost.sh aruaru.example.com 203.0.113.10 127.0.0.1:4000
# 別用途のサブドメインも同様に(UPSTREAM/WEBROOTを変えるだけ)
scripts/gen-vhost.sh tool.example.com 203.0.113.10 127.0.0.1:9000 /var/www/tool

# 2. 生成された設定ファイルを配置してリロード(deploy/generated/ 以下、.gitignore対象)

# 3. TLS証明書を取得(Let's Encrypt / certbot)
scripts/setup-tls.sh aruaru.example.com admin@example.com /var/www/aruaru.example.com

# 4. 自動更新(1日2回)+ 自動監視(1日1回、失効間近を検知)を有効化
sudo deploy/systemd/install-systemd-units.sh
```

登録済みの全ドメインの証明書有効期限は `scripts/check-all-tls.sh` でいつでも
手動確認できる。aruaru-web の GUI 側「サイト管理」タブにも同じ接続先を
登録しておくと、ブラウザからの接続先切り替えと一致させられる。

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
├── Cargo.toml            # crate-type = ["cdylib", "rlib"]、wasm-bindgen/web-sys依存
├── src/
│   ├── lib.rs             # エントリポイント・タブ切り替え・イベント配線
│   ├── dom.rs             # DOM操作の共通ヘルパー
│   ├── graphql.rs         # /graphql への fetch呼び出し
│   ├── render.rs          # SQL結果・レジストリ集計のレンダリング
│   ├── profiles.rs        # サイト管理(接続プロファイル、localStorage保存)
│   └── shell.rs           # HTMLシェル(タブ・フォーム)
├── index.html             # pkg/ を読み込むローダー + CSS
├── pkg/                   # wasm-bindgen生成物(.gitignore対象、ビルドで再生成)
├── scripts/
│   ├── serve.sh            # 任意のIPアドレスから配信する開発サーバー起動
│   ├── gen-vhost.sh         # ドメイン/IPからNginx・Apache vhostを生成
│   ├── setup-tls.sh         # Let's Encrypt証明書の取得
│   ├── check-tls.sh         # 1ドメインの証明書有効期限チェック
│   └── check-all-tls.sh     # 登録済み全ドメインの有効期限を一括チェック
├── deploy/
│   ├── nginx/vhost.conf.template
│   ├── apache/vhost.conf.template
│   ├── systemd/             # 自動更新(renew)・自動監視(monitor)タイマー一式
│   └── generated/           # gen-vhost.shの出力(.gitignore対象)
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
