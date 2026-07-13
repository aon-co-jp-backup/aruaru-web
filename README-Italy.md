# aruaru-web

**"Il secondo KUSANAGI" — uno strumento operativo che, dopo il caricamento
dell'app, la avvia da un indirizzo IP e applica facilmente e in automatico
la registrazione del dominio e l'attivazione di HTTPS (Rust → WebAssembly,
senza framework)**

Sul modello di "KUSANAGI", il kit per la realizzazione di server ottimizzati
per WordPress, questo è uno strumento operativo che punta a coprire in un
unico flusso, una volta caricata l'applicazione: **l'avvio da un indirizzo
IP → la semplificazione della registrazione del dominio → l'automazione di
HTTPS**. Può generare automaticamente configurazioni di reverse proxy
(Nginx/Apache) per ottimizzare siti web basati su qualsiasi stack di
backend — WordPress, PHP + Laravel, Python + FastAPI e altri — e dispone di
una schermata "Gestione siti" per registrare, alternare e verificare la
connettività di più destinazioni. **Non dispone di funzionalità di
connessione a un database (DB)** (intenzionalmente fuori ambito).

📖 Altre lingue: [日本語](README-Japan.md) / [English](README-English.md) /
[中文](README-Chinese.md) / [한국어](README-Korea.md) / [Español](README-Spain.md) /
[Français](README-France.md) / [Deutsch](README-Germany.md) / [Italiano](README-Italy.md) /
[Русский](README-Russia.md) / [العربية](README-Arabic.md)

---

## Cosa funziona già

- **Schermata di gestione siti**: consente di registrare più destinazioni di
  deploy (indirizzo IP/dominio/sottodominio/porta/percorso) per qualsiasi
  stack di backend — aruaru-web stesso, WordPress, Laravel, FastAPI e altri —
  salvandole in `localStorage` per selezionarle e verificarne la
  connettività con un clic (equivalente all'elenco siti di KUSANAGI). Ogni
  scheda dispone di un **pulsante "Test di connessione"** (esegue solo una
  semplice verifica di raggiungibilità HTTP senza cambiare il sito
  selezionato), della validazione del numero di porta (1-65535, un valore
  fuori intervallo blocca il salvataggio), di **esportazione/importazione
  JSON** dell'elenco dei siti registrati (per backup o trasferimento verso
  un altro browser) e di una finestra di conferma prima dell'eliminazione.
- **Avvio da indirizzo IP**: `scripts/serve.sh` consente di eseguire il bind
  su qualsiasi indirizzo IP e porta, in locale o su un VPS.
- **Generazione dei vhost, ottimizzazione e configurazione automatica di
  HTTPS**: `scripts/gen-vhost.sh` genera i vhost Nginx/Apache (redirect
  HTTP→HTTPS incluso) a partire dalla combinazione di dominio, IP e stack di
  backend. Supporta 5 stack: `static` (sito statico), `proxy` (reverse proxy
  generico per aruaru-db, open-web-server, open-raid-z e simili, o per
  qualsiasi backend HTTP), `wordpress`, `laravel` e `fastapi`, includendo
  impostazioni di ottimizzazione specifiche per ciascuno stack: compressione
  gzip, cache a lungo termine per gli asset statici, keepalive verso
  l'upstream, regolazione dei buffer FastCGI, ecc.
- **Monitoraggio e rinnovo automatico di HTTPS (TLS)**: `scripts/setup-tls.sh`
  ottiene il certificato Let's Encrypt (certbot), mentre
  `deploy/systemd/install-systemd-units.sh` abilita il "rinnovo automatico
  due volte al giorno" (`aruaru-tls-renew.timer`) e il "monitoraggio della
  scadenza una volta al giorno" (`aruaru-tls-monitor.timer` →
  `scripts/check-all-tls.sh`).
