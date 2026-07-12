# aruaru-web

**Interface web minimale pour aruaru-db (Rust → WebAssembly, sans framework)**

Un tableau de bord à onglets qui interroge réellement, depuis le navigateur,
la requête `sql` et la requête `registrySummary` (agrégat du registre des
bases de données supportées) exposées en GraphQL (`/graphql`) par
`aruaru-db` (la base de données distribuée Git-on-SQL), et qui en affiche
les résultats. Au-delà de cela, le projet vise à devenir **« le second
KUSANAGI »** (à l'image de KUSANAGI, le kit de construction de serveurs
optimisés pour WordPress : après avoir déployé l'application, on la
démarre depuis une adresse IP et on applique facilement, de façon
automatisée, l'enregistrement de domaine et le passage en HTTPS). Il
comporte pour cela un onglet « Gestion des sites » permettant d'enregistrer
et de basculer entre plusieurs points de connexion (pour aruaru-web comme
pour d'autres projets), ainsi qu'un ensemble complet couvrant le démarrage
depuis une adresse IP, la génération de vhost, et la
configuration/surveillance/renouvellement automatiques de HTTPS (TLS).

📖 Autres langues : [日本語](README-Japan.md) / [English](README-English.md) /
[中文](README-Chinese.md) / [한국어](README-Korea.md) / [Español](README-Spain.md) /
[Français](README-France.md) / [Deutsch](README-Germany.md) / [Italiano](README-Italy.md) /
[Русский](README-Russia.md) / [العربية](README-Arabic.md)

---

## Ce qui est déjà possible

- Envoie de véritables requêtes GraphQL via `fetch()` vers le point de
  terminaison `/graphql` de `aruaru-graphql` (`aruaru-db/crates/aruaru-graphql`) :
  - `sql(query: String!): QueryResultGql` — exécute n'importe quelle requête
    SQL et affiche `columns`/`rows`/`commandTag` sous forme de tableau
  - `registrySummary: RegistrySummaryGql` — affiche sous forme de cartes
    l'agrégat du registre des bases de données supportées (plus de 150
    entrées)
- Si `aruaru-server` n'est pas démarré ou n'est pas joignable, l'UI affiche
  immédiatement des **données d'exemple ayant exactement la forme du schéma
  réel**, avec une mention explicite indiquant qu'il s'agit d'un
  « échantillon hors ligne » (ce comportement a été vérifié dans un vrai
  navigateur — voir « Vérifications effectuées » ci-dessous).
- **Onglet « Gestion des sites »** : permet d'enregistrer plusieurs points
  de connexion pour aruaru-web ou pour d'autres projets (adresse IP /
  domaine / sous-domaine / port / chemin), sauvegardés dans `localStorage`
  et interchangeables en un clic. Les champs de point de terminaison des
  onglets SQL/Registre suivent automatiquement le site sélectionné. Chaque
  carte dispose d'un **bouton « Tester la connexion »** permettant de
  vérifier la connectivité sans changer de site actif, d'une validation du
  numéro de port (1 à 65535), d'un **export/import JSON** de la liste des
  sites enregistrés (pour la sauvegarde ou le transfert vers un autre
  navigateur), et d'une boîte de dialogue de confirmation avant suppression.
- **Ergonomie de l'onglet SQL** : **historique des requêtes** sur les 10
  dernières exécutions (rechargement en un clic, infobulle au survol
  affichant le texte complet), **raccourci d'exécution Ctrl+Entrée /
  Cmd+Entrée**, **export CSV** des résultats, désactivation du bouton
  pendant l'exécution, et tableau de résultats avec compteur de lignes,
  défilement et en-tête figé (sticky).
- **Configuration, surveillance et renouvellement automatiques de HTTPS
  (TLS)** : `scripts/gen-vhost.sh` génère un vhost Nginx/Apache (avec
  redirection HTTP→HTTPS incluse), `scripts/setup-tls.sh` obtient un
  certificat Let's Encrypt (certbot), et
  `deploy/systemd/install-systemd-units.sh` active un « renouvellement
  automatique deux fois par jour (`aruaru-tls-renew.timer`) » ainsi qu'une
  « surveillance de l'expiration une fois par jour
  (`aruaru-tls-monitor.timer` → `scripts/check-all-tls.sh`) ». Voir
  « HTTPS et enregistrement de domaine/sous-domaine » pour plus de détails.

## Ce qui n'est pas encore possible (limites assumées)

