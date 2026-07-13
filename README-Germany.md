# aruaru-web

**"Das zweite KUSANAGI" — ein Ops-Tool, mit dem sich eine App nach dem
Hochladen per IP-Adresse starten lässt und Domain-Registrierung sowie
HTTPS-Einrichtung bequem automatisiert angewendet werden können (Rust →
WebAssembly, ohne Framework)**

Ähnlich wie der WordPress-Beschleunigungs-Baukasten "KUSANAGI" verfolgt
dieses Ops-Tool das Ziel, nach dem Hochladen einer App den kompletten Weg
von **Start per IP-Adresse → vereinfachte Domain-Registrierung → automatische
HTTPS-Einrichtung** aus einer Hand abzudecken. Für beliebige
Backend-Stacks — WordPress, PHP + Laravel, Python + FastAPI und mehr — lassen
sich beschleunigende Reverse-Proxy-Konfigurationen (Nginx/Apache) automatisch
erzeugen, und ein "Sitenverwaltung"-Bildschirm erlaubt es, Verbindungsziele
für mehrere Sites anzulegen, zu wechseln und ihre Erreichbarkeit zu prüfen.
**Eine Anbindung an Datenbanken besitzt das Projekt nicht** (bewusst
außerhalb des Umfangs).

📖 Andere Sprachen: [日本語](README-Japan.md) / [English](README-English.md) /
[中文](README-Chinese.md) / [한국어](README-Korea.md) / [Español](README-Spain.md) /
[Français](README-France.md) / [Deutsch](README-Germany.md) / [Italiano](README-Italy.md) /
[Русский](README-Russia.md) / [العربية](README-Arabic.md)

---

## Was aktuell funktioniert

- **Sitenverwaltung-Bildschirm**: Für aruaru-web selbst, WordPress, Laravel,
  FastAPI und beliebige andere Backend-Stacks lassen sich mehrere
  Deployment-Ziele (IP-Adresse/Domain/Subdomain/Port/Pfad) anlegen, in
  `localStorage` speichern und per Klick auswählen bzw. auf Erreichbarkeit
  prüfen (entspricht der Site-Liste von KUSANAGI). Jede Karte besitzt eine
  **"Verbindungstest"-Taste** (führt nur eine einfache HTTP-Erreichbarkeits-
  prüfung durch, ohne die ausgewählte Site zu wechseln), eine
  Eingabevalidierung für die Portnummer (1–65535), **JSON-Export/-Import**
  der gesamten registrierten Site-Liste (für Backups bzw. die Mitnahme in
  einen anderen Browser) sowie einen Bestätigungsdialog vor dem Löschen.
- **Start über eine IP-Adresse**: `scripts/serve.sh` stellt die App auf einer
  beliebigen IP-Adresse und einem beliebigen Port lokal oder auf einem VPS
  bereit.
- **vhost-Erzeugung, Beschleunigung und automatische HTTPS-Einrichtung**:
  `scripts/gen-vhost.sh` erzeugt aus der Kombination
  Domain/IP-Adresse/Backend-Stack einen Nginx-/Apache-vhost (inklusive
  HTTP→HTTPS-Weiterleitung). Unterstützt werden die fünf Stacks `static`
  (statische Site), `proxy` (generischer Reverse-Proxy für aruaru-db,
  open-web-server, open-raid-z-artige oder beliebige andere Backends),
  `wordpress`, `laravel` und `fastapi`, jeweils mit stackspezifischen
  Beschleunigungseinstellungen wie gzip-Kompression, langfristigem Caching
  statischer Assets, Upstream-Keepalive und angepassten FastCGI-Puffern.
- **Automatische Überwachung und Erneuerung von HTTPS (TLS)**:
  `scripts/setup-tls.sh` besorgt das Zertifikat über Let's Encrypt (certbot),
  und `deploy/systemd/install-systemd-units.sh` aktiviert eine "zweimal
  täglich laufende automatische Erneuerung" (`aruaru-tls-renew.timer`) sowie
  eine "einmal täglich laufende Überwachung auf ablaufende Zertifikate"
  (`aruaru-tls-monitor.timer` → `scripts/check-all-tls.sh`).
