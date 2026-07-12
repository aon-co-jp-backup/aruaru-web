# aruaru-web

**UI web mínima para aruaru-db (Rust → WebAssembly, sin framework)**

Es un panel de control con pestañas que llama de verdad, desde el navegador,
a la consulta `sql` y a la consulta `registrySummary` (resumen del registro
de bases de datos soportadas) que `aruaru-db` (la base de datos distribuida
Git-on-SQL) expone mediante GraphQL (`/graphql`), y muestra los resultados.
Además de la ejecución de SQL y el resumen del registro, incluye una
**pestaña de "Gestión de sitios"** que permite registrar y alternar entre
**varios destinos de conexión (para aruaru-web y para otros proyectos)**, de
forma similar al listado de sitios de KUSANAGI.

📖 Otros idiomas: [日本語](README-Japan.md) / [English](README-English.md) /
[中文](README-Chinese.md) / [한국어](README-Korea.md) / [Español](README-Spain.md) /
[Français](README-France.md) / [Deutsch](README-Germany.md) / [Italiano](README-Italy.md) /
[Русский](README-Russia.md) / [العربية](README-Arabic.md)

---

## Qué se puede hacer ahora

- Envía peticiones GraphQL reales con `fetch()` al endpoint `/graphql` de
  `aruaru-graphql` (`aruaru-db/crates/aruaru-graphql`):
  - `sql(query: String!): QueryResultGql` — ejecuta cualquier SQL y muestra
    `columns`/`rows`/`commandTag` en una tabla.
  - `registrySummary: RegistrySummaryGql` — muestra en tarjetas el resumen
    del registro de bases de datos soportadas (más de 150).
  - `registry: [DbEntryGql!]!` — muestra en una tabla el **listado** del
    registro de bases de datos soportadas (nombre/categoría/compatibilidad
    de protocolo/estado/posición/puntuación/fecha de actualización).
- **Pestaña de gestión de versiones**: permite ejecutar `currentBranch`/
  `branches` (listado de ramas y rama actual), `log(limit)` (historial de
  commits) y `diff(from, to)` (número de líneas añadidas/eliminadas/
  modificadas entre ramas). Todo ello se basa en el esquema real
  (`VcsQuery`) de `aruaru-db/crates/aruaru-graphql`.
- Si `aruaru-server` no está en ejecución o no se puede alcanzar, se
  renderiza al instante **datos de muestra con la misma forma que el
  esquema real**, dejando claro que se trata de una "muestra sin conexión"
  (este comportamiento está verificado en un navegador real — ver
  "Verificación de funcionamiento" más abajo).
- **Pestaña de gestión de sitios**: permite registrar varios destinos de
  conexión (para aruaru-web y para otros proyectos) — dirección IP/dominio/
  subdominio/puerto/ruta —, guardarlos en `localStorage` y cambiar entre
  ellos con un solo clic. El campo de endpoint de las pestañas SQL/Registro
  sigue automáticamente al sitio seleccionado. Cada tarjeta incluye un
  **botón de "Prueba de conexión"** que verifica la conectividad sin cambiar
  el sitio activo, validación del número de puerto (1-65535), **exportación/
  importación en JSON** de la lista de sitios registrados (para copias de
  seguridad o para llevarlos a otro navegador) y un diálogo de confirmación
  antes de eliminar.
- **Comodidad de la pestaña SQL**: **historial de consultas** de las
  últimas 10 (clic para recargar, pasar el cursor por encima para ver el
  texto completo), **atajo de ejecución con Ctrl+Enter / Cmd+Enter**,
  **exportación a CSV** del resultado de la ejecución, desactivación del
  botón mientras se ejecuta, y una tabla de resultados con contador de
  filas, desplazable y con encabezado fijo.
- **Configuración, monitorización y renovación automática de HTTPS (TLS)**:
  `scripts/gen-vhost.sh` genera el vhost de Nginx/Apache (con redirección
  HTTP→HTTPS incluida), `scripts/setup-tls.sh` obtiene el certificado de
  Let's Encrypt (certbot), y `deploy/systemd/install-systemd-units.sh`
  activa la "renovación automática dos veces al día"
  (`aruaru-tls-renew.timer`) y la "monitorización de caducidad una vez al
  día" (`aruaru-tls-monitor.timer` → `scripts/check-all-tls.sh`). Para más
  detalles, ver "Registro de HTTPS y de dominios/subdominios".

