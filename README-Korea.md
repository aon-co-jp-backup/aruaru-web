# aruaru-web

**"제2의 KUSANAGI" — 앱을 업로드한 후 IP 주소로 실행하고,
도메인 등록·HTTPS화를 간편하게 자동 적용할 수 있는 운영 도구(Rust →
WebAssembly, 프레임워크 미사용)**

WordPress 고속화 서버 구축 키트인 "KUSANAGI"처럼, 앱을 업로드하면
**IP 주소로 실행 → 도메인 등록 간소화 → HTTPS 자동화**까지 일련의 과정을
한 번에 처리하는 것을 목표로 하는 운영 도구입니다. WordPress·PHP +
Laravel·Python + FastAPI 등 임의의 백엔드 스택으로 구성된 웹사이트를
고속화하는 리버스 프록시 설정(Nginx/Apache)을 자동 생성할 수 있으며,
여러 사이트의 접속 대상을 등록·전환·연결 확인할 수 있는 "사이트 관리"
화면을 갖추고 있습니다. **DB(데이터베이스) 접속 기능은 가지고 있지
않습니다**(의도적으로 범위 밖).

📖 다른 언어: [日本語](README-Japan.md) / [English](README-English.md) /
[中文](README-Chinese.md) / [한국어](README-Korea.md) / [Español](README-Spain.md) /
[Français](README-France.md) / [Deutsch](README-Germany.md) / [Italiano](README-Italy.md) /
[Русский](README-Russia.md) / [العربية](README-Arabic.md)

---

## 지금 할 수 있는 것

- **사이트 관리 화면**: aruaru-web 자신·WordPress·Laravel·FastAPI 등
  임의의 백엔드 스택에 대한 배포 대상(IP 주소/도메인/서브도메인/포트/
  경로)을 여러 개 등록하여 `localStorage`에 저장하고, 클릭 한 번으로
  선택·연결 확인할 수 있다(KUSANAGI의 사이트 목록에 해당). 카드마다
  **"연결 테스트" 버튼**(선택 중인 사이트를 바꾸지 않고 단순 HTTP
  도달 가능 여부만 확인), 포트 번호 입력 검증(1~65535), 등록된 사이트
  목록의 **JSON 내보내기/가져오기**(백업·다른 브라우저로 이전용),
  삭제 전 확인 대화상자를 갖춘다.
- **IP 주소로 실행**: `scripts/serve.sh`로 로컬/VPS상의 임의의 IP·포트에
  바인딩하여 서비스할 수 있다.
- **vhost 생성·고속화·HTTPS 자동 설정**: `scripts/gen-vhost.sh`로
  도메인·IP·백엔드 스택의 조합에서 Nginx/Apache의 vhost(HTTP→HTTPS
  리다이렉트 포함)를 생성한다. `static`(정적 사이트)·`proxy`(aruaru-db·
  open-web-server·open-raid-z 계열이나 임의의 HTTP 백엔드를 위한 범용
  리버스 프록시)·`wordpress`·`laravel`·`fastapi`의 5가지 스택을 지원하며,
  gzip 압축·정적 자산의 장기 캐시·upstream keepalive·FastCGI 버퍼
  조정 등 스택별 고속화 설정을 포함한다.
