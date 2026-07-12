# aruaru-web

**Minimale Web-UI für aruaru-db (Rust → WebAssembly, ohne Framework)**

Dies ist ein Tab-basiertes Dashboard, das die von `aruaru-db` (der verteilten
Git-on-SQL-Datenbank) über GraphQL (`/graphql`) bereitgestellten Queries `sql`
und `registrySummary` (Aggregat der Registry unterstützter Datenbanken) direkt
aus dem Browser heraus aufruft und die Ergebnisse anzeigt. Neben der
SQL-Ausführung und der Registry-Zusammenfassung besitzt es, ähnlich der
Site-Liste von KUSANAGI, einen **"Sitenverwaltung"-Tab, in dem sich mehrere
Verbindungsziele (für aruaru-web selbst sowie für andere Projekte) anlegen und
per Klick wechseln lassen**.

📖 Andere Sprachen: [日本語](README-Japan.md) / [English](README-English.md) /
[中文](README-Chinese.md) / [한국어](README-Korea.md) / [Español](README-Spain.md) /
[Français](README-France.md) / [Deutsch](README-Germany.md) / [Italiano](README-Italy.md) /
[Русский](README-Russia.md) / [العربية](README-Arabic.md)

---

## Was aktuell funktioniert

- Sendet echte GraphQL-Anfragen per `fetch()` an den `/graphql`-Endpunkt von
  `aruaru-graphql` (`aruaru-db/crates/aruaru-graphql`):
  - `sql(query: String!): QueryResultGql` — führt beliebiges SQL aus und
    zeigt `columns`/`rows`/`commandTag` als Tabelle an
  - `registrySummary: RegistrySummaryGql` — zeigt die Zusammenfassung der
    Registry unterstützter Datenbanken (über 150 Einträge) als Karten an
  - `registry: [DbEntryGql!]!` — zeigt die **Liste** der Registry
    unterstützter Datenbanken als Tabelle an (Name/Kategorie/
    Wire-Kompatibilität/Status/Rang/Score/Aktualisierungsdatum)
- **Versionsverwaltung-Tab**: Führt `currentBranch`/`branches` (Liste der
  Branches sowie der aktuelle Branch), `log(limit)` (Commit-Log) und
  `diff(from, to)` (Anzahl der zwischen zwei Branches hinzugefügten,
  gelöschten und geänderten Zeilen) aus. All dies basiert auf dem echten
  Schema (`VcsQuery`) von `aruaru-db/crates/aruaru-graphql`.
- Ist `aruaru-server` nicht gestartet oder nicht erreichbar, rendert die Seite
  sofort **Beispieldaten in exakt der Form des echten Schemas** und
  kennzeichnet sie deutlich als "Offline-Beispiel" (dieses Verhalten wurde in
  einem echten Browser verifiziert — siehe "Durchgeführte Tests" weiter
  unten).
- **Sitenverwaltung-Tab**: Mehrere Verbindungsziele (für aruaru-web selbst
  sowie für andere Projekte) — IP-Adresse/Domain/Subdomain/Port/Pfad — können
  angelegt und in `localStorage` gespeichert werden; der Wechsel erfolgt per
  Klick. Das Endpunkt-Feld der Tabs "SQL" und "Registry" folgt automatisch der
  gerade ausgewählten Site. Jede Karte besitzt eine
  **"Verbindungstest"-Taste**, mit der sich die Erreichbarkeit prüfen lässt,
  ohne die aktive Site zu wechseln, außerdem eine Eingabevalidierung für die
  Portnummer (1–65535), **JSON-Export/-Import** der gesamten Site-Liste (für
  Backups bzw. die Mitnahme in einen anderen Browser) sowie einen
  Bestätigungsdialog vor dem Löschen.
- **Verbesserte Bedienbarkeit im SQL-Tab**: ein **Query-Verlauf** der letzten
  10 Abfragen (per Klick erneut ausführbar, vollständiger Text beim Hovern
  sichtbar), eine **Tastenkombination Ctrl+Enter / Cmd+Enter** zum Ausführen,
  **CSV-Export** der Ergebnisse, Deaktivierung der Ausführen-Taste während
  der Abfrage sowie eine Ergebnistabelle mit Zeilenzähler, Scrollbarkeit und
  fixiertem Tabellenkopf.
- **Automatische Einrichtung, Überwachung und Erneuerung von HTTPS (TLS)**:
  `scripts/gen-vhost.sh` erzeugt einen Nginx-/Apache-vhost (inklusive
  HTTP→HTTPS-Weiterleitung), `scripts/setup-tls.sh` besorgt das Zertifikat
  über Let's Encrypt (certbot), und `deploy/systemd/install-systemd-units.sh`
  aktiviert eine "zweimal täglich laufende automatische Erneuerung"
  (`aruaru-tls-renew.timer`) sowie eine "einmal täglich laufende Überwachung
  auf ablaufende Zertifikate" (`aruaru-tls-monitor.timer` →
  `scripts/check-all-tls.sh`). Details siehe Abschnitt "HTTPS sowie
  Registrierung von Domains/Subdomains".

