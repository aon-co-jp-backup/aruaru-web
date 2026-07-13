# aruaru-web

**"The second KUSANAGI" — an ops tool that launches from an IP address after
you upload your app, then makes domain registration and HTTPS setup easy to
automate (Rust → WebAssembly, no framework)**

Like KUSANAGI, the WordPress-acceleration server-building kit, this tool aims
to take you from **upload → launch from an IP address → simplified domain
registration → automated HTTPS**, all in one flow. It can auto-generate
speed-tuned reverse-proxy configs (Nginx/Apache) for websites running any
backend stack — WordPress, PHP + Laravel, Python + FastAPI, and more — and it
has a "Site Management" screen where you can register, switch between, and
test connectivity to multiple sites. **It has no database connectivity
features** (intentionally out of scope).

📖 Other languages: [日本語](README-Japan.md) / [English](README-English.md) /
[中文](README-Chinese.md) / [한국어](README-Korea.md) / [Español](README-Spain.md) /
[Français](README-France.md) / [Deutsch](README-Germany.md) / [Italiano](README-Italy.md) /
[Русский](README-Russia.md) / [العربية](README-Arabic.md)

---

## What works today

- **Site Management screen**: Register multiple deployment targets
  (IP address/domain/subdomain/port/path) for aruaru-web itself, WordPress,
  Laravel, FastAPI, or any other backend stack. Entries are saved to
  `localStorage`, and you can select and test connectivity with one click
  (the equivalent of KUSANAGI's site list). Each card has a **"Test
  Connection" button** (runs a simple HTTP reachability check without
  changing the currently selected site), port-number input validation
  (1–65535), **JSON export/import** of the registered site list (for backups
  and moving data to another browser), and a confirmation dialog before
  deletion.
- **Launch from an IP address**: `scripts/serve.sh` binds and serves on any
  IP address/port, whether locally or on a VPS.
- **vhost generation, acceleration, and automated HTTPS setup**:
  `scripts/gen-vhost.sh` generates an Nginx/Apache vhost (including an
  HTTP→HTTPS redirect) from a domain, IP address, and backend stack
  combination. It supports five stacks — `static` (static sites), `proxy`
  (a generic reverse proxy for aruaru-db/open-web-server/open-raid-z-style
  backends, or anything else over HTTP), `wordpress`, `laravel`, and
  `fastapi` — and includes stack-appropriate acceleration settings such as
  gzip compression, long-lived caching for static assets, upstream
  keepalive, and FastCGI buffer tuning.
- **Automated HTTPS (TLS) monitoring and renewal**: `scripts/setup-tls.sh`
  obtains a Let's Encrypt certificate via certbot, and
  `deploy/systemd/install-systemd-units.sh` enables "automatic renewal twice
  a day" (`aruaru-tls-renew.timer`) and "expiry monitoring once a day"
  (`aruaru-tls-monitor.timer` → `scripts/check-all-tls.sh`).
- **Deploying to a VPS**: Just run `scripts/deploy-vps.ps1` from Windows
  PowerShell to automate the build → upload to the VPS → launch pipeline
  (see "Deploying to a VPS" below for details).

## What doesn't work yet (honest scope)

- **No database connectivity features.** Functionality tied to a specific
  database product, such as SQL execution or GraphQL queries, is
  intentionally out of scope and will not be implemented going forward.
  Even when you're using a specific backend like aruaru-db, the "Site
  Management" screen and vhost generation (`--stack=proxy`) can still be
  used as a **generic reverse-proxy/deployment manager**, but no
  database-specific query features are provided.
- Authentication, pagination, and automatic retry on error are not
  implemented.
- This does not provide a native-app experience like Tauri (it's WASM
  running in the browser only).
- **Actual domain acquisition and DNS record registration (operations at
  the registrar) are not performed by this repository** (since they
  involve costs and changes reflected on external services). What's
  automated here goes up to "generating vhost configs" and "obtaining,
  monitoring, and auto-renewing TLS certificates" for a domain you already
  own; DNS registration itself is left to the user to do at their
  registrar.
- Actually contracting for a VPS (a rental server agreement with a
  provider) is likewise not done by this repository.

## Build instructions

No Node.js, npm, or TypeScript — everything is done with just the Rust
toolchain.