- **HTTPS(TLS) 자동 모니터링·자동 갱신**: `scripts/setup-tls.sh`로
  Let's Encrypt(certbot) 인증서 발급, `deploy/systemd/
  install-systemd-units.sh`로 "하루 2회 자동 갱신
  (`aruaru-tls-renew.timer`)"과 "하루 1회 만료 모니터링
  (`aruaru-tls-monitor.timer` → `scripts/check-all-tls.sh`)"을
  활성화할 수 있다.
- **VPS로의 배포**: Windows PowerShell에서 `scripts/deploy-vps.ps1`을
  실행하기만 하면 빌드 → VPS 업로드 → 실행까지 자동화할 수 있다
  (자세한 내용은 아래 "VPS로의 배포" 참조).

## 아직 할 수 없는 것(정직한 범위)

- **DB(데이터베이스) 접속 기능은 가지고 있지 않다**. SQL 실행·GraphQL
  쿼리 등 특정 데이터베이스 제품에 의존하는 기능은 의도적으로 범위 밖
  이며, 앞으로도 구현하지 않는다. aruaru-db와 같은 특정 백엔드를
  사용하는 경우에도 "사이트 관리" 화면이나 vhost 생성
  (`--stack=proxy`)은 **범용 리버스 프록시·배포 관리**로 활용할 수
  있지만, 해당 DB에 고유한 쿼리 기능은 제공하지 않는다.
- 인증·페이지네이션·오류 시 자동 재시도는 미구현이다.
- Tauri와 같은 네이티브 앱 경험은 제공하지 않는다(브라우저에서 동작하는
  WASM뿐이다).
- **실제 도메인 취득·DNS 레코드 등록(등록기관에서의 작업)은 이 저장소
  에서는 수행하지 않는다**(비용·외부 서비스 반영을 수반하기 때문).
  여기서 자동화하는 것은 이미 취득한 도메인에 대한 "vhost 설정 생성"과
  "TLS 인증서 발급·모니터링·자동 갱신"까지이며, DNS 등록 자체는
  이용자가 등록기관에서 직접 수행한다.
- 실제 VPS 계약(렌탈 서버 사업자와의 계약)도 이 저장소에서는 수행하지
  않는다.

## 빌드 방법

Node.js·npm·TypeScript는 사용하지 않는다. Rust 툴체인만으로 완결된다.

```bash
rustup target add wasm32-unknown-unknown        # 최초 1회만
cargo install wasm-bindgen-cli --version 0.2.126 # 최초 1회만(Cargo.lock의 버전과 일치시킬 것)

cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg \
  target/wasm32-unknown-unknown/debug/aruaru_web.wasm

# 정적 서버로 배포하여 연다(무엇이든 상관없다. 예:)
python -m http.server 8080
# 브라우저에서 http://localhost:8080/index.html 을 연다
```

## IP 주소로 실행하기

```bash
scripts/serve.sh 0.0.0.0 8080        # 모든 인터페이스에서 대기
scripts/serve.sh 192.168.1.50 8080   # 특정 IP 주소에서만 대기
```

실행 후 **브라우저 주소창에 직접 IP 주소를 입력**하면 확인할 수 있다
(예: `http://192.168.1.50:8080/index.html`). 도메인 등록 전이라도
IP 주소만으로 동작을 확인할 수 있다는 점이 포인트다.

## VPS로의 배포(Windows PowerShell에서)

VPS 렌탈 서버를 빌린 뒤, Windows PowerShell에서
`scripts/deploy-vps.ps1`을 실행하기만 하면 빌드→업로드→실행까지
자동화할 수 있다. `open-web-server`를 함께 사용하는 경우 동시에
업로드할 수도 있다(이 저장소에서는 `open-web-server`의 내부에는
관여하지 않고, 업로드 대상 경로만 지정한다).

```powershell
# 업로드하여 실행까지 진행하는 경우(aruaru-web만)
.\scripts\deploy-vps.ps1 -VpsHost 203.0.113.10 -VpsUser root -StartServer

# open-web-server (F:\open-runo\open-web-server) 도 함께 업로드하는 경우
.\scripts\deploy-vps.ps1 -VpsHost 203.0.113.10 -VpsUser root -StartServer `
    -OpenWebServerPath "F:\open-runo\open-web-server"
```

이는 내부적으로 다음과 동등한 처리를 수행한다(수동으로 실행해도 된다):

```powershell
# 1. 로컬에서 빌드
cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg `
    target/wasm32-unknown-unknown/debug/aruaru_web.wasm

# 2. VPS로 업로드(OpenSSH 클라이언트, Windows 10 1809 이후/11에 기본 탑재)
ssh root@203.0.113.10 "mkdir -p /root/aruaru-web"
scp -r .\index.html .\pkg .\scripts .\deploy .\Cargo.toml .\src `
    root@203.0.113.10:/root/aruaru-web/

