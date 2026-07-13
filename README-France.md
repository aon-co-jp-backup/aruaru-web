# aruaru-web

**« Le second KUSANAGI » — un outil d'exploitation qui, une fois l'application
uploadée, démarre depuis une adresse IP et permet d'appliquer facilement et
automatiquement l'enregistrement de domaine et la mise en HTTPS (Rust →
WebAssembly, sans framework)**

À la manière de « KUSANAGI », le kit de construction de serveur
d'accélération pour WordPress, cet outil d'exploitation vise à couvrir de
bout en bout, une fois l'application uploadée, le parcours **démarrage
depuis une adresse IP → simplification de l'enregistrement de domaine →
automatisation du HTTPS**. Il peut générer automatiquement des
configurations de reverse proxy (Nginx/Apache) pour accélérer des sites web
reposant sur n'importe quelle pile technique backend — WordPress, PHP +
Laravel, Python + FastAPI, etc. — et dispose d'un écran de « gestion des
sites » permettant d'enregistrer plusieurs destinations de connexion, d'y
basculer et de tester leur joignabilité.
**Il ne dispose d'aucune fonctionnalité de connexion à une base de données**
(hors périmètre de manière délibérée).

📖 Autres langues : [日本語](README-Japan.md) / [English](README-English.md) /
[中文](README-Chinese.md) / [한국어](README-Korea.md) / [Español](README-Spain.md) /
[Français](README-France.md) / [Deutsch](README-Germany.md) / [Italiano](README-Italy.md) /
[Русский](README-Russia.md) / [العربية](README-Arabic.md)

---

## Ce qui est déjà possible

- **Écran de gestion des sites** : permet d'enregistrer plusieurs
  destinations de déploiement (adresse IP/domaine/sous-domaine/port/chemin)
  pour n'importe quelle pile backend — aruaru-web lui-même, WordPress,
  Laravel, FastAPI, etc. —, de les sauvegarder dans `localStorage` et de les
  sélectionner ou tester en un clic (l'équivalent de la liste de sites de
  KUSANAGI). Chaque carte propose un **bouton « Test de connexion »**
  (effectue une simple vérification de joignabilité HTTP sans changer le
  site actuellement sélectionné), une validation de la saisie du numéro de
  port (1 à 65535), l'**export/import JSON** de la liste des sites
  enregistrés (pour sauvegarde ou transfert vers un autre navigateur), et
  une boîte de dialogue de confirmation avant suppression.
- **Démarrage depuis une adresse IP** : `scripts/serve.sh` permet de
  publier le service en se liant à n'importe quelle adresse IP/port, en
  local ou sur un VPS.
- **Génération de vhost, accélération et automatisation du HTTPS** :
  `scripts/gen-vhost.sh` génère, à partir d'une combinaison
  domaine/IP/pile backend, un vhost Nginx/Apache (redirection HTTP→HTTPS
  incluse). Cinq piles sont prises en charge : `static` (site statique),
  `proxy` (reverse proxy générique pour aruaru-db, open-web-server,
  open-raid-z ou tout autre backend HTTP), `wordpress`, `laravel` et
  `fastapi` ; chacune inclut des réglages d'accélération adaptés
  (compression gzip, mise en cache longue durée des assets statiques,
  keepalive vers l'upstream, ajustement des tampons FastCGI, etc.).
- **Surveillance et renouvellement automatiques du HTTPS (TLS)** :
  `scripts/setup-tls.sh` obtient un certificat Let's Encrypt (certbot), et
  `deploy/systemd/install-systemd-units.sh` active un « renouvellement
  automatique deux fois par jour » (`aruaru-tls-renew.timer`) ainsi qu'une
  « surveillance de l'expiration une fois par jour »
  (`aruaru-tls-monitor.timer` → `scripts/check-all-tls.sh`).
- **Déploiement sur un VPS** : il suffit d'exécuter
  `scripts/deploy-vps.ps1` depuis Windows PowerShell pour automatiser la
  compilation, l'upload vers le VPS et le démarrage (voir la section
  « Déploiement sur un VPS » ci-dessous pour le détail).

