# aruaru-web

## Eine minimale Web-UI für aruaru-db (Rust → WebAssembly, ohne Framework)

Dies ist die erste Bootstrap-Version einer Browser-UI, die den von
`aruaru-db` (der verteilten Git-on-SQL-Datenbank) über GraphQL (`/graphql`)
bereitgestellten `sql`-Query und den `registrySummary`-Query (Aggregat der
Registry unterstützter Datenbanken) tatsächlich aufruft und die Ergebnisse
darstellt. **Dies ist noch keine vollständige Admin-Konsole** — der Umfang
ist bewusst klein gehalten: ein SQL-Eingabefeld, ein Registry-Summary-Button
und eine Ergebnistabelle.

📖 Weitere Sprachen: [日本語](README-Japan.md) / [English](README-English.md) /
[中文](README-Chinese.md) / [한국어](README-Korea.md) / [Español](README-Spain.md) /
[Français](README-France.md) / [Deutsch](README-Germany.md) / [Italiano](README-Italy.md) /
[Русский](README-Russia.md) / [العربية](README-Arabic.md)

## Aktueller Stand

- Sendet echte GraphQL-Anfragen per `fetch()` an den `/graphql`-Endpunkt von
  `aruaru-graphql` (`sql(query: String!)`, `registrySummary`).
- Ist `aruaru-server` nicht gestartet oder nicht erreichbar, rendert die
  Seite **Beispieldaten in exakt der Form des echten Schemas** und markiert
  sie deutlich als Offline-Beispiel (in einem echten Browser verifiziert).
- GraphQL-Mutationen, Authentifizierung und Paginierung sind noch nicht
  implementiert.

## Schnellstart

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.126
cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg \
  target/wasm32-unknown-unknown/debug/aruaru_web.wasm
python -m http.server 8080   # danach http://localhost:8080/index.html öffnen
```

## Struktur

`Cargo.toml` (`cdylib`/`rlib`, wasm-bindgen/web-sys-Abhängigkeiten),
`src/lib.rs` (einzige Quelldatei: DOM-Aufbau, fetch, GraphQL-Response-
Rendering), `index.html` (schlanker Loader für `pkg/`).

## Lizenz

Apache-2.0