- **Deploy su VPS**: basta eseguire `scripts/deploy-vps.ps1` da Windows
  PowerShell per automatizzare build → upload sul VPS → avvio (per i
  dettagli vedi "Deploy su VPS" più sotto).

## Cosa non funziona ancora (ambito onesto)

- **Non dispone di funzionalità di connessione a un database (DB)**.
  Funzionalità legate a uno specifico prodotto database, come l'esecuzione
  di SQL o le query GraphQL, sono intenzionalmente fuori ambito e non
  verranno implementate in futuro. Anche quando si utilizza un backend
  specifico come aruaru-db, la schermata "Gestione siti" e la generazione
  dei vhost (`--stack=proxy`) possono essere usate come **reverse proxy e
  gestione del deploy generici**, ma non viene fornita alcuna funzionalità
  di query specifica per quel DB.
- Autenticazione, paginazione e retry automatico in caso di errore non sono
  implementati.
- Non viene offerta un'esperienza da app nativa in stile Tauri (solo WASM
  eseguito nel browser).
- **La registrazione effettiva di domini e dei record DNS (operazioni
  presso il registrar) non viene eseguita da questo repository** (comporta
  costi e ricadute su servizi esterni). Ciò che viene automatizzato qui si
  limita, per un dominio già ottenuto, alla "generazione della
  configurazione vhost" e all'"ottenimento, monitoraggio e rinnovo
  automatico del certificato TLS"; la registrazione DNS in sé spetta
  all'utente presso il proprio registrar.
- Anche la stipula effettiva di un contratto VPS (con un provider di server
  in affitto) non viene effettuata da questo repository.

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

## Avvio da un indirizzo IP

```bash
scripts/serve.sh 0.0.0.0 8080        # in ascolto su tutte le interfacce
scripts/serve.sh 192.168.1.50 8080   # in ascolto solo su un indirizzo IP specifico
```

Dopo l'avvio, è possibile verificare il funzionamento **inserendo
direttamente l'indirizzo IP nella barra degli indirizzi del browser**
(esempio: `http://192.168.1.50:8080/index.html`). Il punto di forza è
poter verificare il funzionamento con il solo indirizzo IP, ancora prima
della registrazione del dominio.

## Deploy su VPS (da Windows PowerShell)

