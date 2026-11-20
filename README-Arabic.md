# aruaru-web

## واجهة ويب بسيطة لـ aruaru-db (Rust ← WebAssembly، بدون أي إطار عمل)

هذه أول نسخة تأسيسية من واجهة متصفح تستدعي فعليًا نقطة نهاية GraphQL
(`/graphql`) التي توفرها `aruaru-db` (قاعدة بيانات Git-on-SQL الموزعة) —
استعلام `sql` واستعلام `registrySummary` (ملخّص سجل قواعد البيانات
المدعومة) — وتعرض النتائج. **هذه ليست بعد لوحة إدارة كاملة** — تم تعمّد
حصر النطاق في حقل إدخال SQL، وزر لملخص السجل، وجدول للنتائج.

📖 لغات أخرى: [日本語](README-Japan.md) / [English](README-English.md) /
[中文](README-Chinese.md) / [한국어](README-Korea.md) / [Español](README-Spain.md) /
[Français](README-France.md) / [Deutsch](README-Germany.md) / [Italiano](README-Italy.md) /
[Русский](README-Russia.md) / [العربية](README-Arabic.md)

## الوضع الحالي

- ترسل طلبات GraphQL حقيقية عبر `fetch()` إلى نقطة نهاية `/graphql` الخاصة
  بـ `aruaru-graphql` (`sql(query: String!)`، `registrySummary`).
- إذا لم يكن `aruaru-server` قيد التشغيل أو تعذّر الوصول إليه، تعرض الصفحة
  **بيانات نموذجية بنفس شكل المخطط الحقيقي تمامًا** وتوضّح بشكل صريح أنها
  عيّنة غير متصلة (تم التحقق منها في متصفح حقيقي).
- لم يتم بعد تنفيذ طفرات GraphQL (mutations)، أو المصادقة، أو ترقيم الصفحات.

## البدء السريع

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.126
cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg \
  target/wasm32-unknown-unknown/debug/aruaru_web.wasm
python -m http.server 8080   # ثم افتح http://localhost:8080/index.html
```

## البنية

`Cargo.toml` (`cdylib`/`rlib`، اعتماديات wasm-bindgen/web-sys)،
`src/lib.rs` (ملف المصدر الوحيد: بناء DOM، fetch، عرض استجابة GraphQL)،
`index.html` (محمّل بسيط لـ `pkg/`).

## الترخيص

Apache-2.0
