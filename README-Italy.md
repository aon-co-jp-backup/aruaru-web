# aruaru-web

## Una UI web minimale per aruaru-db (Rust → WebAssembly, nessun framework)

Questa è la prima versione bootstrap di una UI da browser che chiama
realmente l'endpoint GraphQL (`/graphql`) esposto da `aruaru-db` (il
database distribuito Git-on-SQL) — la query `sql` e la query
`registrySummary` (aggregato del registro dei database supportati) — e ne
mostra i risultati. **Non è ancora una console di amministrazione completa**
— l'ambito è deliberatamente ridotto a un campo di input SQL, un pulsante
per il riepilogo del registro e una tabella dei risultati.

📖 Altre lingue: [日本語](README-Japan.md) / [English](README-English.md) /
[中文](README-Chinese.md) / [한국어](README-Korea.md) / [Español](README-Spain.md) /
[Français](README-France.md) / [Deutsch](README-Germany.md) / [Italiano](README-Italy.md) /
[Русский](README-Russia.md) / [العربية](README-Arabic.md)

## Stato attuale

- Invia vere richieste GraphQL tramite `fetch()` all'endpoint `/graphql` di
  `aruaru-graphql` (`sql(query: String!)`, `registrySummary`).
- Se `aruaru-server` non è avviato o non è raggiungibile, la pagina mostra
  **dati di esempio con la stessa forma dello schema reale**, etichettati
  chiaramente come campione offline (verificato in un browser reale).
- Le mutation GraphQL, l'autenticazione e la paginazione non sono ancora
  implementate.

## Avvio rapido

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.126
cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg \
  target/wasm32-unknown-unknown/debug/aruaru_web.wasm
python -m http.server 8080   # poi apri http://localhost:8080/index.html
```

## Struttura

`Cargo.toml` (`cdylib`/`rlib`, dipendenze wasm-bindgen/web-sys),
`src/lib.rs` (unico file sorgente: costruzione del DOM, fetch, rendering
della risposta GraphQL), `index.html` (loader leggero per `pkg/`).

## Licenza

Apache-2.0