Dopo aver noleggiato un VPS, basta eseguire `scripts/deploy-vps.ps1` da
Windows PowerShell per automatizzare build → upload → avvio. Se si
utilizza anche `open-web-server`, è possibile caricarlo contemporaneamente
(questo repository non entra nel merito del contenuto di
`open-web-server`, si limita a specificare il percorso di destinazione
dell'upload).

```powershell
# Caricamento e avvio (solo aruaru-web)
.\scripts\deploy-vps.ps1 -VpsHost 203.0.113.10 -VpsUser root -StartServer

# Caricamento contemporaneo anche di open-web-server (F:\open-runo\open-web-server)
.\scripts\deploy-vps.ps1 -VpsHost 203.0.113.10 -VpsUser root -StartServer `
    -OpenWebServerPath "F:\open-runo\open-web-server"
```

Internamente esegue un'operazione equivalente alla seguente (può essere
eseguita anche manualmente):

```powershell
# 1. Build in locale
cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg `
    target/wasm32-unknown-unknown/debug/aruaru_web.wasm

# 2. Upload sul VPS (client OpenSSH, incluso di serie da Windows 10 1809 in poi/Windows 11)
ssh root@203.0.113.10 "mkdir -p /root/aruaru-web"
scp -r .\index.html .\pkg .\scripts .\deploy .\Cargo.toml .\src `
    root@203.0.113.10:/root/aruaru-web/

# 3. Avvio da indirizzo IP sul VPS
ssh root@203.0.113.10 "cd /root/aruaru-web && bash scripts/serve.sh 0.0.0.0 8080"
```

Dopo l'avvio, **inserire l'indirizzo IP del VPS nella barra degli indirizzi
del browser** (esempio: `http://203.0.113.10:8080/index.html`). Se si
desidera mantenere sempre aggiornata la copia locale, ad esempio in
`F:\open-runo\aruaru-web`, basta eseguire `git pull` su questo repository:

```powershell
cd F:\open-runo\aruaru-web
git fetch origin
git pull origin <nome-branch>
```

**Non è necessario caricare i file per usarlo**: per provarlo solo in
locale senza un VPS, è sufficiente eseguire `scripts/serve.sh` direttamente
in locale come descritto sopra in "Avvio da un indirizzo IP". Da altri
dispositivi sulla stessa rete LAN si può accedere tramite
`http://<indirizzo-IP-del-PC-locale>:8080/`.

## Generazione dei vhost, ottimizzazione e registrazione di dominio/sottodominio

Questo repository non esegue di per sé l'acquisto di domini né la
registrazione di record DNS (comporta operazioni presso il registrar e dei
costi, e resta quindi a carico dell'utente farlo separatamente). Quanto
segue è l'automazione locale (equivalente alla funzione "aggiungi sito" di
KUSANAGI) per generare facilmente, per ogni stack, un reverse proxy con le
impostazioni di ottimizzazione incluse, per un dominio/sottodominio già
ottenuto.

```bash
# aruaru-web stesso (sito statico)
scripts/gen-vhost.sh --stack=static aruaru.example.com 203.0.113.10

# Reverse proxy generico verso aruaru-db, open-web-server, open-raid-z o backend equivalenti, oppure verso qualsiasi altro backend
scripts/gen-vhost.sh --stack=proxy tool.example.com 203.0.113.10 127.0.0.1:9000

# WordPress (specificare il socket/indirizzo di PHP-FPM)
scripts/gen-vhost.sh --stack=wordpress blog.example.com 203.0.113.10 \
  unix:/run/php/php8.3-fpm.sock /var/www/blog

# Laravel (indicare esplicitamente la directory public)
scripts/gen-vhost.sh --stack=laravel app.example.com 203.0.113.10 \
  unix:/run/php/php8.3-fpm.sock /var/www/app/public

# FastAPI (reverse proxy verso il server ASGI, con supporto WebSocket/streaming)
scripts/gen-vhost.sh --stack=fastapi api.example.com 203.0.113.10 127.0.0.1:8000
```

Dopo aver collocato i file di configurazione generati (sotto
`deploy/generated/`, escluso da `.gitignore`) nella directory di
configurazione di Nginx/Apache ed effettuato il reload, si ottiene il
certificato:

```bash
scripts/setup-tls.sh aruaru.example.com admin@example.com /var/www/aruaru.example.com

# Abilita il rinnovo automatico (due volte al giorno) + il monitoraggio automatico (una volta al giorno, rileva la scadenza imminente)
sudo deploy/systemd/install-systemd-units.sh
```

La scadenza dei certificati di tutti i domini registrati può essere
verificata manualmente in qualsiasi momento con
`scripts/check-all-tls.sh`. Registrando la stessa destinazione anche nella
schermata "Gestione siti" della GUI di aruaru-web, sarà possibile
elencarla ed eseguire il test di connessione anche dal lato browser.

## Verifica di funzionamento (eseguita in questo passaggio)

- Sia `cargo check --target wasm32-unknown-unknown` sia `cargo build
  --target wasm32-unknown-unknown` completati con successo (zero warning).
- `wasm-bindgen --target web` con cui sono stati generati
  `pkg/aruaru_web.js` / `pkg/aruaru_web_bg.wasm`, caricando poi
  `index.html` in un browser reale (Chromium, tramite Playwright) e
  verificando concretamente, con operazioni reali: nella schermata
  "Gestione siti", la visualizzazione dei siti registrati, l'aggiunta di
  un nuovo sito, il rifiuto di una porta non valida, il pulsante di test
  di connessione (verificata con successo la raggiungibilità di un server
  HTTP realmente in esecuzione), la finestra di conferma dell'eliminazione
  (sia annullamento sia conferma) e l'esportazione JSON. Nessun errore JS
  in console.
- Sono stati effettivamente installati Nginx 1.24 (standard su Ubuntu
  24.04) e Apache 2.4; i file generati da `gen-vhost.sh` per tutti e 5 gli
  stack (static/proxy/wordpress/laravel/fastapi) sono stati verificati a
  livello di sintassi sia con `nginx -t` sia con `apache2ctl configtest`
  usando un certificato autofirmato; gli stack static/proxy sono stati
  inoltre avviati realmente e verificati funzionalmente con `curl`
  (redirect HTTP→HTTPS, distribuzione di contenuti statici, risposta 502
  tramite reverse proxy).
- L'emissione effettiva di un certificato Let's Encrypt tramite certbot
  (autenticazione ACME) e il funzionamento di `scripts/deploy-vps.ps1` in
  un vero ambiente VPS non sono stati verificati, poiché in questa sessione
  non sono disponibili un dominio pubblico, un VPS reale né un ambiente
  Windows (per i dettagli vedi CLAUDE.md).

## Struttura

```text
aruaru-web/
├── Cargo.toml            # crate-type = ["cdylib", "rlib"], dipendenze wasm-bindgen/web-sys
├── src/
│   ├── lib.rs             # punto di ingresso, cablaggio degli eventi
│   ├── dom.rs             # helper comuni per la manipolazione del DOM (download file, ecc.)
│   ├── profiles.rs        # gestione siti (profili di connessione, salvataggio in localStorage, test di connessione, import/export JSON)
│   └── shell.rs           # shell HTML
├── index.html             # loader che carica pkg/ + CSS
├── pkg/                   # artefatti generati da wasm-bindgen (esclusi da .gitignore, rigenerati a ogni build)
├── scripts/
│   ├── serve.sh            # avvio del server di sviluppo in ascolto su un indirizzo IP qualsiasi
│   ├── deploy-vps.ps1       # build, upload e avvio su VPS da Windows PowerShell
│   ├── gen-vhost.sh         # genera i vhost Nginx/Apache a partire da dominio/IP/stack
│   ├── setup-tls.sh         # ottenimento del certificato Let's Encrypt
│   ├── check-tls.sh         # controllo della scadenza del certificato di un dominio
│   └── check-all-tls.sh     # controllo collettivo della scadenza di tutti i domini registrati
├── deploy/
│   ├── nginx/vhost-{static,proxy,wordpress,laravel,fastapi}.conf.template
│   ├── apache/vhost-{static,proxy,wordpress,laravel,fastapi}.conf.template
│   ├── systemd/             # insieme dei timer di rinnovo automatico (renew) e monitoraggio automatico (monitor)
│   └── generated/           # output di gen-vhost.sh (escluso da .gitignore)
└── CLAUDE.md
```

## Progetti correlati

Poiché questo repository è uno strumento generico di deploy e gestione
operativa indipendente dal database, può essere **utilizzato insieme** ai
seguenti progetti (registrabili nella schermata "Gestione siti" o
utilizzabili come destinazione di reverse proxy con `--stack=proxy`; non
si entra nel merito del contenuto di ciascun repository):

- **aruaru-db**: https://github.com/aon-co-jp/aruaru-db
- **open-runo**: https://github.com/aon-co-jp/open-runo
- **open-web-server**: https://github.com/aon-co-jp/open-web-server
- **poem-cosmo-tauri**: https://github.com/aon-co-jp/poem-cosmo-tauri
- **open-raid-z** (fonte autorevole delle regole di sviluppo): https://github.com/aon-co-jp/open-raid-z
- **rs-to-readme**: https://github.com/aon-co-jp/rs-to-readme

## License

Apache-2.0
