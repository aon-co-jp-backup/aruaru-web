# aruaru-web

## Una UI web mínima para aruaru-db (Rust → WebAssembly, sin framework)

Esta es la primera versión de arranque de una UI de navegador que realmente
llama al endpoint GraphQL (`/graphql`) que expone `aruaru-db` (la base de
datos distribuida Git-on-SQL): la consulta `sql` y la consulta
`registrySummary` (resumen del registro de bases de datos soportadas), y
renderiza los resultados. **Todavía no es una consola de administración
completa** — está deliberadamente limitada a un cuadro de entrada SQL, un
botón de resumen del registro y una tabla de resultados.

📖 Otros idiomas: [日本語](README-Japan.md) / [English](README-English.md) /
[中文](README-Chinese.md) / [한국어](README-Korea.md) / [Español](README-Spain.md) /
[Français](README-France.md) / [Deutsch](README-Germany.md) / [Italiano](README-Italy.md) /
[Русский](README-Russia.md) / [العربية](README-Arabic.md)

## Estado actual

- Envía peticiones GraphQL reales con `fetch()` al endpoint `/graphql` de
  `aruaru-graphql` (`sql(query: String!)`, `registrySummary`).
- Si `aruaru-server` no está en ejecución o no se puede alcanzar, la página
  renderiza **datos de ejemplo con la misma forma que el esquema real** y lo
  indica claramente como muestra sin conexión (verificado en un navegador
  real).
- Las mutaciones GraphQL, la autenticación y la paginación aún no están
  implementadas.

## Inicio rápido

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.126
cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg \
  target/wasm32-unknown-unknown/debug/aruaru_web.wasm
python -m http.server 8080   # abrir http://localhost:8080/index.html
```

## Estructura

`Cargo.toml` (`cdylib`/`rlib`, dependencias wasm-bindgen/web-sys),
`src/lib.rs` (único archivo fuente: construcción del DOM, fetch, renderizado
de la respuesta GraphQL), `index.html` (cargador ligero de `pkg/`).

## Licencia

Apache-2.0
