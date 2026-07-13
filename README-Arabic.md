# aruaru-web

**"كوساناغي الثانية" — أداة تشغيل تتيح تشغيل التطبيق من عنوان IP بعد رفعه،
مع تطبيق تلقائي وسهل لتسجيل النطاق (domain) وتفعيل HTTPS (مبنية بلغة
Rust → WebAssembly، بدون أي إطار عمل)**

على غرار "كوساناغي" (KUSANAGI)، حزمة بناء الخوادم المتخصصة في تسريع
ووردبريس، تهدف هذه الأداة إلى تغطية المسار الكامل: بعد رفع التطبيق،
**التشغيل من عنوان IP ← تبسيط تسجيل النطاق ← أتمتة HTTPS**. يمكنها توليد
إعدادات بروكسي عكسي (reverse proxy) لـ Nginx/Apache لتسريع مواقع ويب مبنية
على أي حزمة تقنية للخادم الخلفي مثل ووردبريس، أو PHP + Laravel، أو
Python + FastAPI، كما تحتوي على شاشة "إدارة المواقع" التي تتيح تسجيل
وتبديل واختبار اتصال عدة مواقع مستهدفة. **لا تملك أي وظيفة اتصال بقاعدة
بيانات** (وهذا خارج النطاق عمداً).

📖 لغات أخرى: [日本語](README-Japan.md) / [English](README-English.md) /
[中文](README-Chinese.md) / [한국어](README-Korea.md) / [Español](README-Spain.md) /
[Français](README-France.md) / [Deutsch](README-Germany.md) / [Italiano](README-Italy.md) /
[Русский](README-Russia.md) / [العربية](README-Arabic.md)

---

## ما هو متاح الآن

- **شاشة إدارة المواقع**: يمكن تسجيل عدة وجهات نشر (عنوان IP/نطاق/نطاق
  فرعي/منفذ/مسار) لأي حزمة تقنية للخادم الخلفي، سواء لـ aruaru-web نفسها أو
  ووردبريس أو Laravel أو FastAPI، مع الحفظ في `localStorage` واختيار
  الموقع واختبار الاتصال بنقرة واحدة (يعادل قائمة المواقع في KUSANAGI).
  تحتوي كل بطاقة على **زر "اختبار الاتصال"** (ينفذ فحص وصول HTTP بسيطًا فقط
  دون تغيير الموقع المُحدد حالياً)، وتحقق من صحة إدخال رقم المنفذ
  (1 إلى 65535)، و**تصدير/استيراد JSON** لقائمة المواقع المسجلة (لأغراض
  النسخ الاحتياطي والنقل إلى متصفح آخر)، وحوار تأكيد قبل الحذف.
- **التشغيل من عنوان IP**: يتيح `scripts/serve.sh` البث بربط أي عنوان
  IP/منفذ محليًا أو على VPS.
- **توليد vhost والتسريع وأتمتة HTTPS**: يقوم `scripts/gen-vhost.sh` بتوليد
  إعدادات vhost لـ Nginx/Apache (تتضمن إعادة توجيه من HTTP إلى HTTPS) بناءً
  على تركيبة النطاق/IP/الحزمة التقنية للخادم الخلفي. يدعم خمس حزم تقنية:
  `static` (موقع ثابت)، و`proxy` (بروكسي عكسي عام لأنظمة مثل aruaru-db
  وopen-web-server وopen-raid-z أو أي خادم خلفي HTTP آخر)، و`wordpress`،
  و`laravel`، و`fastapi`، وتتضمن إعدادات تسريع خاصة بكل حزمة مثل ضغط gzip،
  والتخزين المؤقت طويل المدى للأصول الثابتة (static assets)، وkeepalive
  للـ upstream، وضبط مخزن FastCGI المؤقت (buffer).
- **المراقبة والتحديث التلقائي لشهادات HTTPS (TLS)**: يتيح
  `scripts/setup-tls.sh` الحصول على شهادة Let's Encrypt (عبر certbot)، بينما
  يتيح `deploy/systemd/install-systemd-units.sh` تفعيل "التحديث التلقائي
  مرتين يوميًا" (`aruaru-tls-renew.timer`) و"مراقبة انتهاء الصلاحية مرة
  واحدة يوميًا" (`aruaru-tls-monitor.timer` ← `scripts/check-all-tls.sh`).
- **النشر على VPS**: بمجرد تشغيل `scripts/deploy-vps.ps1` من Windows
  PowerShell، يمكن أتمتة عملية البناء ← الرفع إلى VPS ← التشغيل بالكامل
  (راجع قسم "النشر على VPS" أدناه للتفاصيل).

