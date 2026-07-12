# aruaru-web

**واجهة ويب حدّية لـ aruaru-db (Rust → WebAssembly، بدون أي إطار عمل)**

هذه لوحة تحكم بتبويبات تتيح من داخل المتصفح استدعاء استعلام `sql` واستعلام
`registrySummary` (تجميع سجلّ قواعد البيانات المتوافقة) اللذين تعرضهما
`aruaru-db` (قاعدة بيانات موزّعة من نوع Git-on-SQL) عبر GraphQL (`/graphql`)،
وعرض النتائج فعلياً. إلى جانب تنفيذ SQL وتجميع السجلّ، تضم الواجهة **تبويب
"إدارة المواقع" الذي يتيح تسجيل عدة جهات اتصال (لأجل aruaru-web ولمشاريع
أخرى) والتبديل بينها**، على غرار قائمة المواقع في KUSANAGI.

📖 لغات أخرى: [日本語](README-Japan.md) / [English](README-English.md) /
[中文](README-Chinese.md) / [한국어](README-Korea.md) / [Español](README-Spain.md) /
[Français](README-France.md) / [Deutsch](README-Germany.md) / [Italiano](README-Italy.md) /
[Русский](README-Russia.md) / [العربية](README-Arabic.md)

---

## ما هو متاح الآن

- إرسال طلبات GraphQL فعلية عبر `fetch()` إلى نقطة النهاية `/graphql` الخاصة
  بـ `aruaru-graphql` (`aruaru-db/crates/aruaru-graphql`):
  - `sql(query: String!): QueryResultGql` — تنفيذ أي استعلام SQL وعرض
    `columns`/`rows`/`commandTag` في جدول
  - `registrySummary: RegistrySummaryGql` — عرض تجميع سجلّ قواعد البيانات
    المتوافقة (أكثر من 150 قاعدة) في بطاقات
  - `registry: [DbEntryGql!]!` — عرض **قائمة** بسجلّ قواعد البيانات المتوافقة
    في جدول (الاسم/الفئة/التوافق مع البروتوكول/الحالة/الترتيب/النتيجة/تاريخ
    التحديث)
- **تبويب إدارة الإصدارات**: يمكن تنفيذ `currentBranch`/`branches` (قائمة
  الفروع والفرع الحالي)، و`log(limit)` (سجلّ الالتزامات/commits)، و
  `diff(from, to)` (عدد الإضافات/الحذوفات/التعديلات في الفرق بين الفروع).
  جميعها مبنية على المخطط الفعلي (`VcsQuery`) في
  `aruaru-db/crates/aruaru-graphql`.
- في حال عدم تشغيل `aruaru-server` أو تعذّر الاتصال به، يتم عرض **بيانات
  نموذجية بنفس شكل المخطط الفعلي** فوراً، مع توضيح صريح أنها "عيّنة دون
  اتصال" (تم التحقق من هذا السلوك فعلياً في متصفح حقيقي — انظر قسم "التحقق
  من عمل التطبيق" أدناه).
- **تبويب إدارة المواقع**: يتيح تسجيل عدة جهات اتصال (لأجل aruaru-web ولمشاريع
  أخرى) تتضمن عنوان IP/نطاق/نطاق فرعي/منفذ/مسار، وحفظها في `localStorage`
  والتبديل بينها بنقرة واحدة. يتبع حقل نقطة النهاية في تبويبَي SQL/تجميع
  السجلّ تلقائياً الموقع المُختار حالياً. تضم كل بطاقة **زر "اختبار الاتصال"**
  الذي يتحقق من الاتصال دون تغيير الموقع النشط، وتحقّقاً من صحة رقم المنفذ
  (1 إلى 65535)، و**تصدير/استيراد بصيغة JSON** لقائمة المواقع المسجَّلة
  (لأغراض النسخ الاحتياطي والنقل إلى متصفح آخر)، ومربع حوار تأكيد قبل الحذف.
- **سهولة استخدام تبويب SQL**: **سجلّ لآخر 10 استعلامات** (يُعاد تحميلها
  بالنقر، ويظهر النص الكامل عند التحويم بالماوس)، **اختصار تنفيذ باستخدام
  Ctrl+Enter أو Cmd+Enter**، **تصدير نتائج التنفيذ بصيغة CSV**، تعطيل الزر
  أثناء التنفيذ، وجدول نتائج يعرض عدد الصفوف وقابل للتمرير مع رأس ثابت
  (sticky).
- **الإعداد التلقائي لبروتوكول HTTPS (TLS) ومراقبته وتجديده تلقائياً**:
  يقوم `scripts/gen-vhost.sh` بتوليد إعدادات vhost لـ Nginx/Apache (تتضمن
  إعادة التوجيه من HTTP إلى HTTPS)، ويقوم `scripts/setup-tls.sh` بالحصول على
  شهادة Let's Encrypt (عبر certbot)، ويمكن لـ
  `deploy/systemd/install-systemd-units.sh` تفعيل "تجديد تلقائي مرتين
  يومياً" (`aruaru-tls-renew.timer`) و"مراقبة انتهاء الصلاحية مرة واحدة
  يومياً" (`aruaru-tls-monitor.timer` ← `scripts/check-all-tls.sh`).
  للتفاصيل انظر قسم "HTTPS وتسجيل النطاقات/النطاقات الفرعية".

