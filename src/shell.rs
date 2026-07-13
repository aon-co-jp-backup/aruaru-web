//! アプリのHTMLシェル(サイト管理画面)。

pub const SHELL_HTML: &str = r#"
<header class="app-header">
  <h1>aruaru-web</h1>
  <p class="muted">
    「第二のKUSANAGI」— アプリのアップロード後にIPアドレスから起動し、
    ドメイン登録・HTTPS化を簡単に自動適用できる運用ツール(Rust &rarr;
    WebAssembly、フレームワーク不使用)。
  </p>
  <p class="muted">選択中のサイト: <strong id="active-site-name">(未設定)</strong></p>
</header>

<section>
  <h2>登録済みサイト</h2>
  <p class="muted">
    aruaru-web自身の管理画面や、WordPress・Laravel・FastAPIなど任意の
    バックエンドスタックのデプロイ先(IPアドレス・ドメイン・サブドメイン)を
    ここで登録し、ワンクリックで選択・疎通確認できます。実際のドメイン取得・
    DNS登録(レジストラでのAレコード設定など)はここでは行いません。
    サーバー側のリバースプロキシ設定(高速化設定込み)を簡単に作るには
    <code>scripts/gen-vhost.sh</code> を使ってください。
  </p>
  <div id="site-list"></div>
  <div class="buttons">
    <button id="site-export" class="secondary">エクスポート(JSON)</button>
    <button id="site-import-trigger" class="secondary">インポート(JSON)</button>
    <input id="site-import-file" type="file" accept="application/json" style="display:none" />
  </div>
</section>

<section>
  <h2>サイトを追加・編集</h2>
  <input id="site-form-id" type="hidden" value="" />
  <div class="form-grid">
    <div>
      <label for="site-name">サイト名</label>
      <input id="site-name" type="text" placeholder="例: 本番WordPress" />
    </div>
    <div>
      <label for="site-purpose">用途</label>
      <select id="site-purpose">
        <option value="self">このサイト(aruaru-web自身)</option>
        <option value="other">他のサイト</option>
      </select>
    </div>
    <div>
      <label for="site-protocol">プロトコル</label>
      <select id="site-protocol">
        <option value="https">https</option>
        <option value="http">http</option>
      </select>
    </div>
    <div>
      <label for="site-host">ホスト(IPアドレス / ドメイン / サブドメイン)</label>
      <input id="site-host" type="text" placeholder="例: 203.0.113.10 または example.com" />
    </div>
    <div>
      <label for="site-port">ポート</label>
      <input id="site-port" type="text" placeholder="443" value="443" />
    </div>
    <div>
      <label for="site-path">パス</label>
      <input id="site-path" type="text" placeholder="/" value="/" />
    </div>
    <div class="form-grid-full">
      <label for="site-stack">バックエンドスタック(自由記述・任意)</label>
      <input id="site-stack" type="text" placeholder="例: WordPress / PHP + Laravel / Python + FastAPI" />
    </div>
  </div>
  <div class="buttons">
    <button id="save-site">保存</button>
    <button id="clear-site-form" class="secondary">クリア</button>
  </div>
</section>

<p id="status" class="muted" aria-live="polite"></p>
"#;