## ما لا يمكن فعله حالياً (نطاق صادق)

- **لا تملك أي وظيفة اتصال بقاعدة بيانات (DB)**. الوظائف المرتبطة بمنتج
  قاعدة بيانات معين، مثل تنفيذ SQL أو استعلامات GraphQL، خارج النطاق عمدًا
  ولن تُنفَّذ مستقبلاً. حتى عند استخدام خادم خلفي معين مثل aruaru-db، يمكن
  استخدام شاشة "إدارة المواقع" وتوليد vhost (`--stack=proxy`) **كأداة عامة
  للبروكسي العكسي وإدارة النشر**، لكن لن تُقدَّم أي وظائف استعلام خاصة
  بتلك القاعدة.
- المصادقة (authentication)، والترقيم (pagination)، وإعادة المحاولة
  التلقائية عند حدوث خطأ غير منفَّذة بعد.
- لا تقدم تجربة تطبيق أصلي (native) على غرار Tauri (تعمل فقط كـ WASM داخل
  المتصفح).
- **لا يتم من هذا المستودع تسجيل نطاق فعلي أو سجلات DNS (عمليات لدى مسجّل
  النطاقات)** لأن ذلك يتطلب تكاليف وتأثيرات على خدمات خارجية. ما تتم أتمتته
  هنا يقتصر على "توليد إعدادات vhost" و"الحصول على شهادة TLS ومراقبتها
  وتحديثها تلقائيًا" لنطاق تم الحصول عليه مسبقًا، أما تسجيل DNS نفسه فيقوم
  به المستخدم لدى مسجّل النطاقات.
- كما لا يتم من هذا المستودع التعاقد الفعلي على VPS (التعاقد مع مزود خادم
  مستأجر).

## طريقة البناء

لا تُستخدم Node.js أو npm أو TypeScript. يكتمل كل شيء باستخدام سلسلة أدوات
Rust فقط.

```bash
rustup target add wasm32-unknown-unknown        # مرة واحدة فقط عند البدء
cargo install wasm-bindgen-cli --version 0.2.126 # مرة واحدة فقط (يجب أن يطابق الإصدار في Cargo.lock)

cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg \
  target/wasm32-unknown-unknown/debug/aruaru_web.wasm

# قم بالبث عبر أي خادم ثابت وافتحه (أي خادم يناسب. مثال:)
python -m http.server 8080
# افتح http://localhost:8080/index.html في المتصفح
```

## التشغيل من عنوان IP

```bash
scripts/serve.sh 0.0.0.0 8080        # الاستماع على جميع الواجهات
scripts/serve.sh 192.168.1.50 8080   # الاستماع على عنوان IP محدد فقط
```

بعد التشغيل، يمكنك التحقق **بإدخال عنوان IP مباشرة في شريط عنوان المتصفح**
(مثال: `http://192.168.1.50:8080/index.html`). النقطة المهمة هي إمكانية
التحقق من العمل بعنوان IP فقط حتى قبل تسجيل أي نطاق.

## النشر على VPS (من Windows PowerShell)

بعد استئجار خادم VPS، يمكن أتمتة عملية البناء ← الرفع ← التشغيل بالكامل
بمجرد تشغيل `scripts/deploy-vps.ps1` من Windows PowerShell. عند استخدام
`open-web-server` جنبًا إلى جنب، يمكن رفعه في الوقت نفسه (لا يتدخل هذا
المستودع في محتوى `open-web-server` نفسه، بل يكتفي بتحديد مسار الرفع).

```powershell
# للرفع والتشغيل معًا (aruaru-web فقط)
.\scripts\deploy-vps.ps1 -VpsHost 203.0.113.10 -VpsUser root -StartServer

# لرفع open-web-server (F:\open-runo\open-web-server) في الوقت نفسه أيضًا
.\scripts\deploy-vps.ps1 -VpsHost 203.0.113.10 -VpsUser root -StartServer `
    -OpenWebServerPath "F:\open-runo\open-web-server"
```

يقوم هذا داخليًا بتنفيذ ما يعادل الخطوات التالية (يمكن تنفيذها يدويًا أيضًا):

```powershell
# 1. البناء محليًا
cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg `
    target/wasm32-unknown-unknown/debug/aruaru_web.wasm

# 2. الرفع إلى VPS (عميل OpenSSH، مضمّن افتراضيًا في Windows 10 1809 وما بعده/11)
ssh root@203.0.113.10 "mkdir -p /root/aruaru-web"
scp -r .\index.html .\pkg .\scripts .\deploy .\Cargo.toml .\src `
    root@203.0.113.10:/root/aruaru-web/

