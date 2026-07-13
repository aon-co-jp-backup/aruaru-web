# aruaru-web

**"El segundo KUSANAGI" — una herramienta de operación que, tras subir la
aplicación, se pone en marcha desde una dirección IP y permite aplicar
fácilmente y de forma automática el registro de dominio y la puesta en
marcha de HTTPS (Rust → WebAssembly, sin framework)**

Al estilo del kit de construcción de servidores "KUSANAGI" para acelerar
WordPress, esta es una herramienta de operación que aspira a cubrir de
principio a fin, una vez subida la aplicación, desde el **arranque desde
una dirección IP** hasta la **simplificación del registro de dominio** y la
**automatización de HTTPS**. Puede generar automáticamente la
configuración de proxy inverso (Nginx/Apache) que acelera sitios web con
cualquier stack de backend, como WordPress, PHP + Laravel o Python +
FastAPI, y cuenta con una pantalla de "Gestión de sitios" para registrar,
alternar y comprobar la conectividad de los destinos de conexión de varios
sitios. **No tiene funciones de conexión a bases de datos (BD)** (queda
fuera de alcance de forma intencionada).

📖 Otros idiomas: [日本語](README-Japan.md) / [English](README-English.md) /
[中文](README-Chinese.md) / [한국어](README-Korea.md) / [Español](README-Spain.md) /
[Français](README-France.md) / [Deutsch](README-Germany.md) / [Italiano](README-Italy.md) /
[Русский](README-Russia.md) / [العربية](README-Arabic.md)

---

## Qué se puede hacer ahora

- **Pantalla de gestión de sitios**: registra varios destinos de despliegue
  (dirección IP/dominio/subdominio/puerto/ruta) de cualquier stack de
  backend —aruaru-web mismo, WordPress, Laravel, FastAPI, etc.—, los
  guarda en `localStorage` y permite seleccionarlos y comprobar la
  conectividad con un solo clic (equivalente al listado de sitios de
  KUSANAGI). Cada tarjeta incluye un **botón de "Prueba de conexión"**
  (realiza solo una comprobación simple de alcance HTTP sin cambiar el
  sitio seleccionado), validación del número de puerto (1-65535),
  **exportación/importación en JSON** de la lista de sitios registrados
  (para copias de seguridad o para llevarlos a otro navegador), y un
  diálogo de confirmación antes de eliminar.
- **Arranque desde una dirección IP**: `scripts/serve.sh` permite servir
  la aplicación haciendo bind a cualquier IP/puerto, tanto en local como
  en un VPS.
- **Generación de vhost, aceleración y configuración automática de
  HTTPS**: `scripts/gen-vhost.sh` genera el vhost de Nginx/Apache (con
  redirección HTTP→HTTPS incluida) a partir de la combinación de dominio,
  IP y stack de backend. Admite los 5 stacks `static` (sitio estático),
  `proxy` (proxy inverso genérico para aruaru-db, open-web-server,
  open-raid-z y similares, o cualquier backend HTTP), `wordpress`,
  `laravel` y `fastapi`, e incluye configuraciones de aceleración según el
  stack, como compresión gzip, caché de larga duración para activos
  estáticos, keepalive del upstream y ajuste del búfer de FastCGI.
- **Monitorización y renovación automática de HTTPS (TLS)**:
  `scripts/setup-tls.sh` obtiene el certificado de Let's Encrypt
  (certbot), y `deploy/systemd/install-systemd-units.sh` permite activar
  la "renovación automática dos veces al día"
  (`aruaru-tls-renew.timer`) y la "monitorización de caducidad una vez al
  día" (`aruaru-tls-monitor.timer` → `scripts/check-all-tls.sh`).