## Qué no se puede hacer todavía (alcance honesto)

- Las mutaciones GraphQL (creación de ramas, fusión, crawl del registro,
  etc.) no están implementadas.
- La autenticación, la paginación y el reintento automático en caso de
  error no están implementados.
- No ofrece una experiencia de aplicación nativa como Tauri (solo WASM
  ejecutándose en el navegador).
- **La adquisición real de dominios y el registro de registros DNS
  (operaciones en el registrador) no se realizan desde este repositorio**
  (porque implican costes y cambios en servicios externos). Lo que se
  automatiza aquí llega hasta la "generación de configuración de vhost" y
  la "obtención, monitorización y renovación automática del certificado
  TLS" para un dominio ya adquirido; el registro DNS en sí lo realiza el
  usuario en su registrador.

## Cómo compilar

No se usan Node.js, npm ni TypeScript. Todo se completa únicamente con la
cadena de herramientas de Rust.

```bash
rustup target add wasm32-unknown-unknown        # solo la primera vez
cargo install wasm-bindgen-cli --version 0.2.126 # solo la primera vez (debe coincidir con la versión de Cargo.lock)

cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg \
  target/wasm32-unknown-unknown/debug/aruaru_web.wasm

# Servir con un servidor estático cualquiera y abrir (por ejemplo:)
python -m http.server 8080
# Abrir http://localhost:8080/index.html en el navegador
```

Para probarlo con `aruaru-db` funcionando de verdad:

```bash
cd ../aruaru-db
cargo run -p aruaru-server -- --data ./data --raft-id 1   # levanta GraphQL en :4000
```

## Iniciar desde una dirección IP

```bash
scripts/serve.sh 0.0.0.0 8080        # escuchar en todas las interfaces
scripts/serve.sh 192.168.1.50 8080   # escuchar solo en una dirección IP concreta
```

## Registro de HTTPS y de dominios/subdominios

Este propio repositorio no realiza la adquisición de dominios ni el
registro de registros DNS (esas operaciones, y su coste, las realiza el
usuario por separado en su registrador). Lo siguiente es la automatización
local (equivalente a "añadir sitio" en KUSANAGI) para dar de alta
fácilmente, para aruaru-web o para otros proyectos, un dominio/subdominio
ya adquirido.

```bash
# 1. Generar el vhost (Nginx/Apache, con redirección HTTP→HTTPS incluida) a partir de dominio+IP+backend
scripts/gen-vhost.sh aruaru.example.com 203.0.113.10 127.0.0.1:4000
# Igual para un subdominio de otro uso (basta con cambiar UPSTREAM/WEBROOT)
scripts/gen-vhost.sh tool.example.com 203.0.113.10 127.0.0.1:9000 /var/www/tool

# 2. Colocar los archivos de configuración generados y recargar (bajo deploy/generated/, excluido por .gitignore)

# 3. Obtener el certificado TLS (Let's Encrypt / certbot)
scripts/setup-tls.sh aruaru.example.com admin@example.com /var/www/aruaru.example.com

# 4. Activar la renovación automática (dos veces al día) + la monitorización automática (una vez al día, detecta caducidad próxima)
sudo deploy/systemd/install-systemd-units.sh
```

La fecha de caducidad de los certificados de todos los dominios registrados
puede comprobarse manualmente en cualquier momento con
`scripts/check-all-tls.sh`. Si se registran los mismos destinos de conexión
en la pestaña "Gestión de sitios" de la GUI de aruaru-web, se mantiene la
coherencia con el cambio de destino desde el navegador.

## Verificación de funcionamiento (realizada en este pase)

- `cargo check --target wasm32-unknown-unknown` / `cargo build --target
  wasm32-unknown-unknown` finalizan correctamente (0 advertencias) en ambos
  casos.
