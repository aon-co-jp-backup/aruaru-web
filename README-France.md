# aruaru-web

## Une UI web minimale pour aruaru-db (Rust → WebAssembly, sans framework)

Première version de démarrage d'une UI de navigateur qui appelle réellement
le point de terminaison GraphQL (`/graphql`) exposé par `aruaru-db` (la base
de données distribuée Git-on-SQL) — la requête `sql` et la requête
`registrySummary` (agrégat du registre des bases de données supportées) —
et affiche les résultats. **Ce n'est pas encore une console d'administration
complète** — le périmètre est volontairement limité à un champ de saisie
SQL, un bouton de résumé du registre et un tableau de résultats.

📖 Autres langues : [日本語](README-Japan.md) / [English](README-English.md) /
[中文](README-Chinese.md) / [한국어](README-Korea.md) / [Español](README-Spain.md) /
[Français](README-France.md) / [Deutsch](README-Germany.md) / [Italiano](README-Italy.md) /
[Русский](README-Russia.md) / [العربية](README-Arabic.md)

## État actuel

- Envoie de vraies requêtes GraphQL via `fetch()` au point de terminaison
  `/graphql` de `aruaru-graphql` (`sql(query: String!)`, `registrySummary`).
- Si `aruaru-server` n'est pas démarré ou injoignable, la page affiche des
  **données d'exemple ayant exactement la forme du schéma réel**, avec une
  mention claire indiquant qu'il s'agit d'un échantillon hors ligne (vérifié
  dans un vrai navigateur).
- Les mutations GraphQL, l'authentification et la pagination ne sont pas
  encore implémentées.

## Démarrage rapide

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.126
cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg \
  target/wasm32-unknown-unknown/debug/aruaru_web.wasm
python -m http.server 8080   # puis ouvrir http://localhost:8080/index.html
```

## Structure

`Cargo.toml` (`cdylib`/`rlib`, dépendances wasm-bindgen/web-sys),
`src/lib.rs` (unique fichier source : construction du DOM, fetch, rendu de
la réponse GraphQL), `index.html` (chargeur léger de `pkg/`).

## Licence

Apache-2.0