## Was aktuell (noch) nicht funktioniert (ehrlicher Umfang)

- GraphQL-Mutationen (Branch-Erstellung, Merge, Registry-Crawl usw.) sind
  noch nicht implementiert.
- Authentifizierung, Paginierung und automatische Wiederholung bei Fehlern
  sind noch nicht implementiert.
- Es wird kein natives App-Erlebnis wie bei Tauri geboten (nur WASM, das im
  Browser läuft).
- **Der eigentliche Domain-Erwerb und das Anlegen von DNS-Einträgen (bei der
  Registrierungsstelle) erfolgen nicht aus diesem Repository heraus** (da
  damit Kosten und Auswirkungen auf externe Dienste verbunden sind). Was hier
  automatisiert ist, beschränkt sich auf "Erzeugung der vhost-Konfiguration"
  sowie "Erwerb, Überwachung und automatische Erneuerung des
  TLS-Zertifikats" für eine bereits registrierte Domain — die
  DNS-Registrierung selbst nimmt der Nutzer bei der Registrierungsstelle vor.

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

Um es mit einem tatsächlich laufenden `aruaru-db` auszuprobieren:

```bash
cd ../aruaru-db
cargo run -p aruaru-server -- --data ./data --raft-id 1   # GraphQL läuft danach auf :4000
```

## Start über eine IP-Adresse

```bash
scripts/serve.sh 0.0.0.0 8080        # auf allen Schnittstellen lauschen
scripts/serve.sh 192.168.1.50 8080   # nur auf einer bestimmten IP-Adresse lauschen
```

## HTTPS sowie Registrierung von Domains/Subdomains

Dieses Repository selbst übernimmt weder den Domain-Erwerb noch die
Registrierung von DNS-Einträgen (das erfolgt bei der Registrierungsstelle und
verursacht Kosten, daher führt der Nutzer dies separat durch). Im Folgenden
geht es um die lokale Automatisierung, mit der eine bereits registrierte
Domain/Subdomain bequem für aruaru-web bzw. andere Projekte bereitgestellt
werden kann (vergleichbar mit "Site hinzufügen" bei KUSANAGI).

```bash
# 1. vhost (Nginx/Apache, inklusive HTTP→HTTPS-Weiterleitung) aus Domain + IP + Backend erzeugen
scripts/gen-vhost.sh aruaru.example.com 203.0.113.10 127.0.0.1:4000
# Ebenso für eine Subdomain mit anderem Zweck (nur UPSTREAM/WEBROOT anpassen)
scripts/gen-vhost.sh tool.example.com 203.0.113.10 127.0.0.1:9000 /var/www/tool

# 2. Die erzeugte Konfigurationsdatei einspielen und neu laden (unter deploy/generated/, von .gitignore ausgeschlossen)

# 3. TLS-Zertifikat beziehen (Let's Encrypt / certbot)
scripts/setup-tls.sh aruaru.example.com admin@example.com /var/www/aruaru.example.com

# 4. Automatische Erneuerung (zweimal täglich) + automatische Überwachung (einmal täglich, erkennt bevorstehenden Ablauf) aktivieren
sudo deploy/systemd/install-systemd-units.sh
```

Die Ablaufdaten der Zertifikate aller registrierten Domains lassen sich
jederzeit manuell mit `scripts/check-all-tls.sh` prüfen. Trägt man dieselben
Verbindungsziele zusätzlich im "Sitenverwaltung"-Tab der aruaru-web-GUI ein,
stimmen die Ziele mit der Verbindungsauswahl im Browser überein.

## Durchgeführte Tests (in diesem Durchlauf)

- `cargo check --target wasm32-unknown-unknown` sowie `cargo build --target
  wasm32-unknown-unknown` waren beide erfolgreich (0 Warnungen).