- **Deployment auf einen VPS**: Ein einziger Aufruf von
  `scripts/deploy-vps.ps1` aus Windows PowerShell automatisiert Build →
  Upload auf den VPS → Start (Details siehe Abschnitt "Deployment auf einen
  VPS" weiter unten).

## Was aktuell (noch) nicht funktioniert (ehrlicher Umfang)

- **Keine Anbindung an Datenbanken.** Funktionen, die von einem bestimmten
  Datenbankprodukt abhängen — etwa SQL-Ausführung oder GraphQL-Queries —
  liegen bewusst außerhalb des Umfangs und werden auch künftig nicht
  implementiert. Auch bei Verwendung eines bestimmten Backends wie
  aruaru-db lassen sich der "Sitenverwaltung"-Bildschirm und die
  vhost-Erzeugung (`--stack=proxy`) als **generischer Reverse-Proxy bzw. als
  Deployment-Verwaltung** nutzen, jedoch werden keine Query-Funktionen
  bereitgestellt, die spezifisch für diese Datenbank sind.
- Authentifizierung, Paginierung und automatische Wiederholung bei Fehlern
  sind nicht implementiert.
- Es wird kein natives App-Erlebnis wie bei Tauri geboten (nur WASM, das im
  Browser läuft).
- **Der eigentliche Domain-Erwerb sowie das Anlegen von DNS-Einträgen (bei
  der Registrierungsstelle) erfolgen nicht aus diesem Repository heraus**
  (da damit Kosten entstehen und Auswirkungen auf externe Dienste verbunden
  sind). Was hier automatisiert ist, beschränkt sich auf die "Erzeugung der
  vhost-Konfiguration" sowie den "Erwerb, die Überwachung und die
  automatische Erneuerung des TLS-Zertifikats" für eine bereits registrierte
  Domain — die DNS-Registrierung selbst nimmt der Nutzer bei der
  Registrierungsstelle vor.
- Der eigentliche Abschluss eines VPS-Vertrags (mit einem
  Hosting-Anbieter) erfolgt ebenfalls nicht aus diesem Repository heraus.

## Build-Anleitung

Node.js, npm und TypeScript werden nicht verwendet. Es genügt allein die
Rust-Toolchain.

```bash
rustup target add wasm32-unknown-unknown        # einmalig
cargo install wasm-bindgen-cli --version 0.2.126 # einmalig (muss mit der Version in Cargo.lock übereinstimmen)

cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg \
  target/wasm32-unknown-unknown/debug/aruaru_web.wasm

# Über einen beliebigen statischen Webserver bereitstellen und öffnen (Beispiel:)
python -m http.server 8080
# Im Browser http://localhost:8080/index.html öffnen
```

## Start über eine IP-Adresse

```bash
scripts/serve.sh 0.0.0.0 8080        # auf allen Schnittstellen lauschen
scripts/serve.sh 192.168.1.50 8080   # nur auf einer bestimmten IP-Adresse lauschen
```

Nach dem Start lässt sich die App überprüfen, indem man **die IP-Adresse
direkt in die Adresszeile des Browsers eingibt** (Beispiel:
`http://192.168.1.50:8080/index.html`). Der entscheidende Punkt: Die
Funktionsprüfung ist bereits allein über die IP-Adresse möglich, noch bevor
eine Domain registriert wurde.

## Deployment auf einen VPS (aus Windows PowerShell)

Nachdem ein VPS-Mietserver angeschafft wurde, genügt ein einziger Aufruf von
`scripts/deploy-vps.ps1` aus Windows PowerShell, um Build → Upload → Start
vollständig zu automatisieren. Wird `open-web-server` parallel eingesetzt,
lässt sich dieses gleichzeitig mit hochladen (dieses Repository greift dabei
nicht in den Inhalt von `open-web-server` ein, sondern gibt lediglich den
Ziel-Uploadpfad an).

```powershell
# Nur hochladen und starten (ausschließlich aruaru-web)
.\scripts\deploy-vps.ps1 -VpsHost 203.0.113.10 -VpsUser root -StartServer

# Zusätzlich open-web-server (F:\open-runo\open-web-server) mit hochladen
.\scripts\deploy-vps.ps1 -VpsHost 203.0.113.10 -VpsUser root -StartServer `
    -OpenWebServerPath "F:\open-runo\open-web-server"
```

Intern entspricht dies den folgenden Schritten (lassen sich auch manuell
ausführen):

```powershell
# 1. Lokaler Build
cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg `
    target/wasm32-unknown-unknown/debug/aruaru_web.wasm

# 2. Upload auf den VPS (OpenSSH-Client, ab Windows 10 1809 bzw. Windows 11 standardmäßig vorhanden)
ssh root@203.0.113.10 "mkdir -p /root/aruaru-web"
scp -r .\index.html .\pkg .\scripts .\deploy .\Cargo.toml .\src `
    root@203.0.113.10:/root/aruaru-web/

# 3. Start auf dem VPS über eine IP-Adresse
ssh root@203.0.113.10 "cd /root/aruaru-web && bash scripts/serve.sh 0.0.0.0 8080"
```

Nach dem Start gibt man **die IP-Adresse des VPS in die Adresszeile des
Browsers ein** (Beispiel: `http://203.0.113.10:8080/index.html`). Soll die
lokale Kopie unter `F:\open-runo\aruaru-web` o.Ä. stets aktuell gehalten
werden, genügt ein `git pull` in diesem Repository:

```powershell
cd F:\open-runo\aruaru-web
git fetch origin
git pull origin <Branch-Name>
```

**Auch ohne Upload nutzbar**: Wer die App ohne VPS nur lokal ausprobieren
möchte, führt einfach `scripts/serve.sh` wie im obigen Abschnitt "Start über
eine IP-Adresse" beschrieben lokal aus. Von anderen Geräten im selben LAN
aus ist die App dann über `http://<IP-Adresse-des-lokalen-PCs>:8080/`
erreichbar.

## vhost-Erzeugung, Beschleunigung und Registrierung von Domains/Subdomains

Dieses Repository selbst übernimmt weder den Domain-Erwerb noch die
Registrierung von DNS-Einträgen (das erfolgt bei der Registrierungsstelle
und verursacht Kosten, daher führt der Nutzer dies separat durch). Im
Folgenden geht es um die lokale Automatisierung, mit der für eine bereits
registrierte Domain/Subdomain bequem ein beschleunigter Reverse-Proxy je
nach Stack bereitgestellt werden kann (vergleichbar mit "Site hinzufügen"
bei KUSANAGI).

```bash
# aruaru-web selbst (statische Site)
scripts/gen-vhost.sh --stack=static aruaru.example.com 203.0.113.10

# Generischer Reverse-Proxy für aruaru-db, open-web-server, open-raid-z-artige oder beliebige Backends
scripts/gen-vhost.sh --stack=proxy tool.example.com 203.0.113.10 127.0.0.1:9000

# WordPress (PHP-FPM-Socket/Adresse angeben)
scripts/gen-vhost.sh --stack=wordpress blog.example.com 203.0.113.10 \
  unix:/run/php/php8.3-fpm.sock /var/www/blog

# Laravel (public-Verzeichnis explizit angeben)
scripts/gen-vhost.sh --stack=laravel app.example.com 203.0.113.10 \
  unix:/run/php/php8.3-fpm.sock /var/www/app/public

# FastAPI (Reverse-Proxy zu einem ASGI-Server, mit WebSocket-/Streaming-Unterstützung)
scripts/gen-vhost.sh --stack=fastapi api.example.com 203.0.113.10 127.0.0.1:8000
```

Nachdem die erzeugte Konfigurationsdatei (unter `deploy/generated/`, von
`.gitignore` ausgeschlossen) in das Konfigurationsverzeichnis von
Nginx/Apache kopiert und neu geladen wurde, wird das Zertifikat bezogen:

```bash
scripts/setup-tls.sh aruaru.example.com admin@example.com /var/www/aruaru.example.com

# Automatische Erneuerung (zweimal täglich) + automatische Überwachung (einmal täglich, erkennt bevorstehenden Ablauf) aktivieren
sudo deploy/systemd/install-systemd-units.sh
```

Die Ablaufdaten der Zertifikate aller registrierten Domains lassen sich
jederzeit manuell mit `scripts/check-all-tls.sh` prüfen. Trägt man dieselben
Verbindungsziele zusätzlich im "Sitenverwaltung"-Bildschirm der
aruaru-web-GUI ein, lassen sie sich von der Browserseite aus auflisten und
auf Erreichbarkeit prüfen.

## Durchgeführte Tests (in diesem Durchlauf)

- `cargo check --target wasm32-unknown-unknown` sowie `cargo build --target
  wasm32-unknown-unknown` waren beide erfolgreich (0 Warnungen).
- Mit `wasm-bindgen --target web` wurden `pkg/aruaru_web.js` und
  `pkg/aruaru_web_bg.wasm` erzeugt, und `index.html` wurde in einem echten
  Browser (Chromium, über Playwright) geladen. Dabei wurde tatsächlich
  Folgendes per Klick geprüft: Anzeige registrierter Sites im
  Sitenverwaltung-Bildschirm, Anlegen neuer Sites, Ablehnung ungültiger
  Porteingaben, die "Verbindungstest"-Taste (einschließlich erfolgreicher
  Erreichbarkeitsprüfung gegen einen tatsächlich laufenden HTTP-Server), der
  Bestätigungsdialog vor dem Löschen (sowohl Abbrechen als auch Ausführen)
  sowie der JSON-Export. Es traten keine JS-Fehler in der Konsole auf.
- Nginx 1.24 (Standard unter Ubuntu 24.04) und Apache 2.4 wurden tatsächlich
  installiert; die von `gen-vhost.sh` für alle fünf Stacks
  (static/proxy/wordpress/laravel/fastapi) erzeugten Konfigurationen wurden
  mit einem selbstsignierten Zertifikat sowohl mit `nginx -t` als auch mit
  `apache2ctl configtest` syntaktisch geprüft; für die Stacks static/proxy
  wurde zusätzlich real gestartet und per `curl` funktional geprüft
  (HTTP→HTTPS-Weiterleitung, statisches Ausliefern, 502-Antwort über den
  Reverse-Proxy).
- Der tatsächliche Zertifikatsbezug bei Let's Encrypt über certbot
  (ACME-Verifizierung) sowie das Verhalten von `scripts/deploy-vps.ps1` in
  einer echten VPS-Umgebung wurden nicht verifiziert, da für diese Sitzung
  weder eine öffentliche Domain noch ein echter VPS noch eine
  Windows-Umgebung zur Verfügung standen (Details siehe CLAUDE.md).

## Struktur

```text
aruaru-web/
├── Cargo.toml            # crate-type = ["cdylib", "rlib"], Abhängigkeiten wasm-bindgen/web-sys
├── src/
│   ├── lib.rs             # Einstiegspunkt, Verdrahtung der Events
│   ├── dom.rs             # gemeinsame Hilfsfunktionen für DOM-Zugriffe (Datei-Download usw.)
│   ├── profiles.rs        # Sitenverwaltung (Verbindungsprofile, Speicherung in localStorage, Verbindungstest, JSON-Import/-Export)
│   └── shell.rs           # HTML-Shell
├── index.html             # Loader für pkg/ + CSS
├── pkg/                   # von wasm-bindgen erzeugte Artefakte (von .gitignore ausgeschlossen, wird beim Build neu erzeugt)
├── scripts/
│   ├── serve.sh            # startet den Dev-Server, bereitgestellt über eine beliebige IP-Adresse
│   ├── deploy-vps.ps1       # Build, Upload und Start auf einem VPS aus Windows PowerShell
│   ├── gen-vhost.sh         # erzeugt einen Nginx-/Apache-vhost aus Domain/IP/Stack
│   ├── setup-tls.sh         # Bezug eines Let's-Encrypt-Zertifikats
│   ├── check-tls.sh         # prüft das Ablaufdatum des Zertifikats einer einzelnen Domain
│   └── check-all-tls.sh     # prüft gesammelt das Ablaufdatum aller registrierten Domains
├── deploy/
│   ├── nginx/vhost-{static,proxy,wordpress,laravel,fastapi}.conf.template
│   ├── apache/vhost-{static,proxy,wordpress,laravel,fastapi}.conf.template
│   ├── systemd/             # vollständiger Satz an Timern für automatische Erneuerung (renew) und Überwachung (monitor)
│   └── generated/           # Ausgabe von gen-vhost.sh (von .gitignore ausgeschlossen)
└── CLAUDE.md
```

## Zugehörige Projekte

Da dieses Repository ein generisches, datenbankunabhängiges Deployment- und
Ops-Tool ist, lässt es sich zusammen mit den folgenden Projekten **kombiniert
einsetzen** (Eintragung im "Sitenverwaltung"-Bildschirm bzw. Nutzung als
Reverse-Proxy-Ziel über `--stack=proxy`; in den Inhalt der jeweiligen
Repositories wird dabei nicht eingegriffen):

- **aruaru-db**: https://github.com/aon-co-jp/aruaru-db
- **open-runo**: https://github.com/aon-co-jp/open-runo
- **open-web-server**: https://github.com/aon-co-jp/open-web-server
- **poem-cosmo-tauri**: https://github.com/aon-co-jp/poem-cosmo-tauri
- **open-raid-z** (Quelle der maßgeblichen Entwicklungsrichtlinien): https://github.com/aon-co-jp/open-raid-z
- **rs-to-readme**: https://github.com/aon-co-jp/rs-to-readme

## License

Apache-2.0