```bash
rustup target add wasm32-unknown-unknown        # first time only
cargo install wasm-bindgen-cli --version 0.2.126 # first time only (must match the version in Cargo.lock)

cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg \
  target/wasm32-unknown-unknown/debug/aruaru_web.wasm

# Serve with any static server (anything works; for example:)
python -m http.server 8080
# Open http://localhost:8080/index.html in a browser
```

## Launching from an IP address

```bash
scripts/serve.sh 0.0.0.0 8080        # listen on all interfaces
scripts/serve.sh 192.168.1.50 8080   # listen on a specific IP address only
```

Once it's running, you can check it by **typing the IP address directly into
your browser's address bar** (e.g. `http://192.168.1.50:8080/index.html`).
The key point is that you can verify it works using just the IP address,
even before a domain has been registered.

## Deploying to a VPS (from Windows PowerShell)

After renting a VPS, just run `scripts/deploy-vps.ps1` from Windows
PowerShell to automate the build → upload → launch pipeline. If you're also
using `open-web-server`, it can be uploaded at the same time (this
repository doesn't reach into `open-web-server`'s internals — it just
specifies the upload destination path).

```powershell
# Upload and launch (aruaru-web only)
.\scripts\deploy-vps.ps1 -VpsHost 203.0.113.10 -VpsUser root -StartServer

# Also upload open-web-server (F:\open-runo\open-web-server) at the same time
.\scripts\deploy-vps.ps1 -VpsHost 203.0.113.10 -VpsUser root -StartServer `
    -OpenWebServerPath "F:\open-runo\open-web-server"
```

Internally, this performs the equivalent of the following (you can also run
these steps manually):

```powershell
# 1. Build locally
cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg `
    target/wasm32-unknown-unknown/debug/aruaru_web.wasm

# 2. Upload to the VPS (OpenSSH client, built into Windows 10 1809+/11)
ssh root@203.0.113.10 "mkdir -p /root/aruaru-web"
scp -r .\index.html .\pkg .\scripts .\deploy .\Cargo.toml .\src `
    root@203.0.113.10:/root/aruaru-web/

# 3. Launch from an IP address on the VPS
ssh root@203.0.113.10 "cd /root/aruaru-web && bash scripts/serve.sh 0.0.0.0 8080"
```

Once it's running, **type the VPS's IP address into your browser's address
bar** (e.g. `http://203.0.113.10:8080/index.html`). If you want to keep a
local copy such as `F:\open-runo\aruaru-web` up to date, just `git pull` this
repository:

```powershell
cd F:\open-runo\aruaru-web
git fetch origin
git pull origin <branch-name>
```

**You don't have to upload anything to use it**: to try it out locally
without a VPS, just run `scripts/serve.sh` locally as described in "Launching
from an IP address" above. Other devices on the same LAN can reach it at
`http://<your local PC's IP address>:8080/`.

## vhost generation, acceleration, and domain/subdomain registration

This repository itself does not perform domain acquisition or DNS record
registration (that involves operations at the registrar, and costs, so users
handle it separately). What follows is local automation — the equivalent of
KUSANAGI's "add site" — for easily issuing a reverse proxy with acceleration
settings, per stack, for a domain/subdomain you already own.

```bash
# aruaru-web itself (static site)
scripts/gen-vhost.sh --stack=static aruaru.example.com 203.0.113.10

# Generic reverse proxy for aruaru-db/open-web-server/open-raid-z-style, or any other backend
scripts/gen-vhost.sh --stack=proxy tool.example.com 203.0.113.10 127.0.0.1:9000

# WordPress (specify the PHP-FPM socket/address)
scripts/gen-vhost.sh --stack=wordpress blog.example.com 203.0.113.10 \
  unix:/run/php/php8.3-fpm.sock /var/www/blog

# Laravel (specify the public directory explicitly)
scripts/gen-vhost.sh --stack=laravel app.example.com 203.0.113.10 \
  unix:/run/php/php8.3-fpm.sock /var/www/app/public

# FastAPI (reverse proxy to an ASGI server, with WebSocket/streaming support)
scripts/gen-vhost.sh --stack=fastapi api.example.com 203.0.113.10 127.0.0.1:8000
```

After placing the generated config files (under `deploy/generated/`, which is
`.gitignore`d) into your Nginx/Apache config directory and reloading, obtain
a certificate:

