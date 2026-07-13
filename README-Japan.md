# aruaru-web

**「第二のKUSANAGI」— アプリのアップロード後にIPアドレスで起動し、
ドメイン登録・HTTPS化を簡単に自動適用できる運用ツール(Rust →
WebAssembly、フレームワーク不使用)**

WordPress高速化サーバー構築キット「KUSANAGI」のように、アプリを
アップロードしたら**IPアドレスから起動 → ドメイン登録の簡易化 →
HTTPS自動化**までを一気通貫でこなすことを目指す運用ツールです。
WordPress・PHP + Laravel・Python + FastAPIなど任意のバックエンドスタックの
Webサイトを高速化するリバースプロキシ設定(Nginx/Apache)を自動生成でき、
複数サイトの接続先を登録・切替・疎通確認できる「サイト管理」画面を持ちます。
**DB(データベース)への接続機能は持ちません**(意図的にスコープ外)。

📖 他の言語: [日本語](README-Japan.md) / [English](README-English.md) /
[中文](README-Chinese.md) / [한국어](README-Korea.md) / [Español](README-Spain.md) /
[Français](README-France.md) / [Deutsch](README-Germany.md) / [Italiano](README-Italy.md) /
[Русский](README-Russia.md) / [العربية](README-Arabic.md)

---

## いまできること

- **サイト管理画面**: aruaru-web自身・WordPress・Laravel・FastAPIなど
  任意のバックエンドスタックのデプロイ先(IPアドレス/ドメイン/サブドメイン/
  ポート/パス)を複数登録し、`localStorage` に保存してワンクリックで選択・
  疎通確認できる(KUSANAGIのサイト一覧に相当)。カードごとに**「接続テスト」
  ボタン**(選択中のサイトを変えずに単純なHTTP到達性確認のみ実行)、ポート
  番号の入力検証(1〜65535)、登録済みサイト一覧の**JSONエクスポート/
  インポート**(バックアップ・他ブラウザへの持ち出し用)、削除前の確認
  ダイアログを備える。
- **IPアドレスから起動**: `scripts/serve.sh` でローカル/VPS上の任意のIP・
  ポートにbindして配信できる。
- **vhost生成・高速化・HTTPS自動設定**: `scripts/gen-vhost.sh` で、
  ドメイン・IP・バックエンドスタックの組み合わせから Nginx/Apache の
  vhost(HTTP→HTTPSリダイレクト込み)を生成する。`static`(静的サイト)・
  `proxy`(aruaru-db・open-web-server・open-raid-z系や任意のHTTPバックエンド
  向け汎用リバースプロキシ)・`wordpress`・`laravel`・`fastapi` の5スタックに
  対応し、gzip圧縮・静的アセットの長期キャッシュ・upstream keepalive・
  FastCGIバッファ調整など、スタックに応じた高速化設定を含む。
- **HTTPS(TLS)の自動監視・自動更新**: `scripts/setup-tls.sh` で
  Let's Encrypt(certbot)の証明書取得、`deploy/systemd/
  install-systemd-units.sh` で「1日2回の自動更新
  (`aruaru-tls-renew.timer`)」と「1日1回の失効監視
  (`aruaru-tls-monitor.timer` → `scripts/check-all-tls.sh`)」を有効化できる。
- **VPSへのデプロイ**: Windows PowerShellから `scripts/deploy-vps.ps1` を
  実行するだけで、ビルド → VPSへのアップロード → 起動までを自動化できる
  (詳細は下記「VPSへのデプロイ」参照)。

## いまできないこと(正直な範囲)

- **DB(データベース)への接続機能は持たない**。SQL実行・GraphQLクエリなど
  特定のデータベース製品に依存する機能は意図的にスコープ外であり、今後も
  実装しない。aruaru-dbのような特定バックエンドを使う場合も、
  「サイト管理」画面やvhost生成(`--stack=proxy`)は**汎用のリバース
  プロキシ・デプロイ管理**として利用できるが、そのDBに固有のクエリ機能は
  提供しない。
- 認証・ページネーション・エラー時の自動リトライは未実装。
- Tauriのようなネイティブアプリ体験は提供しない(ブラウザで動くWASMのみ)。
- **実際のドメイン取得・DNSレコード登録(レジストラでの操作)はこのリポジトリ
  からは行わない**(費用・外部サービスへの反映を伴うため)。ここで自動化して
  いるのは、取得済みドメインに対する「vhost設定生成」「TLS証明書の取得・
  監視・自動更新」までであり、DNS登録自体は利用者がレジストラで行う。
- 実際のVPS契約(レンタルサーバー事業者との契約)もこのリポジトリからは
  行わない。

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

## IPアドレスから起動する

```bash
scripts/serve.sh 0.0.0.0 8080        # 全インターフェースで待受
scripts/serve.sh 192.168.1.50 8080   # 特定のIPアドレスのみで待受
```

起動後、**ブラウザのURL欄に直接IPアドレスを入力**すれば確認できる
(例: `http://192.168.1.50:8080/index.html`)。ドメイン登録前でも
IPアドレスだけで動作確認できるのがポイント。