# 3. التشغيل على VPS من عنوان IP
ssh root@203.0.113.10 "cd /root/aruaru-web && bash scripts/serve.sh 0.0.0.0 8080"
```

بعد التشغيل، أدخل **عنوان IP الخاص بـ VPS في شريط عنوان المتصفح**
(مثال: `http://203.0.113.10:8080/index.html`). إذا رغبت في إبقاء نسخة محلية
مثل `F:\open-runo\aruaru-web` محدَّثة دائمًا، يكفي تنفيذ `git pull` على هذا
المستودع:

```powershell
cd F:\open-runo\aruaru-web
git fetch origin
git pull origin <اسم الفرع>
```

**يمكن استخدامها دون رفع**: إذا أردت التجربة محليًا فقط دون استخدام VPS،
يكفي تشغيل `scripts/serve.sh` محليًا كما هو موضّح في "التشغيل من عنوان IP"
أعلاه. يمكن الوصول إليها من أجهزة أخرى على نفس الشبكة المحلية (LAN) عبر
`http://<عنوان IP الخاص بالجهاز المحلي>:8080/`.

## توليد vhost والتسريع وتسجيل النطاق/النطاق الفرعي

لا يقوم هذا المستودع نفسه بالحصول على نطاق أو تسجيل سجلات DNS (لأن ذلك
يتطلب عمليات لدى مسجّل النطاقات وتكاليف، فيقوم بها المستخدم بشكل منفصل).
ما يلي هو أتمتة محلية (تعادل "إضافة موقع" في KUSANAGI) لتوفير إعدادات
بروكسي عكسي مع تسريع، بسهولة وحسب الحزمة التقنية، لنطاق/نطاق فرعي تم
الحصول عليه مسبقًا.

```bash
# aruaru-web نفسها (موقع ثابت)
scripts/gen-vhost.sh --stack=static aruaru.example.com 203.0.113.10

# بروكسي عكسي عام لـ aruaru-db وopen-web-server وopen-raid-z أو أي خادم خلفي آخر
scripts/gen-vhost.sh --stack=proxy tool.example.com 203.0.113.10 127.0.0.1:9000

# ووردبريس (حدد مقبس/عنوان PHP-FPM)
scripts/gen-vhost.sh --stack=wordpress blog.example.com 203.0.113.10 \
  unix:/run/php/php8.3-fpm.sock /var/www/blog

# Laravel (حدد مجلد public بوضوح)
scripts/gen-vhost.sh --stack=laravel app.example.com 203.0.113.10 \
  unix:/run/php/php8.3-fpm.sock /var/www/app/public

# FastAPI (بروكسي عكسي إلى خادم ASGI، يدعم WebSocket/البث المتدفق)
scripts/gen-vhost.sh --stack=fastapi api.example.com 203.0.113.10 127.0.0.1:8000
```

بعد وضع ملفات الإعداد المُولَّدة (تحت `deploy/generated/`، وهي ضمن
`.gitignore`) في مجلد إعدادات Nginx/Apache وإعادة تحميله، احصل على الشهادة:

```bash
scripts/setup-tls.sh aruaru.example.com admin@example.com /var/www/aruaru.example.com

# تفعيل التحديث التلقائي (مرتين يوميًا) + المراقبة التلقائية (مرة يوميًا، لاكتشاف اقتراب انتهاء الصلاحية)
sudo deploy/systemd/install-systemd-units.sh
```

يمكن التحقق يدويًا في أي وقت من تواريخ انتهاء صلاحية شهادات جميع النطاقات
المسجلة عبر `scripts/check-all-tls.sh`. إذا سجّلت الوجهات نفسها في شاشة
"إدارة المواقع" ضمن واجهة aruaru-web الرسومية، يمكنك عرضها واختبار الاتصال
بها من المتصفح.

## التحقق من العمل (تم إجراؤه في هذا المسار)

- نجح كل من `cargo check --target wasm32-unknown-unknown` و
  `cargo build --target wasm32-unknown-unknown` (بدون أي تحذيرات).