# 3. VPS에서 IP 주소로 실행
ssh root@203.0.113.10 "cd /root/aruaru-web && bash scripts/serve.sh 0.0.0.0 8080"
```

실행 후 **브라우저 주소창에 VPS의 IP 주소를 입력**한다(예:
`http://203.0.113.10:8080/index.html`). 로컬의
`F:\open-runo\aruaru-web` 등을 항상 최신 상태로 유지하고 싶다면,
이 저장소를 `git pull`하기만 하면 된다:

```powershell
cd F:\open-runo\aruaru-web
git fetch origin
git pull origin <브랜치명>
```

**업로드하지 않아도 사용할 수 있다**: VPS를 사용하지 않고 로컬에서만
시도하는 경우, 위의 "IP 주소로 실행하기"와 같이 `scripts/serve.sh`를
그대로 로컬에서 실행하면 된다. 같은 LAN 내의 다른 기기에서는
`http://<로컬 PC의 IP 주소>:8080/`으로 접속할 수 있다.

## vhost 생성·고속화·도메인/서브도메인 등록

이 저장소 자체는 도메인 취득이나 DNS 레코드 등록을 수행하지 않는다
(등록기관에서의 작업·비용이 발생하므로 이용자가 별도로 수행한다).
아래는 이미 취득한 도메인/서브도메인에 대해, 고속화 설정을 포함한
리버스 프록시를 스택별로 간편하게 발급하기 위한 로컬 자동화다
(KUSANAGI의 "사이트 추가"에 해당).

```bash
# aruaru-web 자신(정적 사이트)
scripts/gen-vhost.sh --stack=static aruaru.example.com 203.0.113.10

# aruaru-db·open-web-server·open-raid-z 계열이나 임의의 백엔드를 위한 범용 리버스 프록시
scripts/gen-vhost.sh --stack=proxy tool.example.com 203.0.113.10 127.0.0.1:9000

# WordPress(PHP-FPM 소켓/주소 지정)
scripts/gen-vhost.sh --stack=wordpress blog.example.com 203.0.113.10 \
  unix:/run/php/php8.3-fpm.sock /var/www/blog

# Laravel(public 디렉터리를 명시)
scripts/gen-vhost.sh --stack=laravel app.example.com 203.0.113.10 \
  unix:/run/php/php8.3-fpm.sock /var/www/app/public

# FastAPI(ASGI 서버로의 리버스 프록시, WebSocket/스트리밍 지원)
scripts/gen-vhost.sh --stack=fastapi api.example.com 203.0.113.10 127.0.0.1:8000
```

생성된 설정 파일(`deploy/generated/` 아래, `.gitignore` 대상)을
Nginx/Apache의 설정 디렉터리에 배치하고 리로드한 후, 인증서를
발급받는다:

```bash
scripts/setup-tls.sh aruaru.example.com admin@example.com /var/www/aruaru.example.com

# 자동 갱신(하루 2회) + 자동 모니터링(하루 1회, 만료 임박 감지)을 활성화
sudo deploy/systemd/install-systemd-units.sh
```

등록된 모든 도메인의 인증서 유효기간은 `scripts/check-all-tls.sh`로
언제든지 수동 확인할 수 있다. aruaru-web의 GUI 쪽 "사이트 관리"
화면에도 동일한 접속 대상을 등록해 두면, 브라우저 쪽에서 목록 확인·
연결 테스트를 할 수 있다.

## 동작 확인(이번 작업에서 실시)

- `cargo check --target wasm32-unknown-unknown` / `cargo build --target wasm32-unknown-unknown`
  모두 성공(경고 0건).
