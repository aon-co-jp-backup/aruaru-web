//! アプリのHTMLシェル(タブ切り替え式ダッシュボード)。

pub const SHELL_HTML: &str = r#"
<header class="app-header">
  <h1>aruaru-web</h1>
  <p class="muted">aruaru-db 用の管理/クエリダッシュボード(Rust &rarr; WebAssembly、フレームワーク不使用)。</p>
  <p class="muted">現在の接続先: <strong id="active-site-name">(未設定)</strong></p>
</header>

<nav class="tabs">
  <button class="tab-btn" data-tab="sql">SQL実行</button>
  <button class="tab-btn" data-tab="registry">レジストリ集計</button>
  <button class="tab-btn" data-tab="sites">サイト管理</button>
</nav>

<section id="tab-sql" class="tab-panel">
  <section>
    <label for="endpoint">接続先エンドポイント(「サイト管理」タブで登録した内容が反映されます)</label>
    <input id="endpoint" type="text" />
  </section>
  <section>
    <label for="sql-input">SQL</label>
    <textarea id="sql-input" rows="4"></textarea>
    <p class="muted hint">Ctrl+Enter(Macは&#8984;+Enter)でも実行できます。</p>
    <div class="buttons">
      <button id="run-sql">SQLを実行</button>
      <button id="export-csv" class="secondary" disabled>CSVでエクスポート</button>
    </div>
  </section>
  <section>
    <h2>クエリ履歴(直近10件)</h2>
    <div id="sql-history"></div>
  </section>
</section>

<section id="tab-registry" class="tab-panel">
  <section>
    <p class="muted">対応DBレジストリ(150件超)の集計をカード表示します。</p>
    <div class="buttons">
      <button id="run-registry">レジストリ集計を取得</button>
    </div>
  </section>
</section>

<section id="tab-sites" class="tab-panel">
  <section>
    <h2>登録済みサイト</h2>
    <p class="muted">
      aruaru-web用・他プロジェクト用など、複数の接続先(IPアドレス・ドメイン・
      サブドメイン)をここで登録し、ワンクリックで切り替えられます。実際の
      ドメイン取得・DNS登録(レジストラでのAレコード設定など)はここでは
      行いません。ローカル/サーバー側のリバースプロキシ設定を簡単に作る
      には <code>scripts/gen-vhost.sh</code> を使ってください。
    </p>
    <div id="site-list"></div>
  </section>
  <section>
    <h2>サイトを追加・編集</h2>
    <input id="site-form-id" type="hidden" value="" />
    <div class="form-grid">
      <div>
        <label for="site-name">サイト名</label>
        <input id="site-name" type="text" placeholder="例: 本番aruaru-db" />
      </div>
      <div>
        <label for="site-purpose">用途</label>
        <select id="site-purpose">
          <option value="aruaru-web">aruaru-web用</option>
          <option value="other">他の用途</option>
        </select>
      </div>
      <div>
        <label for="site-protocol">プロトコル</label>
        <select id="site-protocol">
          <option value="http">http</option>
          <option value="https">https</option>
        </select>
      </div>
      <div>
        <label for="site-host">ホスト(IPアドレス / ドメイン / サブドメイン)</label>
        <input id="site-host" type="text" placeholder="例: 203.0.113.10 または api.example.com" />
      </div>
      <div>
        <label for="site-port">ポート</label>
        <input id="site-port" type="text" placeholder="4000" value="8080" />
      </div>
      <div>
        <label for="site-path">パス</label>
        <input id="site-path" type="text" placeholder="/graphql" value="/graphql" />
      </div>
      <div class="form-grid-full">
        <label for="site-stack">バックエンドスタック(自由記述・任意)</label>
        <input id="site-stack" type="text" placeholder="例: Rust + Poem / PHP + Laravel / Python + FastAPI" />
      </div>
    </div>
    <div class="buttons">
      <button id="save-site">保存</button>
      <button id="clear-site-form" class="secondary">クリア</button>
    </div>
  </section>
</section>

<p id="status" class="muted"></p>
<section id="result"></section>
"#;