- Mit `wasm-bindgen --target web` wurden `pkg/aruaru_web.js` und
  `pkg/aruaru_web_bg.wasm` erzeugt und `index.html` in einem echten Browser
  (Chromium, über Playwright) geladen. Dabei wurde tatsächlich Folgendes per
  Klick geprüft: Tab-Wechsel, SQL-Ausführung mit anschließendem Rendering des
  Offline-Fallbacks, Aufzeichnung und erneutes Laden aus dem Query-Verlauf
  samt Tooltip beim Hovern, die Ctrl+Enter-Tastenkombination, der CSV-Export
  (mit tatsächlich ausgelöstem Download), das Abrufen der
  Registry-Zusammenfassung und der Liste registrierter Datenbanken, das
  Abrufen von Branch-Liste, Commit-Log und Diff im Versionsverwaltung-Tab
  (jeweils einschließlich des Offline-Fallbacks), die Anzeige registrierter
  Sites im Sitenverwaltung-Tab, das Anlegen neuer Sites, die Ablehnung
  ungültiger Porteingaben, die "Verbindungstest"-Taste, JSON-Export/-Import
  (Roundtrip verifiziert) sowie der Bestätigungsdialog vor dem Löschen
  (sowohl Abbrechen als auch Ausführen). Es traten keine JS-Fehler in der
  Konsole auf (nur die beabsichtigten Protokollmeldungen zu fehlgeschlagenen
  Verbindungen).
- Nginx 1.24 (Standard unter Ubuntu 24.04), Apache 2.4 und certbot wurden
  tatsächlich installiert; die von `scripts/gen-vhost.sh` erzeugten
  Konfigurationen wurden mit einem selbstsignierten Zertifikat real
  gestartet und per `curl` geprüft (HTTP→HTTPS-Weiterleitung,
  ACME-Challenge-Pfad, Reverse-Proxy auf `/graphql`). `scripts/check-tls.sh`
  wurde gegen den tatsächlich laufenden HTTPS-Server ausgeführt, wobei alle
  drei Zustände WARN/healthy/ERROR bestätigt wurden, und `deploy/systemd/*`
  wurde mit `systemd-analyze verify` geprüft (0 Fehler). Dabei wurde ein
  echter Fehler in der Nginx-vhost-Vorlage gefunden und behoben (`http2
  on;` führt unter Nginx 1.24 zu einem Syntaxfehler). Der tatsächliche
  Zertifikatsbezug bei Let's Encrypt über certbot (ACME-Verifizierung)
  konnte mangels öffentlicher Domain und wegen einer Python-ABI-Inkompatibilität
  in der Testumgebung nicht verifiziert werden (Details siehe CLAUDE.md).

## Struktur

```text
aruaru-web/
├── Cargo.toml            # crate-type = ["cdylib", "rlib"], Abhängigkeiten wasm-bindgen/web-sys
├── src/
│   ├── lib.rs             # Einstiegspunkt, Tab-Umschaltung, Verdrahtung der Events
│   ├── dom.rs             # gemeinsame Hilfsfunktionen für DOM-Zugriffe (Datei-Download usw.)
│   ├── graphql.rs         # fetch-Aufrufe gegen /graphql
│   ├── render.rs          # Rendering der SQL-Ergebnisse und der Registry-Zusammenfassung, CSV-Ausgabe
│   ├── profiles.rs        # Sitenverwaltung (Verbindungsprofile, Speicherung in localStorage, JSON-Import/-Export)
│   ├── history.rs         # SQL-Query-Verlauf (letzte 10 Einträge, Speicherung in localStorage)
│   └── shell.rs           # HTML-Shell (Tabs, Formulare)
├── index.html             # Loader für pkg/ + CSS
├── pkg/                   # von wasm-bindgen erzeugte Artefakte (von .gitignore ausgeschlossen, wird beim Build neu erzeugt)
├── scripts/
│   ├── serve.sh            # startet den Dev-Server, bereitgestellt über eine beliebige IP-Adresse
│   ├── gen-vhost.sh         # erzeugt einen Nginx-/Apache-vhost aus Domain/IP
│   ├── setup-tls.sh         # Bezug eines Let's-Encrypt-Zertifikats
│   ├── check-tls.sh         # prüft das Ablaufdatum des Zertifikats einer einzelnen Domain
│   └── check-all-tls.sh     # prüft gesammelt das Ablaufdatum aller registrierten Domains
├── deploy/
│   ├── nginx/vhost.conf.template
│   ├── apache/vhost.conf.template
│   ├── systemd/             # vollständiger Satz an Timern für automatische Erneuerung (renew) und Überwachung (monitor)
│   └── generated/           # Ausgabe von gen-vhost.sh (von .gitignore ausgeschlossen)
└── CLAUDE.md
```

## Zugehörige Projekte

- **aruaru-db** (das Backend, mit dem sich diese UI verbindet): https://github.com/aon-co-jp/aruaru-db
- **open-runo**: https://github.com/aon-co-jp/open-runo
- **open-web-server**: https://github.com/aon-co-jp/open-web-server
- **poem-cosmo-tauri**: https://github.com/aon-co-jp/poem-cosmo-tauri
- **open-raid-z** (Quelle der maßgeblichen Entwicklungsrichtlinien): https://github.com/aon-co-jp/open-raid-z
- **rs-to-readme**: https://github.com/aon-co-jp/rs-to-readme

## License

Apache-2.0