## VPSへのデプロイ(Windows PowerShellから)

VPSレンタルサーバーを借りたあと、Windows PowerShellから
`scripts/deploy-vps.ps1` を実行するだけで、ビルド→アップロード→起動まで
自動化できる。`open-web-server` を併用する場合は同時にアップロードする
こともできる(このリポジトリからは `open-web-server` の中身には立ち入らず、
アップロード先パスを指定するだけ)。

```powershell
# アップロードして起動まで行う場合(aruaru-webのみ)
.\scripts\deploy-vps.ps1 -VpsHost 203.0.113.10 -VpsUser root -StartServer

# open-web-server (F:\open-runo\open-web-server) も同時にアップロードする場合
.\scripts\deploy-vps.ps1 -VpsHost 203.0.113.10 -VpsUser root -StartServer `
    -OpenWebServerPath "F:\open-runo\open-web-server"
```

これは内部で以下と同等の処理を行う(手動で実行してもよい):

```powershell
# 1. ローカルでビルド
cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg `
    target/wasm32-unknown-unknown/debug/aruaru_web.wasm

# 2. VPSへアップロード(OpenSSH клиент、Windows 10 1809以降/11に標準搭載)
ssh root@203.0.113.10 "mkdir -p /root/aruaru-web"
scp -r .\index.html .\pkg .\scripts .\deploy .\Cargo.toml .\src `
    root@203.0.113.10:/root/aruaru-web/

# 3. VPS上でIPアドレスから起動
ssh root@203.0.113.10 "cd /root/aruaru-web && bash scripts/serve.sh 0.0.0.0 8080"
```

起動後、**ブラウザのURL欄にVPSのIPアドレスを入力**する
(例: `http://203.0.113.10:8080/index.html`)。ローカルの
`F:\open-runo\aruaru-web` 等を常に最新化したい場合は、このリポジトリを
`git pull` するだけでよい:

```powershell
cd F:\open-runo\aruaru-web
git fetch origin
git pull origin <ブランチ名>
```

**アップロードしなくても使える**: VPSを使わずローカルだけで試す場合は、
上記「IPアドレスから起動する」の通り `scripts/serve.sh` をそのままローカルで
実行すればよい。同じLAN内の他端末からは `http://<ローカルPCのIPアドレス>:8080/`
でアクセスできる。

## vhost生成・高速化・ドメイン/サブドメインの登録

このリポジトリ自体はドメイン取得やDNSレコード登録は行わない(レジストラでの
操作・費用が発生するため利用者が別途実施する)。以下は、取得済みのドメイン/
サブドメインに対して、高速化設定込みのリバースプロキシをスタック別に
簡単に払い出すためのローカル自動化(KUSANAGIの「サイト追加」相当)。

```bash
# aruaru-web自身(静的サイト)
scripts/gen-vhost.sh --stack=static aruaru.example.com 203.0.113.10

# aruaru-db・open-web-server・open-raid-z系や任意のバックエンドへの汎用リバースプロキシ
scripts/gen-vhost.sh --stack=proxy tool.example.com 203.0.113.10 127.0.0.1:9000

# WordPress(PHP-FPMソケット/アドレスを指定)
scripts/gen-vhost.sh --stack=wordpress blog.example.com 203.0.113.10 \
  unix:/run/php/php8.3-fpm.sock /var/www/blog

# Laravel(publicディレクトリを明示)
scripts/gen-vhost.sh --stack=laravel app.example.com 203.0.113.10 \
  unix:/run/php/php8.3-fpm.sock /var/www/app/public

# FastAPI(ASGIサーバーへのリバースプロキシ、WebSocket/ストリーミング対応)
scripts/gen-vhost.sh --stack=fastapi api.example.com 203.0.113.10 127.0.0.1:8000
```

生成された設定ファイル(`deploy/generated/` 以下、`.gitignore` 対象)を
Nginx/Apacheの設定ディレクトリに配置してリロードした後、証明書を取得する:

```bash
scripts/setup-tls.sh aruaru.example.com admin@example.com /var/www/aruaru.example.com

# 自動更新(1日2回)+ 自動監視(1日1回、失効間近を検知)を有効化
sudo deploy/systemd/install-systemd-units.sh
```

登録済みの全ドメインの証明書有効期限は `scripts/check-all-tls.sh` でいつでも
手動確認できる。aruaru-web の GUI 側「サイト管理」画面にも同じ接続先を
登録しておくと、ブラウザ側から一覧・接続テストができる。

## 動作確認(このパスで実施)

- `cargo check --target wasm32-unknown-unknown` / `cargo build --target wasm32-unknown-unknown`
  ともに成功(警告0件)。
- `wasm-bindgen --target web` で `pkg/aruaru_web.js` / `pkg/aruaru_web_bg.wasm` を生成し、
  実ブラウザ(Chromium、Playwright経由)で `index.html` を読み込み、以下を実際に
  操作して確認済み: サイト管理画面での登録済みサイト表示・新規追加・不正
  ポート入力の拒否・接続テストボタン(実際に稼働中のHTTPサーバーへの到達性
  確認まで成功)・削除確認ダイアログ(キャンセル/実行の両方)・JSONエクス
  ポート。console上のJSエラーは無し。
- Nginx 1.24(Ubuntu 24.04標準)・Apache 2.4を実際に導入し、`gen-vhost.sh`の
  全5スタック(static/proxy/wordpress/laravel/fastapi)の生成物を自己署名
  証明書で `nginx -t` / `apache2ctl configtest` の両方で構文検証、static/proxy
  スタックは実起動して `curl` で機能検証(HTTP→HTTPSリダイレクト、静的配信、
  リバースプロキシ経由の502応答)まで確認済み。
- 実際のcertbotによるLet's Encrypt発行(ACME認証)、および
  `scripts/deploy-vps.ps1` の実VPS環境での動作は、パブリックドメイン・
  実VPS・Windows環境がこのセッションに無いため未検証(詳細はCLAUDE.md参照)。

## 構成

```text
aruaru-web/
├── Cargo.toml            # crate-type = ["cdylib", "rlib"]、wasm-bindgen/web-sys依存
├── src/
│   ├── lib.rs             # エントリポイント・イベント配線
│   ├── dom.rs             # DOM操作の共通ヘルパー(ファイルダウンロード等)
│   ├── profiles.rs        # サイト管理(接続プロファイル、localStorage保存、接続テスト、JSON入出力)
│   └── shell.rs           # HTMLシェル
├── index.html             # pkg/ を読み込むローダー + CSS
├── pkg/                   # wasm-bindgen生成物(.gitignore対象、ビルドで再生成)
├── scripts/
│   ├── serve.sh            # 任意のIPアドレスから配信する開発サーバー起動
│   ├── deploy-vps.ps1       # Windows PowerShellからVPSへビルド・アップロード・起動
│   ├── gen-vhost.sh         # ドメイン/IP/スタックからNginx・Apache vhostを生成
│   ├── setup-tls.sh         # Let's Encrypt証明書の取得
│   ├── check-tls.sh         # 1ドメインの証明書有効期限チェック
│   └── check-all-tls.sh     # 登録済み全ドメインの有効期限を一括チェック
├── deploy/
│   ├── nginx/vhost-{static,proxy,wordpress,laravel,fastapi}.conf.template
│   ├── apache/vhost-{static,proxy,wordpress,laravel,fastapi}.conf.template
│   ├── systemd/             # 自動更新(renew)・自動監視(monitor)タイマー一式
│   └── generated/           # gen-vhost.shの出力(.gitignore対象)
└── CLAUDE.md
```

## 関連プロジェクト

このリポジトリはDBに依存しない汎用のデプロイ・運用ツールであるため、
以下のような他プロジェクトとも**併用可能**(「サイト管理」画面への登録や
`--stack=proxy` でのリバースプロキシ対象として利用できる。それぞれの
リポジトリの中身には立ち入らない):

- **aruaru-db**: https://github.com/aon-co-jp/aruaru-db
- **open-runo**: https://github.com/aon-co-jp/open-runo
- **open-web-server**: https://github.com/aon-co-jp/open-web-server
- **poem-cosmo-tauri**: https://github.com/aon-co-jp/poem-cosmo-tauri
- **open-raid-z**(開発ルールの正本): https://github.com/aon-co-jp/open-raid-z
- **rs-to-readme**: https://github.com/aon-co-jp/rs-to-readme

## License

Apache-2.0
