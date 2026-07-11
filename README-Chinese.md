# aruaru-web

**面向 aruaru-db 的最小化 Web UI(Rust → WebAssembly,不使用框架)**

`aruaru-db`(分布式 Git-on-SQL 数据库)通过 GraphQL(`/graphql`)对外提供
`sql` 查询与 `registrySummary`(受支持数据库注册表汇总)接口,本项目是一个
标签式仪表盘,可从浏览器中实际调用这些接口并显示结果。除 SQL 执行、注册表
汇总外,还提供一个类似 KUSANAGI 站点列表的**「站点管理」标签页**,可以
注册并切换多个连接目标(供 aruaru-web 自身使用、也可供其他项目使用)。

📖 其他语言: [日本語](README-Japan.md) / [English](README-English.md) /
[中文](README-Chinese.md) / [한국어](README-Korea.md) / [Español](README-Spain.md) /
[Français](README-France.md) / [Deutsch](README-Germany.md) / [Italiano](README-Italy.md) /
[Русский](README-Russia.md) / [العربية](README-Arabic.md)

---

## 目前已实现的功能

- 对 `aruaru-graphql`(`aruaru-db/crates/aruaru-graphql`)的 `/graphql` 端点,
  通过 `fetch()` 实际发送 GraphQL 请求:
  - `sql(query: String!): QueryResultGql` —— 执行任意 SQL,并以表格形式显示
    `columns`/`rows`/`commandTag`
  - `registrySummary: RegistrySummaryGql` —— 以卡片形式显示受支持数据库
    注册表(超过150条)的汇总信息
- 当 `aruaru-server` 未启动或无法连接时,会**当场渲染与真实 schema 结构相同
  的示例数据**,并明确提示这是"离线示例数据"(该行为已在真实浏览器中验证 ——
  详见下文"验证情况")。
- **站点管理标签页**: 可以注册多个连接目标(供 aruaru-web 自身使用、也可供
  其他项目使用),包括 IP 地址/域名/子域名/端口/路径,保存到 `localStorage`
  中,一键即可切换。SQL/注册表标签页的端点输入框会自动跟随当前选中的站点。
  每张卡片都配有可在不切换当前连接的情况下进行连通性检测的**「连接测试」
  按钮**、端口号输入校验(1〜65535)、已注册站点列表的**JSON导出/导入**功能
  (用于备份、迁移到其他浏览器),以及删除前的确认对话框。
- **SQL标签页的易用性**: 最近10条**查询历史**(点击可重新载入、悬停可查看
  完整内容)、**Ctrl+Enter / Cmd+Enter 执行快捷键**、执行结果的**CSV导出**
  功能、执行期间按钮禁用、显示行数且可滚动、表头固定的结果表格。
- **HTTPS(TLS)的自动配置・自动监控・自动更新**: 通过 `scripts/gen-vhost.sh`
  生成 Nginx/Apache 的 vhost 配置(含 HTTP→HTTPS 重定向),通过
  `scripts/setup-tls.sh` 获取 Let's Encrypt(certbot)证书,并可通过
  `deploy/systemd/install-systemd-units.sh` 启用"每天2次自动更新
  (`aruaru-tls-renew.timer`)"与"每天1次的失效监控
  (`aruaru-tls-monitor.timer` → `scripts/check-all-tls.sh`)"。详见
  "HTTPS・域名/子域名的登记"一节。

## 目前尚未实现的功能(如实说明范围)

- GraphQL Mutation(创建分支、合并、注册表 crawl 等)尚未实现。
- 认证、分页、错误时的自动重试尚未实现。
- 不提供类似 Tauri 的原生应用体验(仅为浏览器中运行的 WASM)。
- **本仓库不会实际获取域名或登记 DNS 记录(在注册商处的操作)**(因为涉及
  费用及对外部服务的实际变更)。这里自动化的范围仅限于针对已获取域名的
  "生成 vhost 配置"以及"获取・监控・自动更新 TLS 证书",DNS 记录的登记本身
  需要使用者自行在注册商处完成。

## 构建方法

不使用 Node.js、npm、TypeScript,仅通过 Rust 工具链即可完成。

```bash
rustup target add wasm32-unknown-unknown        # 仅需执行一次
cargo install wasm-bindgen-cli --version 0.2.126 # 仅需执行一次(需与 Cargo.lock 中的版本保持一致)

cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg \
  target/wasm32-unknown-unknown/debug/aruaru_web.wasm

# 使用任意静态服务器提供服务并打开(以下仅为示例)
python -m http.server 8080
# 在浏览器中打开 http://localhost:8080/index.html
```

若要实际运行 `aruaru-db` 进行测试:

```bash
cd ../aruaru-db
cargo run -p aruaru-server -- --data ./data --raft-id 1   # 将在 :4000 上启动 GraphQL
```

## 从指定 IP 地址启动

