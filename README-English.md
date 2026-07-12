# aruaru-web

**A minimal web UI for aruaru-db (Rust -> WebAssembly, no framework)**

A tabbed dashboard that calls the `sql` query and the `registrySummary`
(supported-database registry aggregate) query — both exposed over GraphQL
(`/graphql`) by `aruaru-db`, the distributed Git-on-SQL database — directly
from the browser and renders the results. Beyond that, it aims to be
**"the second KUSANAGI"** — an operations tool, in the spirit of the
WordPress-acceleration server-building kit KUSANAGI, that lets you upload
the app, launch it from an IP address, and easily apply automatic domain
registration and HTTPS setup. To that end it ships a "Site Manager" tab for
registering and switching between multiple connection targets (for
aruaru-web itself and for other projects), together with a full set of
tools for launching from an IP address, generating vhosts, and automatically
setting up, monitoring, and renewing HTTPS (TLS).

Other languages: [日本語](README-Japan.md) / [English](README-English.md) /
[中文](README-Chinese.md) / [한국어](README-Korea.md) / [Español](README-Spain.md) /
[Français](README-France.md) / [Deutsch](README-Germany.md) / [Italiano](README-Italy.md) /
[Русский](README-Russia.md) / [العربية](README-Arabic.md)

---

## What works today

- Sends real `fetch()`-based GraphQL requests to the `/graphql` endpoint
  exposed by `aruaru-graphql` (`aruaru-db/crates/aruaru-graphql`):
  - `sql(query: String!): QueryResultGql` — run arbitrary SQL and render
    `columns`/`rows`/`commandTag` as a table
  - `registrySummary: RegistrySummaryGql` — render the aggregate counts of
    the 150+ entry supported-database registry as summary cards
- If `aruaru-server` is not running or unreachable, the page immediately
  renders **sample data shaped exactly like the real schema** and clearly
  labels it as an offline sample (verified in a real browser session — see
  "Verification performed this pass" below).
- **Site Manager tab**: register multiple connection targets (IP address /
  domain / subdomain / port / path) for aruaru-web itself and for other
  projects, persisted to `localStorage`, and switch between them with one
  click. The endpoint fields on the SQL and Registry tabs automatically
  follow whichever site is currently selected. Each site card has a
  **"Test connection" button** that checks reachability without switching
  the active site, port-number input validation (1-65535), **JSON
  export/import** of the registered site list (for backups or moving
  sites to another browser), and a confirmation dialog before deletion.
- **SQL tab usability**: the last 10 entries of **query history** (click to
  reload, hover to see the full text), a **Ctrl+Enter / Cmd+Enter shortcut**
  to run the query, **CSV export** of the result set, the run button being
  disabled while a query is in flight, and a results table with a row count,
  scrolling, and a sticky header.
- **Automatic HTTPS (TLS) setup, monitoring, and renewal**: `scripts/gen-vhost.sh`
  generates an Nginx/Apache vhost (including an HTTP-to-HTTPS redirect),
  `scripts/setup-tls.sh` obtains a Let's Encrypt certificate via certbot, and
  `deploy/systemd/install-systemd-units.sh` enables both "renew twice a day"
  (`aruaru-tls-renew.timer`) and "check for impending expiry once a day"
  (`aruaru-tls-monitor.timer` → `scripts/check-all-tls.sh`). See "HTTPS and
  domain/subdomain registration" below for details.

## What doesn't work yet (honest scope)

- **Deepening the aruaru-db query/admin surface — version-control queries
  (branches, log, diff, etc.) and fetching the full registry list — is
  intentionally out of scope.** This repository's aim is to be a
  KUSANAGI-like operations tool (launching from an IP address, simplifying
  domain registration, automating HTTPS), and there are no plans to extend
  the DB admin UI beyond the minimal aruaru-db connectivity it already has —
  running SQL and viewing the registry summary.
- GraphQL mutations (branch creation, merge, registry crawl, etc.) are not
  implemented.
- No authentication, pagination, or automatic retry on error.
- No native-app experience like Tauri — this is WASM running in a browser,
  nothing else.
- **This repository does not perform actual domain acquisition or DNS record
  registration (registrar-side operations)**, since those involve cost and
  changes on external services. What is automated here goes only as far as
  generating vhost configuration and obtaining/monitoring/renewing TLS
  certificates for domains you already own; DNS registration itself is left
  to the user, done through their registrar.

## How to build

No Node.js, npm, or TypeScript — the Rust toolchain alone is sufficient.

```bash
rustup target add wasm32-unknown-unknown        # once
cargo install wasm-bindgen-cli --version 0.2.126 # once (must match the Cargo.lock version)

cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg \
  target/wasm32-unknown-unknown/debug/aruaru_web.wasm

# serve with any static file server, e.g.:
python -m http.server 8080
# open http://localhost:8080/index.html in a browser
```

To try it against a real `aruaru-db`:

```bash
cd ../aruaru-db
cargo run -p aruaru-server -- --data ./data --raft-id 1   # GraphQL comes up on :4000
```

## Launching from an IP address

```bash
scripts/serve.sh 0.0.0.0 8080        # listen on all interfaces
scripts/serve.sh 192.168.1.50 8080   # listen on a specific IP address only
```

## HTTPS and domain/subdomain registration

