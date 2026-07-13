# aruaru-web

**「第二个 KUSANAGI」—— 应用上传后即可从 IP 地址启动,并能轻松自动完成
域名登记・HTTPS 化的运维工具(Rust → WebAssembly,不使用框架)**

本项目是一款运维工具,目标是像 WordPress 高速化服务器构建套件
「KUSANAGI」那样,在应用上传之后一气呵成地完成**从 IP 地址启动 → 简化
域名登记 → HTTPS 自动化**的整个流程。它可以为 WordPress・PHP + Laravel・
Python + FastAPI 等任意后端技术栈的网站自动生成用于性能优化的反向代理
配置(Nginx/Apache),并配有可注册・切换・连通性测试多个站点连接目标的
「站点管理」界面。**本项目不具备连接数据库(DB)的功能**(有意排除在
范围之外)。

📖 其他语言: [日本語](README-Japan.md) / [English](README-English.md) /
[中文](README-Chinese.md) / [한국어](README-Korea.md) / [Español](README-Spain.md) /
[Français](README-France.md) / [Deutsch](README-Germany.md) / [Italiano](README-Italy.md) /
[Русский](README-Russia.md) / [العربية](README-Arabic.md)

---

## 现在支持的功能

- **站点管理界面**: 可以为 aruaru-web 自身、WordPress、Laravel、FastAPI
  等任意后端技术栈的部署目标(IP 地址/域名/子域名/端口/路径)注册多条
  记录,保存到 `localStorage` 中,一键即可选择・进行连通性测试(相当于
  KUSANAGI 的站点列表)。每张卡片都配有**「连接测试」按钮**(在不切换当前
  选中站点的情况下,仅执行简单的 HTTP 可达性检测)、端口号输入校验
  (1〜65535)、已注册站点列表的**JSON 导出/导入**功能(用于备份、迁移到
  其他浏览器),以及删除前的确认对话框。
- **从 IP 地址启动**: 通过 `scripts/serve.sh` 可以绑定到本地/VPS 上任意
  的 IP 地址与端口进行分发。
- **vhost 生成・性能优化・HTTPS 自动配置**: 通过 `scripts/gen-vhost.sh`,
  可根据域名・IP・后端技术栈的组合生成 Nginx/Apache 的 vhost 配置(内置
  HTTP→HTTPS 重定向)。支持 `static`(静态站点)・`proxy`(面向
  aruaru-db・open-web-server・open-raid-z 系或任意 HTTP 后端的通用反向
  代理)・`wordpress`・`laravel`・`fastapi` 共 5 种技术栈预设,并针对各
  技术栈内置了 gzip 压缩、静态资源长期缓存、upstream keepalive、FastCGI
  缓冲区调优等性能优化配置。
- **HTTPS(TLS)的自动监控・自动更新**: 通过 `scripts/setup-tls.sh` 获取
  Let's Encrypt(certbot)证书,并可通过 `deploy/systemd/
  install-systemd-units.sh` 启用「每天 2 次自动更新
  (`aruaru-tls-renew.timer`)」与「每天 1 次失效监控
  (`aruaru-tls-monitor.timer` → `scripts/check-all-tls.sh`)」。
- **部署到 VPS**: 只需在 Windows PowerShell 中执行 `scripts/
  deploy-vps.ps1`,即可自动完成从构建 → 上传到 VPS → 启动的全过程
  (详见下文「部署到 VPS」一节)。

## 暂不支持的功能(诚实说明)

- **不具备连接数据库(DB)的功能**。SQL 执行・GraphQL 查询等依赖特定数据库
  产品的功能属于有意排除在范围之外,今后也不会实现。即便使用类似
  aruaru-db 这样的特定后端,「站点管理」界面与 vhost 生成
  (`--stack=proxy`)也只能作为**通用的反向代理・部署管理**工具使用,
  不提供该数据库专属的查询功能。
- 认证、分页、出错时的自动重试尚未实现。
- 不提供类似 Tauri 的原生应用体验(仅为在浏览器中运行的 WASM)。
- **本仓库不会实际获取域名或登记 DNS 记录(在注册商处的操作)**(因为涉及
  费用及对外部服务的实际变更)。这里自动化的范围仅限于针对已获取域名的
  「生成 vhost 配置」以及「获取・监控・自动更新 TLS 证书」,DNS 记录的
  登记本身需要使用者自行在注册商处完成。
- 实际签约 VPS(与租用服务器供应商签约)同样不会由本仓库完成。

## 构建方法

不使用 Node.js、npm、TypeScript,仅通过 Rust 工具链即可完成。

```bash
rustup target add wasm32-unknown-unknown        # 仅需执行一次
cargo install wasm-bindgen-cli --version 0.2.126 # 仅需执行一次(需与 Cargo.lock 中的版本保持一致)

cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg \
  target/wasm32-unknown-unknown/debug/aruaru_web.wasm

# 使用任意静态服务器分发并打开(以下仅为示例)
python -m http.server 8080
# 在浏览器中打开 http://localhost:8080/index.html
```

## 从 IP 地址启动

```bash
scripts/serve.sh 0.0.0.0 8080        # 在所有网络接口上监听
scripts/serve.sh 192.168.1.50 8080   # 仅在指定的 IP 地址上监听
```

启动后,**直接在浏览器地址栏中输入 IP 地址**即可查看
(例如: `http://192.168.1.50:8080/index.html`)。要点在于即便尚未登记
域名,仅凭 IP 地址也能确认运行情况。

## 部署到 VPS(通过 Windows PowerShell)

租用 VPS 服务器之后,只需在 Windows PowerShell 中执行 `scripts/
deploy-vps.ps1`,即可自动完成从构建 → 上传 → 启动的全过程。如果同时
使用 `open-web-server`,也可以一并上传(本仓库不会涉足
`open-web-server` 的内部内容,仅指定上传目标路径)。

```powershell
# 仅上传并启动 aruaru-web 本身的情况
.\scripts\deploy-vps.ps1 -VpsHost 203.0.113.10 -VpsUser root -StartServer

# 同时上传 open-web-server (F:\open-runo\open-web-server) 的情况
.\scripts\deploy-vps.ps1 -VpsHost 203.0.113.10 -VpsUser root -StartServer `
    -OpenWebServerPath "F:\open-runo\open-web-server"
```

该脚本内部执行的是与以下等效的处理(也可以手动执行):

```powershell
# 1. 在本地构建
cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg `
    target/wasm32-unknown-unknown/debug/aruaru_web.wasm

# 2. 上传到 VPS(OpenSSH 客户端,Windows 10 1809 以降/11 标准内置)
ssh root@203.0.113.10 "mkdir -p /root/aruaru-web"
scp -r .\index.html .\pkg .\scripts .\deploy .\Cargo.toml .\src `
    root@203.0.113.10:/root/aruaru-web/

# 3. 在 VPS 上从 IP 地址启动
ssh root@203.0.113.10 "cd /root/aruaru-web && bash scripts/serve.sh 0.0.0.0 8080"
```

启动后,**在浏览器地址栏中输入 VPS 的 IP 地址**
(例如: `http://203.0.113.10:8080/index.html`)。如果希望让本地的
`F:\open-runo\aruaru-web` 等目录始终保持最新,只需对本仓库执行
`git pull` 即可:

```powershell
cd F:\open-runo\aruaru-web
git fetch origin
git pull origin <分支名>
```

**无需上传也能使用**: 若不使用 VPS,仅想在本地试用,可按照上文
「从 IP 地址启动」所述,直接在本地执行 `scripts/serve.sh` 即可。同一
局域网内的其他终端可通过 `http://<本机 IP 地址>:8080/` 进行访问。

## vhost 生成・性能优化・域名/子域名的登记

本仓库自身不会获取域名或登记 DNS 记录(因为涉及在注册商处的操作及费用,
需由使用者另行完成)。以下是针对已获取的域名/子域名,按技术栈简单地
分发内置性能优化配置的反向代理的本地自动化流程(相当于 KUSANAGI 的
「添加站点」功能)。

```bash
# aruaru-web 自身(静态站点)
scripts/gen-vhost.sh --stack=static aruaru.example.com 203.0.113.10

# 面向 aruaru-db・open-web-server・open-raid-z 系或任意后端的通用反向代理
scripts/gen-vhost.sh --stack=proxy tool.example.com 203.0.113.10 127.0.0.1:9000

# WordPress(指定 PHP-FPM 套接字/地址)
scripts/gen-vhost.sh --stack=wordpress blog.example.com 203.0.113.10 \
  unix:/run/php/php8.3-fpm.sock /var/www/blog

# Laravel(明确指定 public 目录)
scripts/gen-vhost.sh --stack=laravel app.example.com 203.0.113.10 \
  unix:/run/php/php8.3-fpm.sock /var/www/app/public

# FastAPI(面向 ASGI 服务器的反向代理,支持 WebSocket/流式传输)
scripts/gen-vhost.sh --stack=fastapi api.example.com 203.0.113.10 127.0.0.1:8000
```

将生成的配置文件(位于 `deploy/generated/` 下,属于 `.gitignore` 对象)
部署到 Nginx/Apache 的配置目录并重新加载之后,再获取证书:

```bash
scripts/setup-tls.sh aruaru.example.com admin@example.com /var/www/aruaru.example.com

# 启用自动更新(每天2次)+ 自动监控(每天1次,检测即将失效的证书)
sudo deploy/systemd/install-systemd-units.sh
```

可随时通过 `scripts/check-all-tls.sh` 手动确认所有已注册域名证书的
有效期。若在 aruaru-web 的 GUI「站点管理」界面中也登记相同的连接目标,
即可在浏览器端进行一览・连接测试。

## 验证情况(本次已完成的验证)

- `cargo check --target wasm32-unknown-unknown` / `cargo build --target
  wasm32-unknown-unknown` 均执行成功(0 条警告)。
- 通过 `wasm-bindgen --target web` 生成了 `pkg/aruaru_web.js` /
  `pkg/aruaru_web_bg.wasm`,并在真实浏览器(Chromium,经由 Playwright)中
  加载 `index.html`,实际操作并确认了以下内容: 站点管理界面中已注册站点
  的显示・新增・拒绝非法端口输入・连接测试按钮(已确认成功连通到实际
  运行中的 HTTP 服务器)・删除确认对话框(取消与执行两种情况均已确认)・
  JSON 导出。控制台中没有 JS 错误。
- 实际安装了 Nginx 1.24(Ubuntu 24.04 标准版)・Apache 2.4,并使用自签名
  证书对 `gen-vhost.sh` 全部 5 种技术栈(static/proxy/wordpress/laravel/
  fastapi)的生成物同时执行了 `nginx -t` / `apache2ctl configtest` 的
  语法验证;static/proxy 技术栈还实际启动并通过 `curl` 完成了功能验证
  (HTTP→HTTPS 重定向、静态内容分发、经由反向代理返回 502 响应)。
- 由于本次会话中没有公网域名・实际 VPS・Windows 环境,实际通过 certbot
  进行 Let's Encrypt 签发(ACME 认证)以及 `scripts/deploy-vps.ps1` 在
  真实 VPS 环境下的运行尚未验证(详情参见 CLAUDE.md)。

## 目录结构

```text
aruaru-web/
├── Cargo.toml            # crate-type = ["cdylib", "rlib"],依赖 wasm-bindgen/web-sys
├── src/
│   ├── lib.rs             # 入口点・事件绑定
│   ├── dom.rs             # DOM 操作的公共辅助函数(文件下载等)
│   ├── profiles.rs        # 站点管理(连接配置、localStorage 保存、连接测试、JSON 导入导出)
│   └── shell.rs           # HTML 外壳
├── index.html             # 加载 pkg/ 的加载器 + CSS
├── pkg/                   # wasm-bindgen 生成产物(属于 .gitignore 对象,构建时重新生成)
├── scripts/
│   ├── serve.sh            # 从任意 IP 地址启动开发服务器
│   ├── deploy-vps.ps1       # 通过 Windows PowerShell 构建・上传・启动到 VPS
│   ├── gen-vhost.sh         # 根据域名/IP/技术栈生成 Nginx・Apache 的 vhost 配置
│   ├── setup-tls.sh         # 获取 Let's Encrypt 证书
│   ├── check-tls.sh         # 检查单个域名证书的有效期
│   └── check-all-tls.sh     # 批量检查所有已注册域名的有效期
├── deploy/
│   ├── nginx/vhost-{static,proxy,wordpress,laravel,fastapi}.conf.template
│   ├── apache/vhost-{static,proxy,wordpress,laravel,fastapi}.conf.template
│   ├── systemd/             # 自动更新(renew)・自动监控(monitor)定时器全套
│   └── generated/           # gen-vhost.sh 的输出(属于 .gitignore 对象)
└── CLAUDE.md
```

## 相关项目

由于本仓库是不依赖特定数据库的通用部署・运维工具,因此可与以下项目
**配合使用**(可将其登记到「站点管理」界面,或作为 `--stack=proxy` 的
反向代理对象使用;本仓库不会涉足各仓库的内部内容):

- **aruaru-db**: https://github.com/aon-co-jp/aruaru-db
- **open-runo**: https://github.com/aon-co-jp/open-runo
- **open-web-server**: https://github.com/aon-co-jp/open-web-server
- **poem-cosmo-tauri**: https://github.com/aon-co-jp/poem-cosmo-tauri
- **open-raid-z**(开发规范的正本): https://github.com/aon-co-jp/open-raid-z
- **rs-to-readme**: https://github.com/aon-co-jp/rs-to-readme

## License

Apache-2.0