```bash
scripts/serve.sh 0.0.0.0 8080        # 在所有网络接口上监听
scripts/serve.sh 192.168.1.50 8080   # 仅在指定的 IP 地址上监听
```

## HTTPS・域名/子域名的登记

本仓库本身不会获取域名或登记 DNS 记录(因为涉及在注册商处的操作及费用,
需由使用者另行完成)。以下是为了将已获取的域名/子域名便捷地分配给
aruaru-web 或其他项目使用的本地自动化流程(相当于 KUSANAGI 的"添加站点"
功能)。

```bash
# 1. 根据域名+IP+后端信息生成 vhost 配置(Nginx/Apache,含 HTTP→HTTPS 重定向)
scripts/gen-vhost.sh aruaru.example.com 203.0.113.10 127.0.0.1:4000
# 用于其他用途的子域名也可用同样方式生成(只需更改 UPSTREAM/WEBROOT)
scripts/gen-vhost.sh tool.example.com 203.0.113.10 127.0.0.1:9000 /var/www/tool

# 2. 部署生成的配置文件并重新加载(位于 deploy/generated/ 下,属于 .gitignore 对象)

# 3. 获取 TLS 证书(Let's Encrypt / certbot)
scripts/setup-tls.sh aruaru.example.com admin@example.com /var/www/aruaru.example.com

# 4. 启用自动更新(每天2次)+ 自动监控(每天1次,用于检测即将过期的证书)
sudo deploy/systemd/install-systemd-units.sh
```

可随时通过 `scripts/check-all-tls.sh` 手动确认所有已注册域名证书的有效期。
若在 aruaru-web 的 GUI 界面"站点管理"标签页中也登记相同的连接目标,即可
与浏览器端的连接切换保持一致。

## 验证情况(本次工作已完成的验证)

- `cargo check --target wasm32-unknown-unknown` / `cargo build --target
  wasm32-unknown-unknown` 均执行成功(0 条警告)。
- 通过 `wasm-bindgen --target web` 生成了 `pkg/aruaru_web.js` /
  `pkg/aruaru_web_bg.wasm`,并在真实浏览器(Chromium,经由 Playwright)中
  加载 `index.html`,实际操作并确认了以下内容: 标签切换、执行 SQL →
  渲染离线回退数据、查询历史的记录・重新载入・悬停提示、Ctrl+Enter 快捷键、
  CSV导出(实际触发下载)、注册表汇总、站点管理标签页中已注册站点的显示・
  新增・拒绝非法端口输入・连接测试按钮・JSON导出/导入(已确认往返一致)・
  删除确认对话框(取消与执行两种情况均已确认)。控制台中没有 JS 错误
  (仅有预期内的连接失败日志)。

## 目录结构

```text
aruaru-web/
├── Cargo.toml            # crate-type = ["cdylib", "rlib"],依赖 wasm-bindgen/web-sys
├── src/
│   ├── lib.rs             # 入口点・标签切换・事件绑定
│   ├── dom.rs             # DOM 操作的公共辅助函数(文件下载等)
│   ├── graphql.rs         # 对 /graphql 的 fetch 调用
│   ├── render.rs          # SQL 结果・注册表汇总的渲染、CSV 输出
│   ├── profiles.rs        # 站点管理(连接配置、localStorage 保存、JSON 导入导出)
│   ├── history.rs         # SQL 查询历史(最近10条,保存于 localStorage)
│   └── shell.rs           # HTML 外壳(标签・表单)
├── index.html             # 加载 pkg/ 的加载器 + CSS
├── pkg/                   # wasm-bindgen 生成产物(属于 .gitignore 对象,构建时重新生成)
├── scripts/
│   ├── serve.sh            # 从任意 IP 地址启动开发服务器
│   ├── gen-vhost.sh         # 根据域名/IP 生成 Nginx・Apache 的 vhost 配置
│   ├── setup-tls.sh         # 获取 Let's Encrypt 证书
│   ├── check-tls.sh         # 检查单个域名证书的有效期
│   └── check-all-tls.sh     # 批量检查所有已注册域名的有效期
├── deploy/
│   ├── nginx/vhost.conf.template
│   ├── apache/vhost.conf.template
│   ├── systemd/             # 自动更新(renew)・自动监控(monitor)定时器全套
│   └── generated/           # gen-vhost.sh 的输出(属于 .gitignore 对象)
└── CLAUDE.md
```

## 相关项目

- **aruaru-db**(本 UI 所连接的对象): https://github.com/aon-co-jp/aruaru-db
- **open-runo**: https://github.com/aon-co-jp/open-runo
- **open-web-server**: https://github.com/aon-co-jp/open-web-server
- **poem-cosmo-tauri**: https://github.com/aon-co-jp/poem-cosmo-tauri
- **open-raid-z**(开发规范的正本): https://github.com/aon-co-jp/open-raid-z
- **rs-to-readme**: https://github.com/aon-co-jp/rs-to-readme

## License

Apache-2.0