- **Despliegue en un VPS**: basta con ejecutar `scripts/deploy-vps.ps1`
  desde Windows PowerShell para automatizar la compilación, la subida al
  VPS y la puesta en marcha (para más detalles, ver "Despliegue en un
  VPS" más abajo).

## Qué no se puede hacer todavía (alcance honesto)

- **No tiene funciones de conexión a bases de datos (BD)**. Las funciones
  que dependen de un producto de base de datos específico, como la
  ejecución de SQL o las consultas GraphQL, quedan fuera de alcance de
  forma intencionada y no se implementarán en el futuro. Incluso al usar
  un backend específico como aruaru-db, la pantalla de "Gestión de
  sitios" y la generación de vhost (`--stack=proxy`) pueden emplearse
  como **proxy inverso y gestión de despliegue de propósito general**,
  pero no se ofrece ninguna función de consulta propia de esa base de
  datos.
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
- La contratación real de un VPS (el contrato con el proveedor de
  servidor de alquiler) tampoco se realiza desde este repositorio.

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

## Iniciar desde una dirección IP

```bash
scripts/serve.sh 0.0.0.0 8080        # escuchar en todas las interfaces
scripts/serve.sh 192.168.1.50 8080   # escuchar solo en una dirección IP concreta
```

Tras iniciar, puede comprobarse **introduciendo directamente la dirección
IP en la barra de direcciones del navegador** (por ejemplo:
`http://192.168.1.50:8080/index.html`). Lo importante es que se puede
verificar el funcionamiento solo con la dirección IP, incluso antes de
registrar el dominio.

## Despliegue en un VPS (desde Windows PowerShell)

Tras alquilar un servidor VPS, basta con ejecutar
`scripts/deploy-vps.ps1` desde Windows PowerShell para automatizar la
compilación → subida → puesta en marcha. Si se usa `open-web-server` en
paralelo, también puede subirse al mismo tiempo (este repositorio no
entra en el contenido de `open-web-server`; solo se especifica la ruta de
destino de la subida).

```powershell
# Para subir y también poner en marcha el servidor (solo aruaru-web)
.\scripts\deploy-vps.ps1 -VpsHost 203.0.113.10 -VpsUser root -StartServer

# Para subir también open-web-server (F:\open-runo\open-web-server) al mismo tiempo
.\scripts\deploy-vps.ps1 -VpsHost 203.0.113.10 -VpsUser root -StartServer `
    -OpenWebServerPath "F:\open-runo\open-web-server"
```

Internamente esto realiza un procesamiento equivalente a lo siguiente
(también puede ejecutarse manualmente):

```powershell
# 1. Compilar en local
cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg `
    target/wasm32-unknown-unknown/debug/aruaru_web.wasm

# 2. Subir al VPS (cliente OpenSSH, incluido de serie desde Windows 10 1809 en adelante / Windows 11)
ssh root@203.0.113.10 "mkdir -p /root/aruaru-web"
scp -r .\index.html .\pkg .\scripts .\deploy .\Cargo.toml .\src `
    root@203.0.113.10:/root/aruaru-web/

# 3. Iniciar en el VPS desde una dirección IP
ssh root@203.0.113.10 "cd /root/aruaru-web && bash scripts/serve.sh 0.0.0.0 8080"
```

Tras iniciar, **introduzca la dirección IP del VPS en la barra de
direcciones del navegador** (por ejemplo:
`http://203.0.113.10:8080/index.html`). Si desea mantener siempre
actualizado su directorio local, por ejemplo `F:\open-runo\aruaru-web`,
basta con hacer `git pull` de este repositorio:

```powershell
cd F:\open-runo\aruaru-web
git fetch origin
git pull origin <nombre-de-rama>
```

**Se puede usar sin necesidad de subir nada**: si quiere probarlo solo en
local sin usar un VPS, basta con ejecutar `scripts/serve.sh` directamente
en local tal como se explica en "Iniciar desde una dirección IP" más
arriba. Desde otros equipos de la misma LAN se puede acceder mediante
`http://<dirección-IP-del-PC-local>:8080/`.

## Generación de vhost, aceleración y registro de dominio/subdominio

Este propio repositorio no realiza la adquisición de dominios ni el
registro de registros DNS (esas operaciones, y su coste, las realiza el
usuario por separado en su registrador). Lo siguiente es la
automatización local (equivalente a "añadir sitio" en KUSANAGI) para
expedir fácilmente, por stack, un proxy inverso con configuración de
aceleración incluida, para un dominio/subdominio ya adquirido.

```bash
# aruaru-web mismo (sitio estático)
scripts/gen-vhost.sh --stack=static aruaru.example.com 203.0.113.10

# Proxy inverso genérico hacia aruaru-db, open-web-server, open-raid-z o similares, o cualquier backend
scripts/gen-vhost.sh --stack=proxy tool.example.com 203.0.113.10 127.0.0.1:9000

# WordPress (especificando el socket/dirección de PHP-FPM)
scripts/gen-vhost.sh --stack=wordpress blog.example.com 203.0.113.10 \
  unix:/run/php/php8.3-fpm.sock /var/www/blog

# Laravel (indicando explícitamente el directorio public)
scripts/gen-vhost.sh --stack=laravel app.example.com 203.0.113.10 \
  unix:/run/php/php8.3-fpm.sock /var/www/app/public

# FastAPI (proxy inverso hacia el servidor ASGI, con soporte de WebSocket/streaming)
scripts/gen-vhost.sh --stack=fastapi api.example.com 203.0.113.10 127.0.0.1:8000
```

Tras colocar los archivos de configuración generados (bajo
`deploy/generated/`, excluido por `.gitignore`) en el directorio de
configuración de Nginx/Apache y recargar, se obtiene el certificado:

```bash
scripts/setup-tls.sh aruaru.example.com admin@example.com /var/www/aruaru.example.com

# Activar la renovación automática (dos veces al día) + la monitorización automática (una vez al día, detecta caducidad próxima)
sudo deploy/systemd/install-systemd-units.sh
```

La fecha de caducidad de los certificados de todos los dominios
registrados puede comprobarse manualmente en cualquier momento con
`scripts/check-all-tls.sh`. Si se registran los mismos destinos de
conexión en la pantalla "Gestión de sitios" de la GUI de aruaru-web,
también se podrá consultar la lista y realizar pruebas de conexión desde
el navegador.

## Verificación de funcionamiento (realizada en este pase)

- `cargo check --target wasm32-unknown-unknown` / `cargo build --target
  wasm32-unknown-unknown` finalizan correctamente (0 advertencias) en
  ambos casos.
- Con `wasm-bindgen --target web` se generaron `pkg/aruaru_web.js` /
  `pkg/aruaru_web_bg.wasm`, se cargó `index.html` en un navegador real
  (Chromium, vía Playwright) y se verificó realizando realmente estas
  acciones: en la pantalla de gestión de sitios, visualización de los
  sitios ya registrados, alta de uno nuevo, rechazo de un puerto
  inválido, botón de prueba de conexión (comprobación de alcance contra
  un servidor HTTP real en ejecución, con éxito), diálogo de
  confirmación de borrado (tanto cancelar como confirmar), y
  exportación/importación en JSON. No hay errores de JS en la consola.
- Se instalaron realmente Nginx 1.24 (versión estándar de Ubuntu 24.04) y
  Apache 2.4, y se verificó con `nginx -t` / `apache2ctl configtest` la
  sintaxis de lo generado por `gen-vhost.sh` para los 5 stacks (static,
  proxy, wordpress, laravel, fastapi) con un certificado autofirmado; los
  stacks static/proxy se pusieron en marcha realmente y se verificó su
  funcionamiento con `curl` (redirección HTTP→HTTPS, servicio estático,
  respuesta 502 a través del proxy inverso).
- La emisión real de un certificado de Let's Encrypt mediante certbot
  (autenticación ACME), así como el funcionamiento de
  `scripts/deploy-vps.ps1` en un entorno VPS real, no se han podido
  verificar en esta sesión por no disponer de un dominio público, un VPS
  real ni un entorno Windows (más detalles en CLAUDE.md).

## Estructura

```text
aruaru-web/
├── Cargo.toml            # crate-type = ["cdylib", "rlib"], dependencias wasm-bindgen/web-sys
├── src/
│   ├── lib.rs             # punto de entrada, cableado de eventos
│   ├── dom.rs             # ayudantes comunes de manipulación del DOM (descarga de archivos, etc.)
│   ├── profiles.rs        # gestión de sitios (perfiles de conexión, guardado en localStorage, prueba de conexión, importación/exportación JSON)
│   └── shell.rs           # shell HTML
├── index.html             # cargador que carga pkg/ + CSS
├── pkg/                   # artefactos generados por wasm-bindgen (excluido por .gitignore, se regenera al compilar)
├── scripts/
│   ├── serve.sh            # arranca el servidor de desarrollo escuchando en cualquier dirección IP
│   ├── deploy-vps.ps1       # compila, sube y pone en marcha en un VPS desde Windows PowerShell
│   ├── gen-vhost.sh         # genera vhosts de Nginx/Apache a partir de dominio/IP/stack
│   ├── setup-tls.sh         # obtiene el certificado de Let's Encrypt
│   ├── check-tls.sh         # comprueba la fecha de caducidad del certificado de un dominio
│   └── check-all-tls.sh     # comprueba de una vez la caducidad de todos los dominios registrados
├── deploy/
│   ├── nginx/vhost-{static,proxy,wordpress,laravel,fastapi}.conf.template
│   ├── apache/vhost-{static,proxy,wordpress,laravel,fastapi}.conf.template
│   ├── systemd/             # conjunto de temporizadores de renovación automática y monitorización automática
│   └── generated/           # salida de gen-vhost.sh (excluido por .gitignore)
└── CLAUDE.md
```

## Proyectos relacionados

Dado que este repositorio es una herramienta genérica de despliegue y
operación independiente de la base de datos, puede **usarse junto con**
otros proyectos como los siguientes (pueden registrarse en la pantalla de
"Gestión de sitios" o emplearse como destino de proxy inverso con
`--stack=proxy`; no se entra en el contenido de cada repositorio):

- **aruaru-db**: https://github.com/aon-co-jp/aruaru-db
- **open-runo**: https://github.com/aon-co-jp/open-runo
- **open-web-server**: https://github.com/aon-co-jp/open-web-server
- **poem-cosmo-tauri**: https://github.com/aon-co-jp/poem-cosmo-tauri
- **open-raid-z** (fuente autoritativa de las reglas de desarrollo): https://github.com/aon-co-jp/open-raid-z
- **rs-to-readme**: https://github.com/aon-co-jp/rs-to-readme

## License

Apache-2.0