- `wasm-bindgen --target web`으로 `pkg/aruaru_web.js` /
  `pkg/aruaru_web_bg.wasm`을 생성하고, 실제 브라우저(Chromium,
  Playwright 경유)에서 `index.html`을 로드하여 다음을 실제로 조작해
  확인 완료: 사이트 관리 화면에서 등록된 사이트 표시·신규 추가·잘못된
  포트 입력 거부·연결 테스트 버튼(실제로 가동 중인 HTTP 서버로의 도달
  가능 여부 확인까지 성공)·삭제 확인 대화상자(취소/실행 양쪽 모두)·
  JSON 내보내기. 콘솔상 JS 오류는 없음.
- Nginx 1.24(Ubuntu 24.04 표준)·Apache 2.4를 실제로 도입하여,
  `gen-vhost.sh`의 전체 5개 스택(static/proxy/wordpress/laravel/
  fastapi)의 생성물을 자체 서명 인증서로 `nginx -t` /
  `apache2ctl configtest` 양쪽 모두 구문 검증, static/proxy 스택은
  실제로 기동하여 `curl`로 기능 검증(HTTP→HTTPS 리다이렉트, 정적
  배포, 리버스 프록시 경유의 502 응답)까지 확인 완료.
- 실제 certbot에 의한 Let's Encrypt 발급(ACME 인증), 그리고
  `scripts/deploy-vps.ps1`의 실제 VPS 환경에서의 동작은, 퍼블릭
  도메인·실제 VPS·Windows 환경이 이번 세션에는 없어 미검증(자세한
  내용은 CLAUDE.md 참조).

## 구성

```text
aruaru-web/
├── Cargo.toml            # crate-type = ["cdylib", "rlib"], wasm-bindgen/web-sys 의존
├── src/
│   ├── lib.rs             # 진입점・이벤트 연결
│   ├── dom.rs             # DOM 조작 공통 헬퍼(파일 다운로드 등)
│   ├── profiles.rs        # 사이트 관리(접속 프로필, localStorage 저장, 연결 테스트, JSON 입출력)
│   └── shell.rs           # HTML 셸
├── index.html             # pkg/ 를 로드하는 로더 + CSS
├── pkg/                   # wasm-bindgen 생성물(.gitignore 대상, 빌드 시 재생성)
├── scripts/
│   ├── serve.sh            # 임의의 IP 주소에서 배포하는 개발 서버 실행
│   ├── deploy-vps.ps1       # Windows PowerShell에서 VPS로 빌드・업로드・실행
│   ├── gen-vhost.sh         # 도메인/IP/스택으로 Nginx・Apache vhost 생성
│   ├── setup-tls.sh         # Let's Encrypt 인증서 발급
│   ├── check-tls.sh         # 단일 도메인의 인증서 유효기간 확인
│   └── check-all-tls.sh     # 등록된 전체 도메인의 유효기간을 일괄 확인
├── deploy/
│   ├── nginx/vhost-{static,proxy,wordpress,laravel,fastapi}.conf.template
│   ├── apache/vhost-{static,proxy,wordpress,laravel,fastapi}.conf.template
│   ├── systemd/             # 자동 갱신(renew)・자동 모니터링(monitor) 타이머 일체
│   └── generated/           # gen-vhost.sh의 출력(.gitignore 대상)
└── CLAUDE.md
```

## 관련 프로젝트

이 저장소는 DB에 의존하지 않는 범용 배포・운영 도구이므로, 아래와
같은 다른 프로젝트와도 **함께 사용 가능**하다("사이트 관리" 화면에
등록하거나 `--stack=proxy`로 리버스 프록시 대상으로 활용할 수 있다.
각 저장소의 내부에는 관여하지 않는다):

- **aruaru-db**: https://github.com/aon-co-jp/aruaru-db
- **open-runo**: https://github.com/aon-co-jp/open-runo
- **open-web-server**: https://github.com/aon-co-jp/open-web-server
- **poem-cosmo-tauri**: https://github.com/aon-co-jp/poem-cosmo-tauri
- **open-raid-z**(개발 규칙의 정본): https://github.com/aon-co-jp/open-raid-z
- **rs-to-readme**: https://github.com/aon-co-jp/rs-to-readme

## License

Apache-2.0