## ما لا يمكن فعله حالياً (بصراحة تامة)

- عمليات GraphQL Mutation (إنشاء فروع، الدمج، زحف السجلّ، إلخ) غير منفَّذة.
- المصادقة، وترقيم الصفحات، وإعادة المحاولة التلقائية عند حدوث خطأ غير
  منفَّذة.
- لا تقدَّم تجربة تطبيق أصلي مثل Tauri (فقط WASM يعمل داخل المتصفح).
- **لا يقوم هذا المستودع فعلياً بشراء النطاقات أو تسجيل سجلات DNS (عبر
  المسجِّل/Registrar)** لأن ذلك يترتب عليه تكلفة ويؤثر على خدمات خارجية.
  ما يتم أتمتته هنا يقتصر على "توليد إعدادات vhost" و"الحصول على شهادة TLS
  ومراقبتها وتجديدها تلقائياً" لنطاق تم الحصول عليه مسبقاً، أما تسجيل DNS
  نفسه فيقوم به المستخدم لدى المسجِّل.

## طريقة البناء

لا يُستخدم Node.js ولا npm ولا TypeScript. يكتمل كل شيء بواسطة سلسلة أدوات
Rust فقط.

```bash
rustup target add wasm32-unknown-unknown        # لأول مرة فقط
cargo install wasm-bindgen-cli --version 0.2.126 # لأول مرة فقط(يجب أن يطابق إصدار Cargo.lock)

cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg \
  target/wasm32-unknown-unknown/debug/aruaru_web.wasm

# قدّم الملفات عبر أي خادم ثابت وافتحها(أي خادم يفي بالغرض، مثال:)
python -m http.server 8080
# افتح http://localhost:8080/index.html في المتصفح
```

لتجربة تشغيل `aruaru-db` فعلياً:

```bash
cd ../aruaru-db
cargo run -p aruaru-server -- --data ./data --raft-id 1   # يعمل GraphQL على المنفذ :4000
```

## التشغيل من عنوان IP

```bash
scripts/serve.sh 0.0.0.0 8080        # الاستماع على جميع الواجهات
scripts/serve.sh 192.168.1.50 8080   # الاستماع على عنوان IP محدد فقط
```

## HTTPS وتسجيل النطاقات/النطاقات الفرعية

لا يقوم هذا المستودع نفسه بشراء النطاقات أو تسجيل سجلات DNS (لأن ذلك يتطلب
عمليات لدى المسجِّل وتكاليف يتحمّلها المستخدم بشكل منفصل). ما يلي هو أتمتة
محلية (تعادل "إضافة موقع" في KUSANAGI) لتسهيل تخصيص نطاق/نطاق فرعي تم الحصول
عليه مسبقاً لاستخدام aruaru-web أو لمشاريع أخرى.

```bash
# 1. توليد vhost(لـ Nginx/Apache، مع تضمين إعادة التوجيه من HTTP إلى HTTPS) من النطاق + IP + الخلفية(backend)
scripts/gen-vhost.sh aruaru.example.com 203.0.113.10 127.0.0.1:4000
# وبالمثل بالنسبة للنطاقات الفرعية المخصّصة لأغراض أخرى(فقط غيّر UPSTREAM/WEBROOT)
scripts/gen-vhost.sh tool.example.com 203.0.113.10 127.0.0.1:9000 /var/www/tool

# 2. ضع ملفات الإعداد المولَّدة في مكانها وأعد التحميل(ضمن deploy/generated/، وهي مدرجة في .gitignore)

# 3. الحصول على شهادة TLS(Let's Encrypt / certbot)
scripts/setup-tls.sh aruaru.example.com admin@example.com /var/www/aruaru.example.com

# 4. تفعيل التجديد التلقائي(مرتين يومياً) + المراقبة التلقائية(مرة يومياً، لرصد اقتراب انتهاء الصلاحية)
sudo deploy/systemd/install-systemd-units.sh
```

يمكن التحقق يدوياً في أي وقت من تواريخ انتهاء صلاحية شهادات جميع النطاقات
المسجَّلة عبر `scripts/check-all-tls.sh`. كما يُنصح بتسجيل جهات الاتصال
نفسها في تبويب "إدارة المواقع" بواجهة aruaru-web الرسومية، لكي تتطابق مع
جهات الاتصال التي يتم التبديل بينها من المتصفح.

## التحقق من عمل التطبيق (تم تنفيذه في هذا المسار)

- نجاح كل من `cargo check --target wasm32-unknown-unknown` و
  `cargo build --target wasm32-unknown-unknown` (بدون أي تحذيرات).