- تم توليد `pkg/aruaru_web.js` / `pkg/aruaru_web_bg.wasm` عبر
  `wasm-bindgen --target web`، وتم تحميل `index.html` في متصفح حقيقي
  (Chromium، عبر Playwright)، وتم التحقق فعليًا من خلال التفاعل معه: عرض
  المواقع المسجلة في شاشة إدارة المواقع، وإضافة موقع جديد، ورفض إدخال منفذ
  غير صالح، وزر اختبار الاتصال (نجح حتى في التحقق من الوصول إلى خادم HTTP
  فعليًا قيد التشغيل)، وحوار تأكيد الحذف (كلاً من الإلغاء والتنفيذ)، وتصدير
  JSON. لا توجد أي أخطاء JavaScript في وحدة التحكم (console).
- تم تثبيت Nginx 1.24 (القياسي في Ubuntu 24.04) وApache 2.4 فعليًا، وتم
  التحقق من صحة الصياغة لنواتج `gen-vhost.sh` لجميع الحزم التقنية الخمس
  (static/proxy/wordpress/laravel/fastapi) باستخدام شهادة موقعة ذاتيًا عبر
  كل من `nginx -t` و`apache2ctl configtest`، وتم تشغيل حزمتي static/proxy
  فعليًا والتحقق من عملهما عبر `curl` (إعادة التوجيه من HTTP إلى HTTPS،
  والبث الثابت، واستجابة 502 عبر البروكسي العكسي).
- لم يتم التحقق من الإصدار الفعلي لشهادة Let's Encrypt عبر certbot (مصادقة
  ACME)، ولا من عمل `scripts/deploy-vps.ps1` في بيئة VPS فعلية، لعدم توفر
  نطاق عام أو VPS فعلي أو بيئة Windows في هذه الجلسة (راجع CLAUDE.md
  للتفاصيل).

## البنية

```text
aruaru-web/
├── Cargo.toml            # crate-type = ["cdylib", "rlib"]، يعتمد على wasm-bindgen/web-sys
├── src/
│   ├── lib.rs             # نقطة الدخول وربط الأحداث
│   ├── dom.rs             # مساعدات مشتركة للتعامل مع DOM (تنزيل الملفات وغيرها)
│   ├── profiles.rs        # إدارة المواقع (ملفات تعريف الاتصال، الحفظ في localStorage، اختبار الاتصال، استيراد/تصدير JSON)
│   └── shell.rs           # هيكل HTML (shell)
├── index.html             # محمّل (loader) يقوم بتحميل pkg/ + CSS
├── pkg/                   # نواتج wasm-bindgen (ضمن .gitignore، تُعاد توليدها عند البناء)
├── scripts/
│   ├── serve.sh            # تشغيل خادم تطوير يبث من أي عنوان IP
│   ├── deploy-vps.ps1       # بناء ورفع وتشغيل على VPS من Windows PowerShell
│   ├── gen-vhost.sh         # توليد إعدادات vhost لـ Nginx وApache من النطاق/IP/الحزمة التقنية
│   ├── setup-tls.sh         # الحصول على شهادة Let's Encrypt
│   ├── check-tls.sh         # التحقق من تاريخ انتهاء شهادة نطاق واحد
│   └── check-all-tls.sh     # التحقق الجماعي من تواريخ انتهاء جميع النطاقات المسجلة
├── deploy/
│   ├── nginx/vhost-{static,proxy,wordpress,laravel,fastapi}.conf.template
│   ├── apache/vhost-{static,proxy,wordpress,laravel,fastapi}.conf.template
│   ├── systemd/             # مجموعة كاملة من المؤقتات للتحديث التلقائي (renew) والمراقبة التلقائية (monitor)
│   └── generated/           # مخرجات gen-vhost.sh (ضمن .gitignore)
└── CLAUDE.md
```

## المشاريع ذات الصلة

نظرًا لأن هذا المستودع أداة عامة للنشر والتشغيل لا تعتمد على قاعدة بيانات
معينة، **يمكن استخدامه جنبًا إلى جنب** مع مشاريع أخرى مثل التالية (يمكن
تسجيلها في شاشة "إدارة المواقع" أو استخدامها كوجهة للبروكسي العكسي عبر
`--stack=proxy`. لا يتدخل هذا المستودع في محتوى أي من هذه المستودعات):

- **aruaru-db**: https://github.com/aon-co-jp/aruaru-db
- **open-runo**: https://github.com/aon-co-jp/open-runo
- **open-web-server**: https://github.com/aon-co-jp/open-web-server
- **poem-cosmo-tauri**: https://github.com/aon-co-jp/poem-cosmo-tauri
- **open-raid-z** (المرجع الأساسي لقواعد التطوير): https://github.com/aon-co-jp/open-raid-z
- **rs-to-readme**: https://github.com/aon-co-jp/rs-to-readme

## License

Apache-2.0
