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

## 現状(このリポジトリ固有)

- 2026-07-11 に本リポジトリを空の状態からブートストラップ(初回コミット)。
- 単一クレート構成(`Cargo.toml`、`src/` は複数モジュールに分割:
  `lib.rs`/`dom.rs`/`graphql.rs`/`render.rs`/`profiles.rs`/`shell.rs`。
  workspaceではなく単体crate)。`crate-type = ["cdylib", "rlib"]`、依存は
  `wasm-bindgen`/`wasm-bindgen-futures`/`js-sys`/`web-sys`/`serde`/`serde_json`
  のみ。重量級フレームワーク無し。
- 実装済み機能:
  - タブ式UI(SQL実行 / レジストリ集計 / サイト管理)。`aruaru-graphql`
    `/graphql` への実 `fetch()`(`sql`/`registrySummary` クエリ)、接続失敗時は
    実スキーマと同形のオフラインサンプルをレンダリングしてその旨を明示。
  - **サイト管理タブ**(`src/profiles.rs`): aruaru-web用・他プロジェクト用の
    接続先(名前/用途/プロトコル/ホスト(IP・ドメイン・サブドメイン)/ポート/
    パス/バックエンドスタック名)を複数登録・編集・削除でき、`localStorage`
    (`aruaru_web_site_profiles_v1`)に保存、選択中のサイトがSQL/レジストリ
    タブのエンドポイントに自動反映される。KUSANAGIのサイト一覧に相当する
    最小限の管理UIで、実際のDNS登録は行わない。
  - **IPアドレスからの起動**: `scripts/serve.sh <BIND_IP> <PORT>` でローカル
    開発サーバーを任意のIP/ポートにbind。
  - **vhost生成・HTTPS自動設定**: `scripts/gen-vhost.sh <DOMAIN> <IP>
    <UPSTREAM> [WEBROOT]` で Nginx/Apache の vhost(HTTP→HTTPSリダイレクト、
    ACME challenge許可込み、`deploy/nginx|apache/vhost.conf.template` から
    生成、出力は `.gitignore` 対象の `deploy/generated/`)を作成し、対象
    ドメインを `deploy/generated/domains.txt` に記録する。
  - **HTTPS自動監視・自動更新**: `scripts/setup-tls.sh` で certbot による
    Let's Encrypt証明書取得、`scripts/check-tls.sh`/`check-all-tls.sh` で
    有効期限監視、`deploy/systemd/install-systemd-units.sh` で
    `aruaru-tls-renew.timer`(1日2回、`certbot renew` + webサーバーreload)と
    `aruaru-tls-monitor.timer`(1日1回、登録済み全ドメインの失効監視)を
    有効化する。実際のドメイン取得・DNSレコード登録(レジストラ操作)は
    ここでは行わない(利用者が別途実施する前提)。
- `cargo build --target wasm32-unknown-unknown` / `cargo clippy --target
  wasm32-unknown-unknown` ともに警告0件で成功(このパスで確認済み)。
  `scripts/gen-vhost.sh` は実際に実行し、生成された `.nginx.conf`/
  `.apache.conf`/`domains.txt` の中身を目視確認済み。全シェルスクリプトは
  `bash -n` で構文検証済み。
- `todo!()`/`unimplemented!()`/TODO/FIXMEマーカーは0件(実装した範囲は
  スタブなしで完結)。
- **未実施(次回パスへの持ち越し)**: `wasm-bindgen-cli` のインストールに
  時間がかかるため今回のパスでは中断し、`wasm-bindgen` による `pkg/` 生成と
  実ブラウザでの動作確認(新しいタブUI・サイト管理フォームの実クリック)は
  実施できていない。次回パスの最優先事項とする。
  `certbot`/`systemctl` を伴う実際のTLS取得・自動更新も、実サーバー環境が
  無いためスクリプトの構文・ロジック確認のみ(実運用環境での動作確認は未実施)。

## HANDOFF(直近の自動巡回ログ、上が最新)

- **2026-07-11(2回目パス)**: ユーザーからの要望「使い勝手向上・複数サイト
  (aruaru-web用/他用途用)の接続先管理を簡単に・IPアドレスからの起動・
  HTTPSの自動監視/自動設定/自動更新」を受けて実装。
  `src/lib.rs` を `dom.rs`/`graphql.rs`/`render.rs`/`profiles.rs`/`shell.rs`
  に分割し、タブ式UI(SQL実行/レジストリ集計/サイト管理)へ刷新。
  `profiles.rs` に `localStorage` ベースの接続プロファイル管理(KUSANAGIの
  サイト一覧相当)を実装。`scripts/serve.sh`(IP指定起動)、
  `scripts/gen-vhost.sh` + `deploy/{nginx,apache}/vhost.conf.template`
  (ドメイン/IP/アップストリームからvhost生成、HTTP→HTTPSリダイレクト込み)、
  `scripts/setup-tls.sh`(certbotでのLet's Encrypt取得)、
  `scripts/check-tls.sh`/`check-all-tls.sh`(有効期限監視)、
  `deploy/systemd/`(`aruaru-tls-renew.timer` = 自動更新、
  `aruaru-tls-monitor.timer` = 自動監視、`install-systemd-units.sh` で導入)
  を新設。実際のドメイン取得・DNS登録はスコープ外である旨をREADME・
  スクリプトのコメント双方に明記(ユーザーへも確認済み)。
  `cargo build`/`cargo clippy`(both `--target wasm32-unknown-unknown`)は
  警告0件。`scripts/gen-vhost.sh` の実行結果は目視確認済み。
  **次回パスがすべきこと**: (1) `wasm-bindgen-cli 0.2.126` を導入して
  `pkg/` を生成し、実Chromiumブラウザで新タブUI(サイト追加・編集・削除・
  選択→エンドポイント欄への反映)を実際にクリックして検証、(2) 可能なら
  実サーバー環境で `scripts/setup-tls.sh`/`install-systemd-units.sh` の
  実動作(certbot取得・タイマー起動)を確認、(3) README以外の8言語版
  README(English以降)は今回未更新のため、必要なら同様に更新する。

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