- Con `wasm-bindgen --target web` se generaron `pkg/aruaru_web.js` /
  `pkg/aruaru_web_bg.wasm`, se cargó `index.html` en un navegador real
  (Chromium, vía Playwright) y se verificó realizando realmente estas
  acciones: cambio de pestañas, ejecución de SQL → renderizado del
  fallback sin conexión, registro y recarga del historial de consultas y
  su tooltip al pasar el cursor, atajo Ctrl+Enter, exportación a CSV (con
  descarga real disparada), resumen del registro y obtención del listado
  de bases de datos registradas, obtención en la pestaña de gestión de
  versiones del listado de ramas, el historial de commits y el diff (todo
  ello incluyendo el fallback sin conexión), visualización de los sitios
  ya registrados en la pestaña de gestión de sitios, alta de un nuevo
  sitio, rechazo de un puerto inválido, botón de prueba de conexión,
  exportación/importación en JSON (con ida y vuelta verificada) y el
  diálogo de confirmación de borrado (tanto cancelar como confirmar). No
  hay errores de JS en la consola (solo los registros de fallo de conexión
  esperados).
- Se instalaron realmente Nginx 1.24 (versión estándar de Ubuntu 24.04),
  Apache 2.4 y certbot, y se puso en marcha lo generado por
  `scripts/gen-vhost.sh` con un certificado autofirmado, verificándolo con
  `curl` (redirección HTTP→HTTPS, ruta de desafío ACME, proxy inverso hacia
  `/graphql`); se ejecutó `scripts/check-tls.sh` contra un servidor HTTPS
  real, comprobando los tres estados WARN/healthy/ERROR; y se verificaron
  `deploy/systemd/*` con `systemd-analyze verify` (0 errores). En este
  proceso se descubrió y corrigió un error real en la plantilla de vhost
  de Nginx (`http2 on;` provoca un error de sintaxis en Nginx 1.24). La
  emisión real de un certificado de Let's Encrypt mediante certbot
  (autenticación ACME) no se ha podido verificar, por no disponer de un
  dominio público y por una incompatibilidad de ABI de Python en el
  entorno de pruebas (más detalles en CLAUDE.md).

## Estructura

```text
aruaru-web/
├── Cargo.toml            # crate-type = ["cdylib", "rlib"], dependencias wasm-bindgen/web-sys
├── src/
│   ├── lib.rs             # punto de entrada, cambio de pestañas, cableado de eventos
│   ├── dom.rs             # ayudantes comunes de manipulación del DOM (descarga de archivos, etc.)
│   ├── graphql.rs         # llamadas fetch al endpoint /graphql
│   ├── render.rs          # renderizado de resultados SQL y del resumen del registro, exportación a CSV
│   ├── profiles.rs        # gestión de sitios (perfiles de conexión, guardado en localStorage, importación/exportación JSON)
│   ├── history.rs         # historial de consultas SQL (últimas 10, guardado en localStorage)
│   └── shell.rs           # shell HTML (pestañas, formularios)
├── index.html             # cargador que carga pkg/ + CSS
├── pkg/                   # artefactos generados por wasm-bindgen (excluido por .gitignore, se regenera al compilar)
├── scripts/
│   ├── serve.sh            # arranca el servidor de desarrollo escuchando en cualquier dirección IP
│   ├── gen-vhost.sh         # genera vhosts de Nginx/Apache a partir de dominio/IP
│   ├── setup-tls.sh         # obtiene el certificado de Let's Encrypt
│   ├── check-tls.sh         # comprueba la fecha de caducidad del certificado de un dominio
│   └── check-all-tls.sh     # comprueba de una vez la caducidad de todos los dominios registrados
├── deploy/
│   ├── nginx/vhost.conf.template
│   ├── apache/vhost.conf.template
│   ├── systemd/             # conjunto de temporizadores de renovación automática y monitorización automática
│   └── generated/           # salida de gen-vhost.sh (excluido por .gitignore)
└── CLAUDE.md
```

## Proyectos relacionados

- **aruaru-db** (el destino al que se conecta esta UI): https://github.com/aon-co-jp/aruaru-db
- **open-runo**: https://github.com/aon-co-jp/open-runo
- **open-web-server**: https://github.com/aon-co-jp/open-web-server
- **poem-cosmo-tauri**: https://github.com/aon-co-jp/poem-cosmo-tauri
- **open-raid-z** (fuente autoritativa de las reglas de desarrollo): https://github.com/aon-co-jp/open-raid-z
- **rs-to-readme**: https://github.com/aon-co-jp/rs-to-readme

## License

Apache-2.0
