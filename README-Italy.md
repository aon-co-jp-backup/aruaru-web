# aruaru-web

**UI Web minimale per aruaru-db (Rust → WebAssembly, senza framework)**

Una dashboard a schede che invoca realmente, dal browser, la query `sql` e
la query `registrySummary` (aggregato del registro dei database supportati)
esposte tramite GraphQL (`/graphql`) da `aruaru-db` (il database distribuito
Git-on-SQL), mostrandone i risultati. Oltre all'esecuzione di SQL e
all'aggregazione del registro, dispone di una scheda **"Gestione siti"** che,
come l'elenco dei siti di KUSANAGI, permette di registrare e alternare tra
**più destinazioni di connessione (per aruaru-web e per altri progetti)**.

📖 Altre lingue: [日本語](README-Japan.md) / [English](README-English.md) /
[中文](README-Chinese.md) / [한국어](README-Korea.md) / [Español](README-Spain.md) /
[Français](README-France.md) / [Deutsch](README-Germany.md) / [Italiano](README-Italy.md) /
[Русский](README-Russia.md) / [العربية](README-Arabic.md)

---

## Cosa funziona già

- Invia richieste GraphQL reali tramite `fetch()` all'endpoint `/graphql` di
  `aruaru-graphql` (`aruaru-db/crates/aruaru-graphql`):
  - `sql(query: String!): QueryResultGql` — esegue SQL arbitrario e mostra
    `columns`/`rows`/`commandTag` in una tabella
  - `registrySummary: RegistrySummaryGql` — mostra in schede riepilogative
    l'aggregato del registro dei database supportati (oltre 150 voci)
- Se `aruaru-server` non è avviato o non è raggiungibile, viene disegnato al
  volo un **set di dati di esempio con la stessa forma dello schema reale**,
  indicando chiaramente che si tratta di un "campione offline" (comportamento
  verificato in un browser reale — vedi "Verifica di funzionamento" più
  sotto).
- **Scheda "Gestione siti"**: permette di registrare più destinazioni di
  connessione per aruaru-web e per altri progetti (indirizzo IP/dominio/
  sottodominio/porta/percorso), salvandole in `localStorage` e passando
  dall'una all'altra con un clic. Il campo endpoint delle schede SQL/Registro
  segue automaticamente il sito selezionato. Ogni scheda sito dispone di un
  **pulsante "Test di connessione"** che verifica la raggiungibilità senza
  cambiare il sito attivo, di una validazione del numero di porta (da 1 a
  65535), di **esportazione/importazione JSON** dell'elenco dei siti
  registrati (per backup o trasferimento verso un altro browser) e di una
  finestra di conferma prima dell'eliminazione.
- **Usabilità della scheda SQL**: **cronologia delle query** delle ultime 10
  esecuzioni (clic per ricaricarle, tooltip al passaggio del mouse con il
  testo completo), **scorciatoia di esecuzione con Ctrl+Invio / Cmd+Invio**,
  **esportazione CSV** dei risultati, disabilitazione del pulsante durante
  l'esecuzione, tabella dei risultati con conteggio delle righe, scorrimento
  e intestazione fissa (sticky).
- **Configurazione, monitoraggio e rinnovo automatico di HTTPS (TLS)**:
  `scripts/gen-vhost.sh` genera i vhost Nginx/Apache (redirect HTTP→HTTPS
  incluso), `scripts/setup-tls.sh` ottiene il certificato Let's Encrypt
  (certbot) e `deploy/systemd/install-systemd-units.sh` abilita il "rinnovo
  automatico due volte al giorno" (`aruaru-tls-renew.timer`) e il
  "monitoraggio della scadenza una volta al giorno"
  (`aruaru-tls-monitor.timer` → `scripts/check-all-tls.sh`). Per i dettagli
  vedi "Registrazione di HTTPS e dominio/sottodominio".

## Cosa non funziona ancora (limiti dichiarati onestamente)

- Le mutation GraphQL (creazione branch, merge, crawl del registro, ecc.)
  non sono implementate.
- Autenticazione, paginazione e retry automatico in caso di errore non sono
  implementati.
- Non viene offerta un'esperienza da app nativa in stile Tauri (solo WASM
  eseguito nel browser).
- **La registrazione effettiva di domini e dei record DNS (operazioni presso
  il registrar) non viene eseguita da questo repository** (comporta costi e
  ricadute su servizi esterni). Ciò che viene automatizzato qui si limita,
  per un dominio già ottenuto, alla "generazione della configurazione vhost"
  e all'"ottenimento, monitoraggio e rinnovo automatico del certificato TLS";
  la registrazione DNS in sé spetta all'utente presso il proprio registrar.

## Modalità di build

Non vengono usati Node.js, npm o TypeScript: tutto si completa con la sola
toolchain Rust.

```bash
rustup target add wasm32-unknown-unknown        # solo alla prima esecuzione
cargo install wasm-bindgen-cli --version 0.2.126 # solo alla prima esecuzione (deve corrispondere alla versione in Cargo.lock)

cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg \
  target/wasm32-unknown-unknown/debug/aruaru_web.wasm

# avvia un server statico qualsiasi per servire i file (esempio:)
python -m http.server 8080
# apri http://localhost:8080/index.html nel browser
```

Per provarlo con `aruaru-db` realmente in esecuzione:

```bash
cd ../aruaru-db
cargo run -p aruaru-server -- --data ./data --raft-id 1   # il servizio GraphQL parte sulla porta :4000
```

## Avvio da un indirizzo IP

