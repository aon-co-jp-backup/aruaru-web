# 技術スタック・開発ルール(aruaru-web)

このリポジトリ、および関連プロジェクト(`open-runo`/`open-web-server`/
`aruaru-db`/`poem-cosmo-tauri`/`open-raid-z`/`rs-to-readme`)で開発・保守を
行う際は、以下を基本方針とする。作業ドライブは `F:\open-runo`(E:ドライブは
2026-07-10に消失、以後Fが実体)。この節は
[`open-raid-z`](https://github.com/aon-co-jp/open-raid-z) の `CLAUDE.md`
を正本とし、各プロジェクトへコピーして同期する。

## このリポジトリの役割・現在のステータス(2026-07-13、廃止・後継決定)

**このリポジトリは2026-07-13付けで機能開発を終了し、後継リポジトリへの
移行が完了した(廃止/deprecated)。** 理由: `aruaru-web`が「第二の
KUSANAGI」として開発していた機能は次の2系統に完全に分割され、
両方とも本リポジトリの外へ移動済みであり、**本リポジトリに残る
固有の役割が存在しない**ため。
- (1) **サイト管理・IPアドレス起動・ドメイン/HTTPS登録・HTTPS自動監視/
  自動発行/自動更新・VPSデプロイ**(=DB非依存の「簡単な運用」機能全て)
  → 新設の [`open-easyweb`](https://github.com/aon-co-jp/open-easyweb)
  へ移行済み。
- (2) **KUSANAGI風のWeb高速化機能**(gzip圧縮・静的アセットの長期
  キャッシュ・FastCGIバッファ調整・upstream keepaliveプーリング)
  → Nginx/Apache設定生成という形ではなく、[`open-runo`](https://github.com/aon-co-jp/open-runo)/
  [`poem-cosmo-tauri`](https://github.com/aon-co-jp/poem-cosmo-tauri)
  側のネイティブRust実装(hyperミドルウェア、`with_compression`/
  `with_static_cache_headers`)として統合済み。

**判断の根拠**: 分離前の`aruaru-web`の役割は「DBに依存しない第二の
KUSANAGI」の一点に尽きており、(1)(2)の分割で両方とも実体を失った
——(1)は`open-easyweb`が名称・スコープともに正確に引き継ぎ、(2)は
Nginx/Apache設定生成というアプローチ自体をやめてRustネイティブに
再実装する方針転換のため、`aruaru-web`側に「高速化はしないが他は残す」
という中間的な狭いスコープを残す合理性がない(README/CLAUDE.mdを
含む全コンテンツが両後継リポジトリに実体としてコピー済みで、
本リポジトリに残すことで得られる独自価値が無いため)。**今後、本
リポジトリへの新規機能追加は行わない**。バグ報告や既存ユーザーへの
参照目的でリポジトリ自体は残すが、開発は`open-easyweb`
(easy-ops)・`open-runo`/`poem-cosmo-tauri`(高速化)側で継続する。

---

以下は分離前(2026-07-13以前)の記録として残す。

`aruaru-web` は「**第二のKUSANAGI**」を目指す、DBに依存しない汎用の
デプロイ・運用ツールだった。WordPress高速化サーバー構築キット
「KUSANAGI」のように、アプリのアップロード後にIPアドレスから起動し、
ドメイン登録・HTTPS化を簡単に自動適用できることを目指していた。
**このリポジトリは`aruaru-db`/`open-runo`/`open-web-server` とは
別チームの並行作業対象であり、それらのディレクトリには立ち入らない**
(README/CLAUDE.mdの参照のみ可)。

**方針(2026-07-13、ユーザー確認済み・重要な方針転換)**: 2026-07-12時点では
「aruaru-db向けWeb UI」として、SQL実行・レジストリ集計というaruaru-dbへの
最小限のGraphQL接続機能を維持していたが、2026-07-13にユーザーから
「DBは一切必要ない」という明確な指示を受け、**SQL実行・GraphQL fetchを
含むDB接続機能をコードから完全に撤去**した。現在の `aruaru-web` は
特定のデータベース製品に依存しない汎用ツールであり、中核機能は:
- **サイト管理**(接続先の複数登録・切替・接続テスト・JSON入出力)
- **IPアドレス起動**(`scripts/serve.sh`)
- **vhost生成・高速化・HTTPS自動設定/監視/更新**(`scripts/gen-vhost.sh`
  他。WordPress/Laravel/FastAPI/汎用リバースプロキシの5スタック対応)
- **VPSへのデプロイ**(`scripts/deploy-vps.ps1`、Windows PowerShellから
  ビルド・アップロード・起動を自動化)

の4つ。`aruaru-db`・`open-web-server`・`open-raid-z` 系のプロジェクトとは
**併用可能**(「サイト管理」画面への登録や `gen-vhost.sh --stack=proxy` の
リバースプロキシ対象として利用できる)にしつつ、それらのDB固有機能を
このリポジトリで深追いすることはしない。詳細はHANDOFFログ参照。

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
- **ローカル作業ドライブ(`F:\open-runo`)上の各リポジトリは、常にリモート
  (GitHub)の最新コミットに追従させておくこと**(`git fetch`/`git pull`を
  こまめに実行する。ローカルにのみ存在する未コミット変更がある場合は、
  上書き前に必ず内容を確認し、必要なら `git stash` で退避してから最新化
  する — 詳細はHANDOFFログのトラブルシュート例を参照)。
- **無人自動開発(確認不要・自動デバッグ)のタイミングでは、20〜30分おきの
  スケジュール実行待ちにせず、1パス内でできる限り連続して作業を進める**
  こと。小さく検証可能な単位(1機能ごとに `cargo build --target
  wasm32-unknown-unknown` → 実ブラウザ確認 → commit → push)を保ちながら
  進める。
- **このリポジトリはエコシステム内で優先度が低い(他リポジトリが安定稼働中の
  ため)。過剰なスコープ拡大はせず、小さく実用的な増分を積み重ねる。**

## 現状(このリポジトリ固有)

- 2026-07-11 に本リポジトリを空の状態からブートストラップ(初回コミット)。
- 単一クレート構成(`Cargo.toml`、`src/` は `lib.rs`/`dom.rs`/`profiles.rs`/
  `shell.rs` の4モジュールのみ。workspaceではなく単体crate)。
  `crate-type = ["cdylib", "rlib"]`、依存は `wasm-bindgen`/
  `wasm-bindgen-futures`/`js-sys`/`web-sys`/`serde`/`serde_json` のみ。
  重量級フレームワーク無し。**GraphQL/SQL関連の依存・コードは無い**
  (2026-07-13に完全撤去、詳細はHANDOFF参照)。
- 実装済み機能(2026-07-13時点):
  - **サイト管理画面**(`src/profiles.rs`、単一画面・タブ無し): aruaru-web
    自身・WordPress・Laravel・FastAPIなど任意のバックエンドスタックの
    デプロイ先(名前/用途[self/other]/プロトコル/ホスト/ポート/パス/
    バックエンドスタック名)を複数登録・編集・削除でき、`localStorage`
    (`aruaru_web_site_profiles_v2`)に保存。カードごとに**「接続テスト」
    ボタン**(選択中のサイトを変えずに、`fetch(url, {mode: 'no-cors'})`
    による汎用HTTP到達性チェックのみ実行。GraphQL等特定プロトコルには
    依存しない)、ポート番号の入力検証(1〜65535以外は保存を拒否)、
    **削除前の確認ダイアログ**、登録済みサイト一覧の**JSONエクスポート/
    インポート**(`FileReader`、バックアップ・他ブラウザへの持ち出し用、
    インポート時も確認ダイアログ)を実装。
  - **IPアドレスからの起動**: `scripts/serve.sh <BIND_IP> <PORT>` でローカル/
    VPS上の任意のIP/ポートにbind。
  - **vhost生成・高速化・HTTPS自動設定**: `scripts/gen-vhost.sh
    [--stack=STACK] <DOMAIN> <BIND_IP> [UPSTREAM] [WEBROOT]` で、
    `static`(静的サイト。aruaru-web自身向け)・`proxy`(aruaru-db・
    open-web-server・open-raid-z系や任意のHTTPバックエンド向け汎用
    リバースプロキシ、named upstream + keepaliveで高速化)・
    `wordpress`・`laravel`(いずれもPHP-FPM + gzip/静的キャッシュ/
    FastCGIバッファ調整)・`fastapi`(ASGIサーバーへのリバースプロキシ、
    WebSocket/ストリーミング対応)の5スタックに対応した Nginx/Apache の
    vhost(HTTP→HTTPSリダイレクト、ACME challenge許可込み)を
    `deploy/{nginx,apache}/vhost-<stack>.conf.template` から生成する
    (出力は `.gitignore` 対象の `deploy/generated/`)。対象ドメインは
    `deploy/generated/domains.txt` に記録される。
  - **HTTPS自動監視・自動更新**: `scripts/setup-tls.sh` で certbot による
    Let's Encrypt証明書取得、`scripts/check-tls.sh`/`check-all-tls.sh` で
    有効期限監視、`deploy/systemd/install-systemd-units.sh` で
    `aruaru-tls-renew.timer`(1日2回、`certbot renew` + webサーバーreload)と
    `aruaru-tls-monitor.timer`(1日1回、登録済み全ドメインの失効監視)を
    有効化する。実際のドメイン取得・DNSレコード登録(レジストラ操作)は
    ここでは行わない(利用者が別途実施する前提)。
  - **VPSへのデプロイ**: `scripts/deploy-vps.ps1`(Windows PowerShell)で、
    ローカルでの `cargo build`/`wasm-bindgen` → `scp` でのVPSへの
    アップロード(`open-web-server` の同時アップロードにも対応、
    `-OpenWebServerPath` パラメータ)→ `ssh` 経由での `scripts/serve.sh`
    起動、までを自動化する。
- `cargo build --target wasm32-unknown-unknown` / `cargo clippy --target
  wasm32-unknown-unknown` ともに警告0件で成功(このパスで確認済み)。
  `todo!()`/`unimplemented!()`/TODO/FIXMEマーカーは0件。
- **今回のパスで実施した実機検証**:
  - `wasm-bindgen` → Playwright(Chromium)で、サイト管理画面のみの単一画面
    UI(タブ・SQL入力欄が完全に消えていること)、サイト新規追加・不正
    ポート入力の拒否・接続テスト(**実際に稼働中のPythonの
    `http.server`への到達性確認が成功すること**まで確認)・サイト選択・
    削除確認ダイアログ(キャンセル/実行の両方)・JSONエクスポートを実
    クリックで確認済み。console上のJSエラーは無し。
  - `scripts/gen-vhost.sh` の全5スタック(static/proxy/wordpress/laravel/
    fastapi)の生成物を、実際に導入したNginx 1.24・Apache 2.4に対して
    `nginx -t` / `apache2ctl configtest` の両方で構文検証(全てエラー0件、
    proxy/fastapiスタックのnamed upstreamも含めて検証)。static/proxy
    スタックは実起動して `curl` で機能検証(HTTP→HTTPSリダイレクト、
    静的ファイル配信、リバースプロキシ経由の502応答)まで確認済み。
  - `scripts/deploy-vps.ps1`(PowerShell)は、このコンテナにPowerShell
    ランタイムが無く、かつ実VPS環境も無いため、コード目視レビューのみで
    実行検証はできていない(次回、実際にWindows環境+VPSがあれば検証)。
  - README(ルート、英語以下9言語)は全て新しい方針(DB機能撤去・
    第二のKUSANAGI路線・5スタックのvhost高速化・VPSデプロイ)に同期済み。
    コードブロック(コマンド・パス・URL・ライセンス識別子)は各言語版でも
    翻訳せず原文のまま。

## HANDOFF(直近の自動巡回ログ、上が最新)

- **2026-07-13(11回目パス・最終パス、リポジトリ分割・廃止)**: ユーザーの
  エコシステム再編指示を受け、`aruaru-web`を2系統に分割:
  (1) DB非依存の「簡単な運用」機能(サイト管理・IPアドレス起動・
  ドメイン/HTTPS登録・HTTPS自動監視/自動発行/自動更新・VPSデプロイ)を
  新設の`open-easyweb`(https://github.com/aon-co-jp/open-easyweb)へ
  移植(`src/*.rs`・`index.html`・`scripts/*`・`deploy/systemd/*`を
  そのままブランディング変更のみでコピー、`deploy/nginx|apache/*`は
  高速化ディレクティブ——gzip・expires/Cache-Control・fastcgi_buffers・
  named upstream+keepalive——を全て削除した差分を新規作成)。
  (2) KUSANAGI風のWeb高速化機能(gzip圧縮・静的アセット長期キャッシュ・
  FastCGIバッファ調整・upstream keepaliveプーリング)は、Nginx/Apache
  設定生成というアプローチ自体をやめ、`open-runo`/`poem-cosmo-tauri`
  側のネイティブRust実装(hyperミドルウェア)として統合する方針に転換
  ——両リポジトリの`crates/open-runo-router/src/middleware_hyper.rs`に
  `with_static_cache_headers`(静的アセットへの`Cache-Control:
  public, max-age=N, immutable`付与、Nginxの`expires`/`Cache-Control`
  ディレクティブに相当)を新規実装(gzip応答圧縮は既存の
  `with_compression`が既にNginxの`gzip`ディレクティブ相当をカバー
  済みだったため実装済みとして確認のみ)。FastCGIバッファ調整・named
  upstream keepaliveプーリングはNginx固有のリバースプロキシ実装詳細
  であり、`open-runo`/`poem-cosmo-tauri`はNginxの代わりとなる
  Rustサーバー自体であってNginxの手前に立つプロキシではないため、
  移植すべき同等概念が無いと判断(hyperのkeep-alive接続プーリングは
  クライアント→このサーバー間の話であり、Nginx→上流アプリという
  構図とは対応しない)。実バイナリ+curlで
  `Cache-Control: public, max-age=2592000, immutable`が静的アセット
  (`/pkg/*.js`)にのみ付与され`/health`には付与されないことを確認済み
  (詳細は両リポジトリの同日CLAUDE.md HANDOFF参照)。
  **本リポジトリの今後の方針(ユーザーへの確認なしで判断・明記)**:
  分割後の`aruaru-web`には、DB非依存の運用機能(`open-easyweb`が
  正確に継承)ともKUSANAGI風高速化機能(`open-runo`/`poem-cosmo-tauri`
  が継承)とも異なる、固有に残すべきスコープが存在しないと判断し、
  **本リポジトリを廃止(deprecated)とする**。今後新規の機能開発は
  行わない。上記「このリポジトリの役割・現在のステータス」節に判断
  根拠を明記済み。README(ルート+全9言語)は今回未更新——廃止済み
  リポジトリのREADMEを更新する優先度は低いと判断し、CLAUDE.mdでの
  ステータス明記のみに留めた(次回何らかの理由でこのリポジトリに
  戻ってくることがあれば、READMEにも同様の廃止notice追記を検討)。
  次回パスがすべきこと: 通常は無い(廃止済みのため)。もしユーザーから
  再開の指示があれば、まずこのHANDOFFエントリと`open-easyweb`/
  `open-runo`/`poem-cosmo-tauri`側の対応するエントリを確認すること。

- **2026-07-13(10回目パス)**: 9回目パスで残っていた作業を完了。
  (1) README全10言語(English/Chinese/Korea/Spain/France/Germany/Italy/
  Russia/Arabic、日本語は9回目パスで完了済み)を「第二のKUSANAGI」新方針
  (DB機能完全撤去・5スタックvhost高速化・VPSデプロイ)に全面刷新。
  専用サブエージョン9体を並列実行(1回目は8体が長時間応答なしとなり、
  再実行して全て完了)。フランス語版で一部コード内コメント・ディレクトリ
  構成図がフランス語に翻訳されず日本語のまま残っていたのを検出し、手動で
  修正済み(他言語は問題なし、中国語・韓国語の漢字/中点記号は正常)。
  (2) ユーザーの指示によりopen-raid-z(正本)・open-runo・
  open-web-server・poem-cosmo-tauri・rs-to-readme・aruaru-dbの6リポジトリを
  このセッションに追加し、「ローカル作業ドライブ(`F:\open-runo`)を常に
  最新化する」という運用ルールをそれぞれのCLAUDE.mdに追記・push。
  ただし`aruaru-db`のみpushが403(書き込み権限なし)で失敗 — 別チームの
  並行作業により上流に新しいコミットが入っていたことも確認しており、
  「別チームの並行作業対象であり立ち入らない」という既存方針と整合的な
  結果のため、`aruaru-db`への反映は見送った(ローカルにのみコミットが
  残っているが、これはこのセッション固有の使い捨て環境でのことであり、
  実リポジトリには影響していない)。
  (3) `cargo build`/`cargo clippy`は引き続き警告0件、Playwright実ブラウザ
  検証でも新UIが正常動作することを再確認。

- **2026-07-13(9回目パス)**: ユーザーから、8回目パスよりさらに踏み込んだ
  明確な指示:「DBは一切必要ないです」「第二のKUSANAGI(アプリの
  アップロード後にIPアドレスで起動→ドメイン登録・HTTPS自動化)を中核に
  据え」「KUSANAGIの様にWordPress/PHP+Laravel/Python+FastAPIなどのWEBを
  高速化してみて」「open-web-server/aruaru-db/open-raid-zなどと併用も
  出来るようにして」。これを受け、8回目パスで残していた
  `sql`/`registrySummary` へのGraphQL fetch(SQL実行・レジストリ集計)を
  含む**DB接続機能を完全に撤去**し、DBに依存しない汎用デプロイツールへ
  刷新した:
  - `src/graphql.rs`・`render.rs`・`history.rs` を削除(GraphQL fetch・
    結果レンダリング・SQLクエリ履歴のロジックを全廃)。
  - `src/profiles.rs`: 「接続テスト」をGraphQLクエリから
    `fetch(url, {mode: 'no-cors'})` による汎用HTTP到達性チェックに変更
    (任意の外部サイトはCORSヘッダを持たないことが多いため)。
    `purpose` の値を `aruaru-web`/`other` から `self`/`other` に整理し、
    ストレージキーを `_v1` → `_v2` に変更(スキーマ変更のため)。
  - `src/shell.rs`・`lib.rs`: タブ切り替えUI(SQL実行/レジストリ集計/
    サイト管理)を廃止し、サイト管理を唯一の単一画面に統合。
  - `Cargo.toml`・`index.html`: 不要になった `HtmlTextAreaElement`/
    `KeyboardEvent`/`CssStyleDeclaration`/`DomTokenList` 等のweb-sys
    機能とタブ/テーブル用CSSを削減。
  - **vhost高速化(KUSANAGI風)**: `deploy/{nginx,apache}/` の汎用
    `vhost.conf.template` を、`vhost-{static,proxy,wordpress,laravel,
    fastapi}.conf.template` の5スタックに分割・新設。`static`=静的サイト、
    `proxy`=aruaru-db/open-web-server/open-raid-z系や任意バックエンド向け
    汎用リバースプロキシ(named upstream + keepaliveで高速化)、
    `wordpress`/`laravel`=PHP-FPM + gzip/静的キャッシュ/FastCGIバッファ
    調整、`fastapi`=ASGIリバースプロキシ(WebSocket対応)。
    `scripts/gen-vhost.sh` に `--stack=STACK` オプションを追加し、
    ドメインごとに一意な `UPSTREAM_NAME`(nginxのnamed upstream衝突回避)を
    自動生成するようにした。
  - **VPSデプロイ**: `scripts/deploy-vps.ps1`(Windows PowerShell)を新設。
    ローカルビルド→`scp`でのVPSアップロード(`open-web-server`の同時
    アップロードにも対応)→`ssh`経由での起動、を自動化。
  - README(全10言語)・CLAUDE.mdを新方針に合わせて全面改訂。「関連
    プロジェクト」節に、aruaru-db/open-web-server/open-raid-z等とは
    「併用可能」(サイト管理画面への登録や`--stack=proxy`でのリバース
    プロキシ対象として利用できる)である旨を明記。
  - `cargo build`/`cargo clippy`(`--target wasm32-unknown-unknown`)は
    警告0件。Playwright(Chromium)で、サイト管理単一画面への刷新・
    実際に稼働中のHTTPサーバーへの接続テスト成功・追加/削除/検証/
    エクスポートが正常動作することを確認。`gen-vhost.sh`の全5スタックは
    実際に導入したNginx 1.24・Apache 2.4で`nginx -t`/`apache2ctl
    configtest`の両方で構文検証(エラー0件)、static/proxyスタックは
    実起動してcurl機能検証まで実施。
  **今後の方針**: DB(特定のデータベース製品)への機能拡張は今後も
  行わない。増分は「第二のKUSANAGI」路線(サイト管理・IPアドレス起動・
  vhost高速化・HTTPS自動化・VPSデプロイ)の完成度向上に集中する。
  **次回パスがすべきこと**: (1) 実際のWindows環境+VPSがあれば
  `scripts/deploy-vps.ps1` の実行検証、(2) 実際にパブリックドメイン+
  外部到達可能なサーバー環境があれば `scripts/setup-tls.sh` での実際の
  Let's Encrypt発行を確認、(3) wordpress/laravel/fastapiスタックについても
  実際のPHP-FPM/ASGIサーバーを用意できれば、static/proxyスタックと同様に
  実起動してのcurl機能検証を行う。

- **2026-07-12(8回目パス)**: ユーザーから明確な方針転換の指示:
  「第二のKUSANAGIで、このアプリをUPLOAD後にIPアドレスで起動して簡単に
  ドメイン登録とHTTPSを自動適用。この後、DB対応は意図してません。」
  これを受け、6回目パスで追加した「バージョン管理タブ」(VcsQuery:
  branches/currentBranch/log/diff)と「レジストリ集計タブ」の
  「登録DB一覧を取得」(registry クエリ)を**撤去**。
  - `src/shell.rs`: `tab-vcs` セクション・`run-registry-list` ボタンを削除。
  - `src/lib.rs`: 該当ボタンの配線・`on_run_branches`/`on_run_log`/
    `on_run_diff`/`on_run_registry_list` ハンドラ・`show_tab` のタブ一覧
    エントリを削除。
  - `src/graphql.rs`: `BRANCHES_QUERY`/`LOG_QUERY`/`DIFF_QUERY`/
    `REGISTRY_LIST_QUERY` を削除(`SQL_QUERY`/`REGISTRY_SUMMARY_QUERY`は
    維持)。
  - `src/render.rs`: `render_branches`/`render_log`/`render_diff`/
    `render_registry_list` とそれぞれのオフラインサンプル関数、
    `str_field` ヘルパーを削除。共通テーブル描画ヘルパー `render_table`
    自体は `render_query_result`(SQLタブ)の実装簡素化に資するため
    そのまま維持。
  - README(ルート+全9言語)から「バージョン管理タブ」「registry一覧」の
    記載を除去し、代わりに「aruaru-webは"第二のKUSANAGI"(アプリの
    アップロード後にIPアドレスから起動し、ドメイン登録・HTTPS化を簡単に
    自動適用できる運用ツール)を目指し、aruaru-db側のDB機能を深掘りする
    方向(VcsQuery拡張・レジストリ詳細一覧等)は意図的に対象外とする」旨を
    「いまできないこと」節に明記(専用サブエージェント9体を並列実行して
    全言語版に反映)。冒頭の紹介文・「このリポジトリの役割」節にも同じ
    方針を追記。
  - `cargo build`/`cargo clippy`(`--target wasm32-unknown-unknown`)は
    警告0件。`wasm-bindgen` → Playwright(Chromium)で、バージョン管理タブ・
    登録DB一覧ボタンが実際に消えていること、SQL実行・レジストリ集計・
    サイト管理の既存機能が引き続き正常動作することを実クリックで確認済み。
  - `aruaru-db`(読み取り専用参照、`/workspace/aruaru-db`)への変更は
    一切無し。
  **今後の方針**: aruaru-db側のクエリ機能拡張(VcsQuery/AdminQueryの
  深掘り)は行わない。今後の増分は「第二のKUSANAGI」路線
  (IPアドレス起動・vhost/ドメイン登録・HTTPS自動設定/監視/更新・
  サイト管理・SQL/レジストリ集計UIの使いやすさ)に集中する。

- **2026-07-11(7回目パス)**: 「適材適所で完成度を高めて」という要望を受け、
  前回パス(6回目)でルートREADME.md/README-Japan.mdにのみ追記した
  「バージョン管理タブ」「registry(DB一覧)」の記載が、他8言語版READMEに
  反映されていなかった(同期漏れ)ことを検出し、専用サブエージェント9体の
  並列実行で全言語版を最新のルートREADMEに再同期。`cargo build`/
  `cargo clippy` は引き続き警告0件。`aruaru-db`(読み取り専用参照)への
  変更が無いことも確認済み。
  **次回パスがすべきこと**: (1) 実際にパブリックドメイン+外部到達可能な
  サーバー環境がある場合、`scripts/setup-tls.sh` での実際のLet's
  Encrypt発行を確認、(2) 価値があれば、ユーザー確認の上でバックアップ/
  クラスタ状態など他のAdminQueryの追加UIを検討(同様に実スキーマ確認必須)、
  (3) README更新時は10言語全てを同一パス内で同期させ、今回のような
  部分同期漏れを防ぐこと。

- **2026-07-11(6回目パス)**: 前回パスのもう一つの持ち越し事項「他の
  VcsQueryクエリ、またはregistry(DB一覧)クエリの追加UI」に対応。
  推測でのフィールド追加を避けるため、ユーザーに確認の上で `aruaru-db`
  リポジトリをこのセッションに読み取り専用で追加(`add_repo` →
  `/workspace/aruaru-db`、**このリポジトリへの変更は一切行っていない**)、
  `crates/aruaru-graphql/src/lib.rs`(`VcsQuery`)・`admin_resolvers.rs`/
  `admin_types.rs`(`AdminQuery::registry`)の実ソースを直接確認して
  実装。新設: 「バージョン管理」タブ(`branches`/`currentBranch`・
  `log(limit)`・`diff(from, to)`)、「レジストリ集計」タブへの
  「登録DB一覧を取得」ボタン(`registry` クエリ、CSVエクスポート対応)。
  `render.rs` に共通テーブル描画ヘルパー `render_table` を追加し
  `render_query_result` 含む全テーブル系表示で共用(重複削減、CSV
  エクスポートが自動的に全テーブルで使えるようになった)。
  `cargo build`/`cargo clippy`(`--target wasm32-unknown-unknown`)は
  警告0件。`wasm-bindgen` → Playwright(Chromium)で新タブの全ボタン
  (ブランチ一覧・ログ・Diff・登録DB一覧)を実クリックし、オフライン
  フォールバックの正常動作・Diffのfrom/to未入力時のバリデーションまで
  確認済み。GraphQL Mutation(`createBranch`/`checkout`/`merge`等)や
  バックアップ・クラスタ・マイグレーション・並列実行・フェデレーション系の
  AdminQuery/AdminMutationは今回スコープ外(過剰なスコープ拡大を避けるため、
  ユーザーから要望のあった範囲=VcsQuery抜粋+registry一覧のみに限定)。
  **次回パスがすべきこと**: (1) 実際にパブリックドメイン+外部到達可能な
  サーバー環境がある場合、`scripts/setup-tls.sh` での実際のLet's
  Encrypt発行を確認、(2) 価値があれば、ユーザー確認の上でバックアップ/
  クラスタ状態など他のAdminQueryの追加UIを検討(同様に実スキーマ確認必須)。

- **2026-07-11(5回目パス)**: 前回パスの持ち越し事項「実サーバー環境で
  scripts/setup-tls.sh・install-systemd-units.sh の実動作を確認」に対応。
  このセッションのコンテナに Nginx 1.24・Apache 2.4・certbot を実際に
  導入し、`scripts/gen-vhost.sh` の生成物を自己署名証明書で `nginx -t`/
  `apache2ctl configtest` → 実起動 → `curl` での機能検証(HTTP→HTTPS
  リダイレクト、ACME challengeパス、`/graphql`リバースプロキシ)まで実施。
  **この検証で `deploy/nginx/vhost.conf.template` の実バグを発見・修正**
  (`http2 on;` がNginx 1.24で `unknown directive` エラーになるため、
  旧来互換の `listen ... ssl http2;` に修正)。`scripts/check-tls.sh` も
  実際に起動したHTTPSサーバーに対してWARN/healthy/ERRORの3状態を実行
  確認。`deploy/systemd/*` は `systemd-analyze verify` でエラー0件。
  一方、実際の certbot による Let's Encrypt 発行(ACME HTTP-01)は、
  (1) パブリックドメイン・外部到達可能なIPがこのセッションには無い、
  (2) このコンテナの `/usr/bin/python3` と apt版 `python3-cffi` の
  ABI不一致で apt版certbotバイナリ自体が起動できない、という2つの
  コンテナ固有の制約により検証できなかった(本リポジトリのスクリプトの
  不具合ではない)。詳細は上記「現状」参照。
  **次回パスがすべきこと**: (1) 実際にパブリックドメイン+外部到達可能な
  サーバー環境がある場合、`scripts/setup-tls.sh` での実際のLet's
  Encrypt発行を確認、(2) 価値があれば `branches`/`log`/`diff` など他の
  VcsQueryクエリ、または `registry`(DB一覧)クエリの追加UIを検討(ただし
  aruaru-db側の実スキーマを確認できる場合のみ実装し、推測でのフィールド
  追加は避ける — このセッションでは `aruaru-db` リポジトリへのアクセスが
  スコープ外のため、ユーザーに実施可否を確認すること)。

- **2026-07-11(4回目パス)**: 「より使いやすさ・完成度・実用性の見直し」
  という要望を受けて実装。(1) 使いやすさ/実用性の追加改善: `dom.rs` に
  ファイルダウンロード共通ヘルパー(`trigger_download`)を追加しCSV/JSON
  エクスポートで共用、サイト削除に確認ダイアログ(`window.confirm`)を追加、
  登録済みサイト一覧の**JSONエクスポート/インポート**(`profiles.rs`、
  `FileReader`経由、インポート時も確認ダイアログ)を追加、クエリ履歴項目に
  ホバー時ツールチップ(`title`属性)を追加、`#status` に
  `aria-live="polite"` を付与(スクリーンリーダー対応)。
  (2) 全機能をPlaywright(Chromium)で実ブラウザ再検証(上記「現状」参照、
  削除確認のキャンセル/実行・JSONエクスポート→インポートのラウンド
  トリップまで確認)。
  (3) 8言語版README(English/Chinese/Korea/Spain/France/Germany/Italy/
  Russia/Arabic)を専用サブエージェント9体の並列実行で、現在のルート
  README.md(日本語)の内容に完全同期(サイト管理タブ・IPアドレス起動・
  HTTPS自動化・SQLクエリ履歴/CSV/ショートカットの記載を反映)。
  `cargo build`/`cargo clippy`(`--target wasm32-unknown-unknown`)は
  警告0件。
  **次回パスがすべきこと**: (1) 実サーバー環境で `scripts/setup-tls.sh`/
  `install-systemd-units.sh` の実動作(certbot取得・タイマー起動)を確認、
  (2) 価値があれば `branches`/`log`/`diff` など他のVcsQueryクエリ、または
  `registry`(DB一覧)クエリの追加UIを検討(ただしaruaru-db側の実スキーマを
  確認できる場合のみ実装し、推測でのフィールド追加は避ける)。

- **2026-07-11(3回目パス)**: 「実用性・完成度・使いやすさをさらに向上」
  という要望を受け、(1) 実ブラウザ検証: 前回パスで持ち越していた
  `wasm-bindgen-cli` 導入 → `pkg/` 生成 → Playwright(Chromium)での実クリック
  検証を完了(上記「現状」参照、全機能が期待通り動作しconsoleエラー無し)。
  (2) 使いやすさ向上の実装: `src/history.rs` を新設しSQLクエリ履歴
  (直近10件、`localStorage`、クリックで再読込)を追加、`render.rs` に
  CSVエクスポート(`Blob`/`Url`/`<a download>`)と結果テーブルの行数表示・
  スクロール・ヘッダー固定を追加、`lib.rs` にCtrl+Enter/Cmd+Enter実行
  ショートカットとボタンの実行中無効化を追加、`profiles.rs` にサイトカード
  ごとの「接続テスト」ボタン(アクティブなサイトを変えずに疎通確認)と
  ポート番号の入力検証(1〜65535外は保存拒否)を追加。
  `cargo build`/`cargo clippy`(`--target wasm32-unknown-unknown`)は
  警告0件。
  **次回パスがすべきこと**: (1) 実サーバー環境で `scripts/setup-tls.sh`/
  `install-systemd-units.sh` の実動作(certbot取得・タイマー起動)を確認、
  (2) README以外の8言語版README(English以降)は今回未更新のため、
  必要なら同様に更新する、(3) 価値があれば `branches`/`log`/`diff` など
  他のVcsQueryクエリ、または `registry`(DB一覧)クエリの追加UIを検討。

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