## Ce qui n'est pas encore possible (limites honnêtes)

- **Aucune fonctionnalité de connexion à une base de données**. L'exécution
  de requêtes SQL, les requêtes GraphQL et toute autre fonctionnalité liée
  à un produit de base de données spécifique sont délibérément hors
  périmètre et ne seront pas implémentées à l'avenir. Même en cas
  d'utilisation d'un backend spécifique comme aruaru-db, l'écran « gestion
  des sites » et la génération de vhost (`--stack=proxy`) peuvent servir de
  **reverse proxy et de gestion de déploiement génériques**, mais aucune
  fonctionnalité de requête propre à cette base de données n'est fournie.
- L'authentification, la pagination et les nouvelles tentatives
  automatiques en cas d'erreur ne sont pas implémentées.
- Aucune expérience d'application native de type Tauri n'est proposée
  (uniquement du WASM fonctionnant dans le navigateur).
- **L'enregistrement effectif d'un domaine et des enregistrements DNS
  (opérations chez le registrar) ne sont pas réalisés par ce dépôt** (car
  cela implique des coûts et des effets sur des services externes). Ce qui
  est automatisé ici s'arrête à la « génération de la configuration vhost »
  et à « l'obtention/la surveillance/le renouvellement automatique du
  certificat TLS » pour un domaine déjà enregistré ; l'enregistrement DNS
  lui-même reste à la charge de l'utilisateur chez son registrar.
- La souscription effective d'un VPS (contrat avec un hébergeur) n'est pas
  non plus réalisée par ce dépôt.

## Méthode de compilation

Ni Node.js, ni npm, ni TypeScript ne sont utilisés. Tout se fait uniquement
avec la chaîne d'outils Rust.

```bash
rustup target add wasm32-unknown-unknown        # première fois seulement
cargo install wasm-bindgen-cli --version 0.2.126 # première fois seulement (à faire correspondre à la version de Cargo.lock)

cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg \
  target/wasm32-unknown-unknown/debug/aruaru_web.wasm

# servir avec un serveur statique quelconque et ouvrir (exemple ci-dessous)
python -m http.server 8080
# ouvrir http://localhost:8080/index.html dans le navigateur
```

## Démarrer depuis une adresse IP

```bash
scripts/serve.sh 0.0.0.0 8080        # écoute sur toutes les interfaces
scripts/serve.sh 192.168.1.50 8080   # écoute uniquement sur une adresse IP spécifique
```

Une fois démarré, il suffit de **saisir directement l'adresse IP dans la
barre d'URL du navigateur** pour vérifier le fonctionnement (exemple :
`http://192.168.1.50:8080/index.html`). L'intérêt est de pouvoir vérifier
le fonctionnement avec la seule adresse IP, avant même tout enregistrement
de domaine.

## Déploiement sur un VPS (depuis Windows PowerShell)

