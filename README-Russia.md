# aruaru-web

## Минимальный веб-интерфейс для aruaru-db (Rust → WebAssembly, без фреймворков)

Это первая bootstrap-версия браузерного интерфейса, который действительно
обращается к GraphQL-эндпоинту (`/graphql`), предоставляемому `aruaru-db`
(распределённой базой данных Git-on-SQL) — запрос `sql` и запрос
`registrySummary` (агрегат реестра поддерживаемых БД) — и отображает
результаты. **Это ещё не полноценная админ-консоль** — объём намеренно
ограничен полем ввода SQL, кнопкой сводки реестра и таблицей результатов.

📖 Другие языки: [日本語](README-Japan.md) / [English](README-English.md) /
[中文](README-Chinese.md) / [한국어](README-Korea.md) / [Español](README-Spain.md) /
[Français](README-France.md) / [Deutsch](README-Germany.md) / [Italiano](README-Italy.md) /
[Русский](README-Russia.md) / [العربية](README-Arabic.md)

## Текущее состояние

- Отправляет настоящие GraphQL-запросы через `fetch()` на эндпоинт
  `/graphql` из `aruaru-graphql` (`sql(query: String!)`, `registrySummary`).
- Если `aruaru-server` не запущен или недоступен, страница отображает
  **пример данных точно в форме реальной схемы** и явно помечает его как
  офлайн-образец (проверено в реальном браузере).
- GraphQL-мутации, аутентификация и пагинация пока не реализованы.

## Быстрый старт

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.126
cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg \
  target/wasm32-unknown-unknown/debug/aruaru_web.wasm
python -m http.server 8080   # затем открыть http://localhost:8080/index.html
```

## Структура

`Cargo.toml` (`cdylib`/`rlib`, зависимости wasm-bindgen/web-sys),
`src/lib.rs` (единственный исходный файл: построение DOM, fetch, рендеринг
ответа GraphQL), `index.html` (тонкий загрузчик для `pkg/`).

## Лицензия

Apache-2.0