```bash
scripts/serve.sh 0.0.0.0 8080        # in ascolto su tutte le interfacce
scripts/serve.sh 192.168.1.50 8080   # in ascolto solo su un indirizzo IP specifico
```

## Registrazione di HTTPS e dominio/sottodominio

Questo repository non esegue di per sé la registrazione di domini né dei
record DNS (comporta operazioni presso il registrar e costi, e resta quindi
a carico dell'utente farlo separatamente). Quanto segue è l'automazione
locale (equivalente alla funzione "aggiungi sito" di KUSANAGI) per assegnare
facilmente un dominio/sottodominio già ottenuto ad aruaru-web o ad altri
progetti.

```bash
# 1. Genera il vhost (Nginx/Apache, redirect HTTP→HTTPS incluso) da dominio+IP+backend
scripts/gen-vhost.sh aruaru.example.com 203.0.113.10 127.0.0.1:4000
# Allo stesso modo per un sottodominio con altro scopo (basta cambiare UPSTREAM/WEBROOT)
scripts/gen-vhost.sh tool.example.com 203.0.113.10 127.0.0.1:9000 /var/www/tool

# 2. Distribuisci il file di configurazione generato e ricarica il server (sotto deploy/generated/, escluso da .gitignore)

# 3. Ottieni il certificato TLS (Let's Encrypt / certbot)
scripts/setup-tls.sh aruaru.example.com admin@example.com /var/www/aruaru.example.com

# 4. Abilita il rinnovo automatico (due volte al giorno) + il monitoraggio automatico (una volta al giorno, rileva la scadenza imminente)
sudo deploy/systemd/install-systemd-units.sh
```

La scadenza dei certificati di tutti i domini registrati può essere
verificata manualmente in qualsiasi momento con `scripts/check-all-tls.sh`.
Registrando la stessa destinazione anche nella scheda "Gestione siti" della
GUI di aruaru-web, è possibile mantenerla allineata al cambio di
destinazione effettuato dal browser.

## Verifica di funzionamento (eseguita in questo passaggio)

- Sia `cargo check --target wasm32-unknown-unknown` sia `cargo build
  --target wasm32-unknown-unknown` completati con successo (zero warning).
- `wasm-bindgen --target web` con cui sono stati generati
  `pkg/aruaru_web.js` / `pkg/aruaru_web_bg.wasm`, caricando poi `index.html`
  in un browser reale (Chromium, tramite Playwright) e verificando
  concretamente, con interazioni reali: il cambio di scheda, l'esecuzione
  SQL con conseguente rendering di fallback offline, la registrazione/
  ricarica nella cronologia delle query e il tooltip al passaggio del
  mouse, la scorciatoia Ctrl+Invio, l'esportazione CSV (con avvio effettivo
  del download), l'aggregazione del registro, e nella scheda "Gestione
  siti": la visualizzazione dei siti registrati, l'aggiunta di un nuovo
  sito, il rifiuto di una porta non valida, il pulsante di test di
  connessione, l'esportazione/importazione JSON (verificata in andata e
  ritorno) e la finestra di conferma dell'eliminazione (sia annullamento
  sia conferma). Nessun errore JS in console (solo i log di fallimento di
  connessione attesi).

## Struttura

```text
aruaru-web/
├── Cargo.toml            # crate-type = ["cdylib", "rlib"], dipendenze wasm-bindgen/web-sys
├── src/
│   ├── lib.rs             # punto di ingresso, cambio scheda, cablaggio degli eventi
│   ├── dom.rs             # helper comuni per la manipolazione del DOM (download file, ecc.)
│   ├── graphql.rs         # chiamate fetch verso /graphql
│   ├── render.rs          # rendering dei risultati SQL e dell'aggregato del registro, esportazione CSV
│   ├── profiles.rs        # gestione siti (profili di connessione, salvataggio in localStorage, import/export JSON)
│   ├── history.rs         # cronologia delle query SQL (ultime 10, salvate in localStorage)
│   └── shell.rs           # shell HTML (schede, moduli)
├── index.html             # loader che carica pkg/ + CSS
├── pkg/                   # artefatti generati da wasm-bindgen (esclusi da .gitignore, rigenerati a ogni build)
├── scripts/
│   ├── serve.sh            # avvio del server di sviluppo in ascolto su un indirizzo IP qualsiasi
│   ├── gen-vhost.sh         # genera i vhost Nginx/Apache a partire da dominio/IP
│   ├── setup-tls.sh         # ottenimento del certificato Let's Encrypt
│   ├── check-tls.sh         # controllo della scadenza del certificato di un dominio
│   └── check-all-tls.sh     # controllo collettivo della scadenza di tutti i domini registrati
├── deploy/
│   ├── nginx/vhost.conf.template
│   ├── apache/vhost.conf.template
│   ├── systemd/             # insieme dei timer di rinnovo automatico (renew) e monitoraggio automatico (monitor)
│   └── generated/           # output di gen-vhost.sh (escluso da .gitignore)
└── CLAUDE.md
```

## Progetti correlati

- **aruaru-db** (la destinazione a cui si connette questa UI): https://github.com/aon-co-jp/aruaru-db
- **open-runo**: https://github.com/aon-co-jp/open-runo
- **open-web-server**: https://github.com/aon-co-jp/open-web-server
- **poem-cosmo-tauri**: https://github.com/aon-co-jp/poem-cosmo-tauri
- **open-raid-z** (fonte autorevole delle regole di sviluppo): https://github.com/aon-co-jp/open-raid-z
- **rs-to-readme**: https://github.com/aon-co-jp/rs-to-readme

## Licenza

Apache-2.0