Une fois un VPS loué, il suffit d'exécuter `scripts/deploy-vps.ps1` depuis
Windows PowerShell pour automatiser la compilation, l'upload et le
démarrage. En cas d'utilisation conjointe d'`open-web-server`, celui-ci
peut être uploadé en même temps (ce dépôt ne touche pas au contenu
d'`open-web-server`, il se contente d'indiquer le chemin de destination de
l'upload).

```powershell
# Pour uploader et démarrer directement (aruaru-web seul)
.\scripts\deploy-vps.ps1 -VpsHost 203.0.113.10 -VpsUser root -StartServer

# Pour uploader en même temps open-web-server (F:\open-runo\open-web-server)
.\scripts\deploy-vps.ps1 -VpsHost 203.0.113.10 -VpsUser root -StartServer `
    -OpenWebServerPath "F:\open-runo\open-web-server"
```

En interne, cela revient à effectuer ce qui suit (l'exécution manuelle est
également possible) :

```powershell
# 1. Compilation locale
cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg `
    target/wasm32-unknown-unknown/debug/aruaru_web.wasm

# 2. Upload vers le VPS (client OpenSSH, inclus par défaut avec Windows 10 1809+/11)
ssh root@203.0.113.10 "mkdir -p /root/aruaru-web"
scp -r .\index.html .\pkg .\scripts .\deploy .\Cargo.toml .\src `
    root@203.0.113.10:/root/aruaru-web/

# 3. Démarrage depuis une adresse IP sur le VPS
ssh root@203.0.113.10 "cd /root/aruaru-web && bash scripts/serve.sh 0.0.0.0 8080"
```

Une fois démarré, **saisissez l'adresse IP du VPS dans la barre d'URL du
navigateur** (exemple : `http://203.0.113.10:8080/index.html`). Pour garder
en permanence à jour une copie locale telle que `F:\open-runo\aruaru-web`,
il suffit de faire un `git pull` de ce dépôt :

```powershell
cd F:\open-runo\aruaru-web
git fetch origin
git pull origin <nom-de-la-branche>
```

**Utilisable sans upload** : pour essayer uniquement en local sans VPS, il
suffit d'exécuter `scripts/serve.sh` en local, comme décrit ci-dessus dans
« Démarrer depuis une adresse IP ». Les autres postes du même réseau local
peuvent y accéder via `http://<adresse IP du PC local>:8080/`.

## Génération de vhost, accélération et enregistrement de domaine/sous-domaine

Ce dépôt lui-même n'effectue ni l'obtention de domaine ni l'enregistrement
DNS (ces opérations, réalisées chez le registrar et impliquant des coûts,
restent à la charge de l'utilisateur). Ce qui suit est l'automatisation
locale (l'équivalent de « l'ajout de site » dans KUSANAGI) permettant de
générer facilement, pour un domaine/sous-domaine déjà enregistré, un
reverse proxy incluant des réglages d'accélération, adapté à chaque pile
technique.

```bash
# aruaru-web lui-même (site statique)
scripts/gen-vhost.sh --stack=static aruaru.example.com 203.0.113.10

# Reverse proxy générique pour aruaru-db, open-web-server, la famille open-raid-z ou tout autre backend
scripts/gen-vhost.sh --stack=proxy tool.example.com 203.0.113.10 127.0.0.1:9000

# WordPress (indiquer le socket/adresse de PHP-FPM)
scripts/gen-vhost.sh --stack=wordpress blog.example.com 203.0.113.10 \
  unix:/run/php/php8.3-fpm.sock /var/www/blog

# Laravel (indiquer explicitement le répertoire public)
scripts/gen-vhost.sh --stack=laravel app.example.com 203.0.113.10 \
  unix:/run/php/php8.3-fpm.sock /var/www/app/public

# FastAPI (reverse proxy vers un serveur ASGI, avec support WebSocket/streaming)
scripts/gen-vhost.sh --stack=fastapi api.example.com 203.0.113.10 127.0.0.1:8000
```

Une fois le fichier de configuration généré (sous `deploy/generated/`,
exclu via `.gitignore`) placé dans le répertoire de configuration de
Nginx/Apache et le service rechargé, on obtient le certificat :

```bash
scripts/setup-tls.sh aruaru.example.com admin@example.com /var/www/aruaru.example.com

# Active le renouvellement automatique (2x/jour) + la surveillance automatique (1x/jour, détection d'expiration imminente)
sudo deploy/systemd/install-systemd-units.sh
```

La date d'expiration des certificats de tous les domaines enregistrés peut
à tout moment être vérifiée manuellement via `scripts/check-all-tls.sh`. En
enregistrant les mêmes destinations dans l'écran « gestion des sites » de
l'interface graphique d'aruaru-web, on peut aussi les lister et tester leur
connectivité depuis le navigateur.

## Vérifications effectuées (au cours de cette passe)

- `cargo check --target wasm32-unknown-unknown` et
  `cargo build --target wasm32-unknown-unknown` réussissent tous les deux
  (zéro avertissement).
- `wasm-bindgen --target web` génère `pkg/aruaru_web.js` /
  `pkg/aruaru_web_bg.wasm` ; `index.html` a été chargé dans un vrai
  navigateur (Chromium, via Playwright) et les opérations suivantes ont été
  réellement testées : affichage des sites enregistrés dans l'écran de
  gestion des sites, ajout d'un nouveau site, rejet d'une saisie de port
  invalide, bouton de test de connexion (jusqu'à la vérification réussie de
  la joignabilité d'un serveur HTTP réellement en fonctionnement), boîte de
  dialogue de confirmation de suppression (annulation et exécution), export
  JSON. Aucune erreur JS dans la console.
- Nginx 1.24 (standard d'Ubuntu 24.04) et Apache 2.4 ont été réellement
  installés ; les productions de `gen-vhost.sh` pour les 5 piles
  (static/proxy/wordpress/laravel/fastapi) ont été validées
  syntaxiquement avec un certificat auto-signé via `nginx -t` et
  `apache2ctl configtest`, et les piles static/proxy ont été réellement
  démarrées puis vérifiées fonctionnellement via `curl` (redirection
  HTTP→HTTPS, service statique, réponse 502 via le reverse proxy).
- L'émission effective d'un certificat Let's Encrypt par certbot
  (validation ACME) ainsi que le fonctionnement de
  `scripts/deploy-vps.ps1` sur un véritable environnement VPS n'ont pas pu
  être vérifiés, cette session ne disposant ni d'un domaine public, ni
  d'un VPS réel, ni d'un environnement Windows (voir CLAUDE.md pour le
  détail).

## Arborescence

```text
aruaru-web/
├── Cargo.toml            # crate-type = ["cdylib", "rlib"], dépendances wasm-bindgen/web-sys
├── src/
│   ├── lib.rs             # Point d'entrée, câblage des événements
│   ├── dom.rs             # Fonctions utilitaires DOM communes (téléchargement de fichiers, etc.)
│   ├── profiles.rs        # Gestion des sites (profils de connexion, stockage localStorage, test de connexion, import/export JSON)
│   └── shell.rs           # Coquille HTML
├── index.html             # Loader qui charge pkg/ + CSS
├── pkg/                   # Sortie de wasm-bindgen (exclue via .gitignore, régénérée à la compilation)
├── scripts/
│   ├── serve.sh            # Démarrage du serveur de développement sur une adresse IP quelconque
│   ├── deploy-vps.ps1       # Compilation, upload et démarrage vers un VPS depuis Windows PowerShell
│   ├── gen-vhost.sh         # Génère un vhost Nginx/Apache à partir du domaine/IP/pile technique
│   ├── setup-tls.sh         # Obtention du certificat Let's Encrypt
│   ├── check-tls.sh         # Vérification de la date d'expiration du certificat pour un domaine
│   └── check-all-tls.sh     # Vérification groupée des dates d'expiration de tous les domaines enregistrés
├── deploy/
│   ├── nginx/vhost-{static,proxy,wordpress,laravel,fastapi}.conf.template
│   ├── apache/vhost-{static,proxy,wordpress,laravel,fastapi}.conf.template
│   ├── systemd/             # Ensemble des timers de renouvellement (renew) et de surveillance (monitor) automatiques
│   └── generated/           # Sortie de gen-vhost.sh (exclue via .gitignore)
└── CLAUDE.md
```

## Projets associés

Ce dépôt étant un outil de déploiement et d'exploitation générique,
indépendant de toute base de données, il peut être **utilisé conjointement**
avec les projets suivants (en les enregistrant dans l'écran « gestion des
sites » ou en les ciblant comme reverse proxy via `--stack=proxy` ; le
contenu de chacun de ces dépôts n'est pas concerné par ce dépôt) :

- **aruaru-db** : https://github.com/aon-co-jp/aruaru-db
- **open-runo** : https://github.com/aon-co-jp/open-runo
- **open-web-server** : https://github.com/aon-co-jp/open-web-server
- **poem-cosmo-tauri** : https://github.com/aon-co-jp/poem-cosmo-tauri
- **open-raid-z** (référence normative des règles de développement) : https://github.com/aon-co-jp/open-raid-z
- **rs-to-readme** : https://github.com/aon-co-jp/rs-to-readme

## License

Apache-2.0