This repository itself does not acquire domains or register DNS records
(that involves registrar-side operations and cost, so it's left to the
user). What follows is local automation for easily provisioning
already-acquired domains/subdomains for aruaru-web or other projects — the
equivalent of KUSANAGI's "add site" feature.

```bash
# 1. Generate a vhost (Nginx/Apache, including an HTTP-to-HTTPS redirect)
#    from a domain + IP + backend
scripts/gen-vhost.sh aruaru.example.com 203.0.113.10 127.0.0.1:4000
# Do the same for a subdomain used by another project (just change UPSTREAM/WEBROOT)
scripts/gen-vhost.sh tool.example.com 203.0.113.10 127.0.0.1:9000 /var/www/tool

# 2. Deploy the generated config files and reload (under deploy/generated/, gitignored)

# 3. Obtain a TLS certificate (Let's Encrypt / certbot)
scripts/setup-tls.sh aruaru.example.com admin@example.com /var/www/aruaru.example.com

# 4. Enable automatic renewal (twice a day) + automatic monitoring
#    (once a day, detects certificates nearing expiry)
sudo deploy/systemd/install-systemd-units.sh
```

You can check certificate expiry dates for all registered domains at any
time with `scripts/check-all-tls.sh`. Registering the same connection
targets in aruaru-web's own "Site Manager" tab keeps them in sync with the
endpoint you switch to from the browser.

## Verification performed this pass

- Both `cargo check --target wasm32-unknown-unknown` and
  `cargo build --target wasm32-unknown-unknown` succeed with zero warnings.
- Generated `pkg/aruaru_web.js` / `pkg/aruaru_web_bg.wasm` with
  `wasm-bindgen --target web`, loaded `index.html` in a real Chromium
  browser session (via Playwright), and confirmed the following by actually
  operating the UI: switching tabs, running SQL and seeing it fall back to
  offline sample rendering, query history being recorded/reloaded and its
  hover tooltip, the Ctrl+Enter shortcut, CSV export (a real download
  firing), fetching the registry summary, the Site Manager tab showing
  registered sites, adding a new site, rejecting an invalid port number, the
  test connection button, JSON export/import (round-trip verified), and the
  delete confirmation dialog (both cancel and confirm). No JS errors
  appeared in the console (only the expected connection-failure logging).
- Actually installed Nginx 1.24 (the Ubuntu 24.04 default), Apache 2.4, and
  certbot, then started `scripts/gen-vhost.sh`'s generated configs with a
  self-signed certificate and verified them with `curl` (HTTP-to-HTTPS
  redirect, the ACME challenge path, and the `/graphql` reverse proxy); ran
  `scripts/check-tls.sh` against the real HTTPS server and confirmed all
  three states (WARN/healthy/ERROR); and validated `deploy/systemd/*` with
  `systemd-analyze verify` (zero errors). In the course of this, a real bug
  in the Nginx vhost template was found and fixed (`http2 on;` is a syntax
  error on Nginx 1.24). Actual certificate issuance from Let's Encrypt via
  certbot (ACME authorization) remains unverified, due to the lack of a
  public domain and a Python ABI mismatch in the test environment (see
  CLAUDE.md for details).

## Layout

```text
aruaru-web/
├── Cargo.toml            # crate-type = ["cdylib", "rlib"], wasm-bindgen/web-sys deps
├── src/
│   ├── lib.rs             # entry point, tab switching, event wiring
│   ├── dom.rs             # shared DOM helpers (file download, etc.)
│   ├── graphql.rs         # fetch calls to /graphql
│   ├── render.rs          # rendering of SQL results and registry summary, CSV output
│   ├── profiles.rs        # site management (connection profiles, localStorage persistence, JSON import/export)
│   ├── history.rs         # SQL query history (last 10 entries, localStorage persistence)
│   └── shell.rs           # HTML shell (tabs, forms)
├── index.html             # loader that imports pkg/, plus CSS
├── pkg/                   # wasm-bindgen output (gitignored, regenerated by the build)
├── scripts/
│   ├── serve.sh            # start a dev server bound to an arbitrary IP address
│   ├── gen-vhost.sh         # generate Nginx/Apache vhosts from a domain/IP
│   ├── setup-tls.sh         # obtain a Let's Encrypt certificate
│   ├── check-tls.sh         # check certificate expiry for one domain
│   └── check-all-tls.sh     # check expiry for all registered domains in one pass
├── deploy/
│   ├── nginx/vhost.conf.template
│   ├── apache/vhost.conf.template
│   ├── systemd/             # the full set of auto-renew / auto-monitor timers
│   └── generated/           # output of gen-vhost.sh (gitignored)
└── CLAUDE.md
```

## Related projects

- **aruaru-db** (what this UI talks to): https://github.com/aon-co-jp/aruaru-db
- **open-runo**: https://github.com/aon-co-jp/open-runo
- **open-web-server**: https://github.com/aon-co-jp/open-web-server
- **poem-cosmo-tauri**: https://github.com/aon-co-jp/poem-cosmo-tauri
- **open-raid-z** (canonical dev-rules doc): https://github.com/aon-co-jp/open-raid-z
- **rs-to-readme**: https://github.com/aon-co-jp/rs-to-readme

## License

Apache-2.0
</content>
</invoke>