```bash
scripts/setup-tls.sh aruaru.example.com admin@example.com /var/www/aruaru.example.com

# Enable auto-renewal (twice a day) + auto-monitoring (once a day, detects near-expiry certs)
sudo deploy/systemd/install-systemd-units.sh
```

You can manually check the certificate expiry of all registered domains at
any time with `scripts/check-all-tls.sh`. If you also register the same
targets in aruaru-web's GUI "Site Management" screen, you can list them and
test connectivity from the browser side too.

## Verification (performed during this pass)

- Both `cargo check --target wasm32-unknown-unknown` and
  `cargo build --target wasm32-unknown-unknown` succeed with zero warnings.
- Generated `pkg/aruaru_web.js` / `pkg/aruaru_web_bg.wasm` via
  `wasm-bindgen --target web`, loaded `index.html` in a real browser
  (Chromium, via Playwright), and confirmed the following through actual
  interaction: the Site Management screen displaying registered sites,
  adding a new site, rejecting invalid port input, the Test Connection
  button (successfully confirming reachability to an actually running HTTP
  server), the delete confirmation dialog (both cancel and confirm), and
  JSON export. No JS errors in the console.
- Actually installed Nginx 1.24 (Ubuntu 24.04 default) and Apache 2.4,
  syntax-validated the output of `gen-vhost.sh` for all five stacks
  (static/proxy/wordpress/laravel/fastapi) with self-signed certificates
  using both `nginx -t` and `apache2ctl configtest`, and for the
  static/proxy stacks actually started the servers and verified behavior
  with `curl` (HTTP→HTTPS redirect, static file serving, a 502 response
  through the reverse proxy).
- Actual Let's Encrypt issuance via certbot (ACME authentication), and
  actually running `scripts/deploy-vps.ps1` against a real VPS environment,
  remain unverified since this session doesn't have a public domain, a real
  VPS, or a Windows environment available (see CLAUDE.md for details).

## Layout

```text
aruaru-web/
├── Cargo.toml            # crate-type = ["cdylib", "rlib"], depends on wasm-bindgen/web-sys
├── src/
│   ├── lib.rs             # entry point / event wiring
│   ├── dom.rs             # shared DOM helpers (file download, etc.)
│   ├── profiles.rs        # site management (connection profiles, localStorage persistence, connection test, JSON import/export)
│   └── shell.rs           # HTML shell
├── index.html             # loader that pulls in pkg/ + CSS
├── pkg/                   # wasm-bindgen output (.gitignore'd, regenerated by the build)
├── scripts/
│   ├── serve.sh            # start a dev server that serves from any IP address
│   ├── deploy-vps.ps1       # build, upload, and launch on a VPS from Windows PowerShell
│   ├── gen-vhost.sh         # generate Nginx/Apache vhosts from a domain/IP/stack
│   ├── setup-tls.sh         # obtain a Let's Encrypt certificate
│   ├── check-tls.sh         # check certificate expiry for a single domain
│   └── check-all-tls.sh     # check expiry for all registered domains at once
├── deploy/
│   ├── nginx/vhost-{static,proxy,wordpress,laravel,fastapi}.conf.template
│   ├── apache/vhost-{static,proxy,wordpress,laravel,fastapi}.conf.template
│   ├── systemd/             # the full set of auto-renewal and auto-monitoring timers
│   └── generated/           # output of gen-vhost.sh (.gitignore'd)
└── CLAUDE.md
```

## Related projects

Since this repository is a database-agnostic, general-purpose deployment and
ops tool, it can be **used alongside** other projects such as the following
(they can be registered in the "Site Management" screen or used as a reverse
proxy target via `--stack=proxy`; this repository does not reach into the
internals of any of these repositories):

- **aruaru-db**: https://github.com/aon-co-jp/aruaru-db
- **open-runo**: https://github.com/aon-co-jp/open-runo
- **open-web-server**: https://github.com/aon-co-jp/open-web-server
- **poem-cosmo-tauri**: https://github.com/aon-co-jp/poem-cosmo-tauri
- **open-raid-z** (the source of truth for development rules): https://github.com/aon-co-jp/open-raid-z
- **rs-to-readme**: https://github.com/aon-co-jp/rs-to-readme

## License

Apache-2.0