- **Les requêtes de gestion de versions d'aruaru-db (branches, journal des
  commits, diff, etc.) ainsi que la récupération de la liste détaillée du
  registre sont volontairement hors périmètre.** L'objectif de ce dépôt est
  un outil d'exploitation dans l'esprit du « second KUSANAGI » (démarrage
  depuis une adresse IP, simplification de l'enregistrement de domaine,
  automatisation de HTTPS), et non l'extension de l'interface
  d'administration/requêtage d'aruaru-db au-delà des fonctions minimales de
  connexion que sont l'exécution SQL et l'agrégat du registre — aucune
  extension de ce type n'est prévue pour la suite.
- Les mutations GraphQL (création de branche, fusion, crawl du registre,
  etc.) ne sont pas implémentées.
- L'authentification, la pagination et les nouvelles tentatives
  automatiques en cas d'erreur ne sont pas implémentées.
- Aucune expérience d'application native façon Tauri n'est proposée
  (uniquement du WASM exécuté dans le navigateur).
- **L'achat réel d'un domaine et l'enregistrement des enregistrements DNS
  (opérations chez un bureau d'enregistrement) ne sont pas effectués depuis
  ce dépôt** (car cela implique des coûts et des actions chez un service
  externe). Ce qui est automatisé ici s'arrête à la « génération de
  configuration vhost » et à « l'obtention, la surveillance et le
  renouvellement automatique du certificat TLS » pour un domaine déjà
  acquis ; l'enregistrement DNS lui-même reste à la charge de l'utilisateur,
  chez son bureau d'enregistrement.

## Méthode de compilation

Aucun usage de Node.js, npm ou TypeScript : tout repose uniquement sur la
chaîne d'outils Rust.

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

Pour tester avec une véritable instance de `aruaru-db` en cours d'exécution :

```bash
cd ../aruaru-db
cargo run -p aruaru-server -- --data ./data --raft-id 1   # le GraphQL démarre sur :4000
```

## Démarrer depuis une adresse IP

```bash
scripts/serve.sh 0.0.0.0 8080        # écoute sur toutes les interfaces
scripts/serve.sh 192.168.1.50 8080   # écoute uniquement sur une adresse IP spécifique
```

## HTTPS et enregistrement de domaine/sous-domaine

Ce dépôt n'effectue lui-même ni l'achat de domaine ni l'enregistrement DNS
(ces opérations, effectuées chez un bureau d'enregistrement et impliquant
des coûts, restent à la charge de l'utilisateur, qui les réalise
séparément). Ce qui suit est une automatisation locale (l'équivalent de
« Ajouter un site » chez KUSANAGI) permettant d'attribuer facilement un
domaine/sous-domaine déjà acquis à aruaru-web ou à un autre projet.

```bash
# 1. Générer le vhost (Nginx/Apache, avec redirection HTTP→HTTPS incluse) à partir du domaine + IP + backend
scripts/gen-vhost.sh aruaru.example.com 203.0.113.10 127.0.0.1:4000
# De même pour un sous-domaine à un autre usage (il suffit de changer UPSTREAM/WEBROOT)
scripts/gen-vhost.sh tool.example.com 203.0.113.10 127.0.0.1:9000 /var/www/tool

# 2. Déployer le fichier de configuration généré puis recharger le serveur (sous deploy/generated/, exclu par .gitignore)

# 3. Obtenir le certificat TLS (Let's Encrypt / certbot)
scripts/setup-tls.sh aruaru.example.com admin@example.com /var/www/aruaru.example.com

# 4. Activer le renouvellement automatique (2 fois par jour) + la surveillance automatique (1 fois par jour, détection d'expiration imminente)
sudo deploy/systemd/install-systemd-units.sh
```

La date d'expiration des certificats de tous les domaines enregistrés peut
à tout moment être vérifiée manuellement avec `scripts/check-all-tls.sh`.
En enregistrant les mêmes points de connexion dans l'onglet « Gestion des
sites » de l'interface graphique d'aruaru-web, on peut faire correspondre
ces domaines avec le basculement de point de connexion effectué depuis le
navigateur.

## Vérifications effectuées (lors de cette itération)

- `cargo check --target wasm32-unknown-unknown` et
  `cargo build --target wasm32-unknown-unknown` réussissent tous les deux
  (zéro avertissement).
- `wasm-bindgen --target web` a généré `pkg/aruaru_web.js` /
  `pkg/aruaru_web_bg.wasm` ; `index.html` a été chargé dans un vrai
  navigateur (Chromium, via Playwright), et les opérations suivantes ont
  été effectivement testées : changement d'onglet, exécution SQL →
  affichage du repli hors ligne, enregistrement dans l'historique des
  requêtes → rechargement → infobulle au survol, raccourci Ctrl+Entrée,
  export CSV (déclenchement réel du téléchargement), agrégation du
  registre, affichage des sites enregistrés dans l'onglet « Gestion des
  sites », ajout d'un nouveau site, rejet d'un numéro de port invalide,
  bouton de test de connexion, export/import JSON (aller-retour vérifié),
  boîte de dialogue de confirmation de suppression (annulation et
  validation testées toutes les deux). Aucune erreur JS dans la console
  (seuls les journaux d'échec de connexion attendus apparaissent).
- Nginx 1.24 (la version standard d'Ubuntu 24.04), Apache 2.4 et certbot
  ont été réellement installés ; les fichiers générés par
  `scripts/gen-vhost.sh` ont été démarrés avec un certificat auto-signé et
  vérifiés via `curl` (redirection HTTP→HTTPS, chemin du challenge ACME,
  proxy inverse vers `/graphql`), `scripts/check-tls.sh` a été exécuté
  contre un vrai serveur HTTPS pour confirmer les trois états WARN /
  healthy / ERROR, et `deploy/systemd/*` a été vérifié avec
  `systemd-analyze verify` (zéro erreur). Ce processus a permis de
  découvrir et corriger un véritable bug dans le modèle de vhost Nginx
  (`http2 on;` provoque une erreur de syntaxe sous Nginx 1.24). En
  revanche, l'émission réelle d'un certificat Let's Encrypt par certbot
  (authentification ACME) n'a pas pu être vérifiée, faute de domaine
  public disponible et en raison d'une incompatibilité d'ABI Python dans
  l'environnement de test (voir CLAUDE.md pour le détail).

## Structure

```text
aruaru-web/
├── Cargo.toml            # crate-type = ["cdylib", "rlib"], dépendances wasm-bindgen/web-sys
├── src/
│   ├── lib.rs             # point d'entrée, changement d'onglet, câblage des événements
│   ├── dom.rs             # fonctions utilitaires communes pour le DOM (téléchargement de fichier, etc.)
│   ├── graphql.rs         # appels fetch vers /graphql
│   ├── render.rs          # rendu des résultats SQL et de l'agrégat du registre, export CSV
│   ├── profiles.rs        # gestion des sites (profils de connexion, sauvegarde localStorage, import/export JSON)
│   ├── history.rs         # historique des requêtes SQL (10 dernières, sauvegarde localStorage)
│   └── shell.rs           # squelette HTML (onglets, formulaires)
├── index.html             # chargeur de pkg/ + CSS
├── pkg/                   # fichiers générés par wasm-bindgen (exclu par .gitignore, régénéré à la compilation)
├── scripts/
│   ├── serve.sh            # démarrage d'un serveur de développement servant depuis une adresse IP quelconque
│   ├── gen-vhost.sh         # génère un vhost Nginx/Apache à partir d'un domaine/IP
│   ├── setup-tls.sh         # obtention du certificat Let's Encrypt
│   ├── check-tls.sh         # vérification de la date d'expiration du certificat d'un domaine
│   └── check-all-tls.sh     # vérification groupée de l'expiration pour tous les domaines enregistrés
├── deploy/
│   ├── nginx/vhost.conf.template
│   ├── apache/vhost.conf.template
│   ├── systemd/             # ensemble des timers de renouvellement automatique (renew) et de surveillance automatique (monitor)
│   └── generated/           # sortie de gen-vhost.sh (exclu par .gitignore)
└── CLAUDE.md
```

## Projets liés

- **aruaru-db** (le projet auquel cette UI se connecte) : https://github.com/aon-co-jp/aruaru-db
- **open-runo** : https://github.com/aon-co-jp/open-runo
- **open-web-server** : https://github.com/aon-co-jp/open-web-server
- **poem-cosmo-tauri** : https://github.com/aon-co-jp/poem-cosmo-tauri
- **open-raid-z** (référence normative des règles de développement) : https://github.com/aon-co-jp/open-raid-z
- **rs-to-readme** : https://github.com/aon-co-jp/rs-to-readme

## License

Apache-2.0