- تم توليد `pkg/aruaru_web.js` / `pkg/aruaru_web_bg.wasm` عبر
  `wasm-bindgen --target web`، وتحميل `index.html` في متصفح حقيقي (Chromium
  عبر Playwright)، والتحقق فعلياً من خلال التفاعل المباشر مع ما يلي: التبديل
  بين التبويبات، تنفيذ SQL ← عرض البيانات الاحتياطية دون اتصال، تسجيل
  الاستعلامات في السجلّ وإعادة تحميلها وظهور تلميح عند التحويم، اختصار
  Ctrl+Enter، تصدير CSV (مع إطلاق تنزيل فعلي)، الحصول على تجميع السجلّ وعلى
  قائمة قواعد البيانات المسجَّلة، والحصول في تبويب إدارة الإصدارات على قائمة
  الفروع وسجلّ الالتزامات ونتيجة Diff (بما في ذلك مسار البيانات الاحتياطية
  دون اتصال في كل هذه الحالات)، وفي تبويب إدارة المواقع: عرض المواقع
  المسجَّلة، إضافة موقع جديد، رفض إدخال منفذ غير صالح، زر اختبار الاتصال،
  تصدير/استيراد JSON (تم التحقق من نجاح العملية ذهاباً وإياباً)، ومربع حوار
  تأكيد الحذف (كلا الخيارين: الإلغاء والتنفيذ). لا توجد أي أخطاء JavaScript
  في وحدة التحكم (console) سوى سجلات فشل الاتصال المقصودة.
- تم تثبيت Nginx 1.24 (الإصدار المعتمد في Ubuntu 24.04) وApache 2.4 وcertbot
  فعلياً، وتشغيل مخرجات `scripts/gen-vhost.sh` باستخدام شهادة موقّعة ذاتياً
  والتحقق عبر `curl` (إعادة التوجيه من HTTP إلى HTTPS، مسار تحدي ACME،
  والوكيل العكسي (reverse proxy) نحو `/graphql`)، وتشغيل `scripts/check-tls.sh`
  على خادم HTTPS فعلي والتأكد من الحالات الثلاث WARN/healthy/ERROR، والتحقق
  من `deploy/systemd/*` عبر `systemd-analyze verify` (بدون أي أخطاء). خلال
  هذه العملية تم اكتشاف وإصلاح خطأ فعلي في قالب vhost الخاص بـ Nginx (كانت
  `http2 on;` تسبب خطأ نحوياً في Nginx 1.24). أما الحصول الفعلي على شهادة
  Let's Encrypt عبر certbot (مصادقة ACME) فلم يُتحقق منه بسبب عدم توفر نطاق
  عام (public domain) وعدم توافق إصدارات Python ABI في بيئة الاختبار
  (للتفاصيل انظر CLAUDE.md).

## البنية

```text
aruaru-web/
├── Cargo.toml            # crate-type = ["cdylib", "rlib"]、wasm-bindgen/web-sys依存
├── src/
│   ├── lib.rs             # نقطة الدخول، التبديل بين التبويبات، ربط الأحداث
│   ├── dom.rs             # دوال مساعدة مشتركة للتعامل مع DOM(مثل تنزيل الملفات)
│   ├── graphql.rs         # استدعاءات fetch نحو /graphql
│   ├── render.rs          # عرض نتائج SQL وتجميع السجلّ، وإخراج CSV
│   ├── profiles.rs        # إدارة المواقع(ملفات جهات الاتصال، الحفظ في localStorage، تصدير/استيراد JSON)
│   ├── history.rs         # سجلّ استعلامات SQL(آخر 10 استعلامات، الحفظ في localStorage)
│   └── shell.rs           # هيكل HTML(التبويبات والنماذج)
├── index.html             # محمِّل يقرأ pkg/ + ملف CSS
├── pkg/                   # نواتج wasm-bindgen(ضمن .gitignore، تُعاد توليدها عند البناء)
├── scripts/
│   ├── serve.sh            # تشغيل خادم تطوير يستمع من أي عنوان IP
│   ├── gen-vhost.sh         # توليد إعدادات vhost لـ Nginx/Apache من النطاق/IP
│   ├── setup-tls.sh         # الحصول على شهادة Let's Encrypt
│   ├── check-tls.sh         # فحص تاريخ انتهاء صلاحية شهادة نطاق واحد
│   └── check-all-tls.sh     # فحص تواريخ انتهاء الصلاحية لجميع النطاقات المسجَّلة دفعة واحدة
├── deploy/
│   ├── nginx/vhost.conf.template
│   ├── apache/vhost.conf.template
│   ├── systemd/             # مجموعة مؤقّتات التجديد التلقائي(renew) والمراقبة التلقائية(monitor)
│   └── generated/           # مخرجات gen-vhost.sh(ضمن .gitignore)
└── CLAUDE.md
```

## المشاريع ذات الصلة

- **aruaru-db** (الجهة التي تتصل بها هذه الواجهة): https://github.com/aon-co-jp/aruaru-db
- **open-runo**: https://github.com/aon-co-jp/open-runo
- **open-web-server**: https://github.com/aon-co-jp/open-web-server
- **poem-cosmo-tauri**: https://github.com/aon-co-jp/poem-cosmo-tauri
- **open-raid-z** (المرجع الرسمي لقواعد التطوير): https://github.com/aon-co-jp/open-raid-z
- **rs-to-readme**: https://github.com/aon-co-jp/rs-to-readme

## License

Apache-2.0
