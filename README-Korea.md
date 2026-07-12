# aruaru-web

**aruaru-db용 최소한의 Web UI(Rust → WebAssembly, 프레임워크 미사용)**

`aruaru-db`(분산 Git-on-SQL 데이터베이스)가 GraphQL(`/graphql`)로 공개하는
`sql` 쿼리와 `registrySummary`(지원 DB 레지스트리 집계)를 브라우저에서
직접 호출하여 결과를 보여주는 탭 방식 대시보드입니다. SQL 실행・레지스트리
집계 기능에 더해, KUSANAGI의 사이트 목록처럼 **여러 접속 대상(aruaru-web용・
다른 프로젝트용)을 등록하고 전환할 수 있는 "사이트 관리" 탭**을 갖추고
있습니다.

📖 다른 언어: [日本語](README-Japan.md) / [English](README-English.md) /
[中文](README-Chinese.md) / [한국어](README-Korea.md) / [Español](README-Spain.md) /
[Français](README-France.md) / [Deutsch](README-Germany.md) / [Italiano](README-Italy.md) /
[Русский](README-Russia.md) / [العربية](README-Arabic.md)

---

## 지금 할 수 있는 것

- `aruaru-graphql`(`aruaru-db/crates/aruaru-graphql`)의 `/graphql` 엔드포인트로
  실제로 `fetch()`를 통해 GraphQL 요청을 전송:
  - `sql(query: String!): QueryResultGql` — 임의의 SQL을 실행하여 `columns`/`rows`/`commandTag`를
    테이블로 표시
  - `registrySummary: RegistrySummaryGql` — 지원 DB 레지스트리(150건 이상)의
    집계를 카드 형태로 표시
  - `registry: [DbEntryGql!]!` — 지원 DB 레지스트리의 **목록**을 테이블로 표시
    (이름/카테고리/와이어 호환성/상태/순위/점수/갱신 일시)
- **버전 관리 탭**: `currentBranch`/`branches`(브랜치 목록・현재 브랜치),
  `log(limit)`(커밋 로그), `diff(from, to)`(브랜치 간 차이의 추가/삭제/변경
  건수)를 실행할 수 있다. 모두 `aruaru-db/crates/aruaru-graphql`의 실제
  스키마(`VcsQuery`)에 기반한다.
- `aruaru-server`가 실행되지 않았거나 연결할 수 없는 경우, **실제 스키마와
  동일한 형태의 샘플 데이터**를 즉시 렌더링하고 "오프라인 샘플"임을 명시한다
  (이 동작은 실제 브라우저에서 검증 완료 — 아래 "동작 확인" 참고).
- **사이트 관리 탭**: aruaru-web용・다른 프로젝트용 접속 대상(IP 주소/
  도메인/서브도메인/포트/경로)을 여러 개 등록하여 `localStorage`에 저장하고
  클릭 한 번으로 전환할 수 있다. SQL/레지스트리 탭의 엔드포인트 입력란은
  선택된 사이트를 자동으로 따라간다. 카드마다 접속을 전환하지 않고도 연결
  상태를 확인할 수 있는 **"연결 테스트" 버튼**, 포트 번호 입력 검증(1~65535),
  등록된 사이트 목록의 **JSON 내보내기/가져오기**(백업・다른 브라우저로
  이전용), 삭제 전 확인 대화상자를 갖추고 있다.
- **SQL 탭의 사용성**: 최근 10건의 **쿼리 이력**(클릭으로 다시 불러오기・
  마우스를 올리면 전체 내용 표시), **Ctrl+Enter / Cmd+Enter 실행 단축키**,
  실행 결과의 **CSV 내보내기**, 실행 중 버튼 비활성화, 행 수 표시・
  스크롤 가능하고 헤더가 고정된 결과 테이블.
- **HTTPS(TLS)의 자동 설정・자동 감시・자동 갱신**: `scripts/gen-vhost.sh`로
  Nginx/Apache의 vhost(HTTP→HTTPS 리다이렉트 포함)를 생성하고,
  `scripts/setup-tls.sh`로 Let's Encrypt(certbot) 인증서를 발급하며,
  `deploy/systemd/install-systemd-units.sh`로 "하루 2회 자동 갱신
  (`aruaru-tls-renew.timer`)"과 "하루 1회 만료 감시
  (`aruaru-tls-monitor.timer` → `scripts/check-all-tls.sh`)"를 활성화할 수 있다.
  자세한 내용은 "HTTPS・도메인/서브도메인 등록" 항목을 참고.

## 아직 할 수 없는 것(솔직한 범위)

- GraphQL Mutation(브랜치 생성・병합・레지스트리 크롤 등)은 아직 구현되지
  않았다.
- 인증・페이지네이션・오류 발생 시 자동 재시도는 아직 구현되지 않았다.
- Tauri와 같은 네이티브 앱 경험은 제공하지 않는다(브라우저에서 동작하는
  WASM만 제공).
- **실제 도메인 취득・DNS 레코드 등록(레지스트라 조작)은 이 저장소에서
  수행하지 않는다**(비용 발생 및 외부 서비스 반영을 수반하기 때문). 여기서
  자동화하는 범위는 이미 취득한 도메인에 대한 "vhost 설정 생성"과 "TLS
  인증서 발급・감시・자동 갱신"까지이며, DNS 등록 자체는 이용자가 레지스트라
  에서 직접 수행해야 한다.

## 빌드 방법

Node.js・npm・TypeScript는 사용하지 않는다. Rust 툴체인만으로 완결된다.

```bash
rustup target add wasm32-unknown-unknown        # 최초 1회만
cargo install wasm-bindgen-cli --version 0.2.126 # 최초 1회만(Cargo.lock의 버전과 일치시킬 것)

cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg \
  target/wasm32-unknown-unknown/debug/aruaru_web.wasm

# 정적 서버로 배포하여 열기(무엇이든 상관없음. 예:)
python -m http.server 8080
# 브라우저에서 http://localhost:8080/index.html 을 연다
```

`aruaru-db`를 실제로 구동해서 테스트하는 경우:

```bash
cd ../aruaru-db
cargo run -p aruaru-server -- --data ./data --raft-id 1   # :4000 에 GraphQL이 뜬다
```

## IP 주소로 실행하기

```bash
scripts/serve.sh 0.0.0.0 8080        # 모든 인터페이스에서 대기
scripts/serve.sh 192.168.1.50 8080   # 특정 IP 주소에서만 대기
```

## HTTPS・도메인/서브도메인 등록

이 저장소 자체는 도메인 취득이나 DNS 레코드 등록을 수행하지 않는다(레지스트라
조작・비용이 발생하기 때문에 이용자가 별도로 진행). 다음은 이미 취득한 도메인/
서브도메인을 aruaru-web용・다른 프로젝트용으로 간편하게 배정하기 위한 로컬
자동화(KUSANAGI의 "사이트 추가"에 해당).

```bash
# 1. 도메인+IP+백엔드로부터 vhost(Nginx/Apache, HTTP→HTTPS 리다이렉트 포함)를 생성
scripts/gen-vhost.sh aruaru.example.com 203.0.113.10 127.0.0.1:4000
# 다른 용도의 서브도메인도 동일한 방식으로(UPSTREAM/WEBROOT만 바꾸면 됨)
scripts/gen-vhost.sh tool.example.com 203.0.113.10 127.0.0.1:9000 /var/www/tool

# 2. 생성된 설정 파일을 배치하고 리로드(deploy/generated/ 아래, .gitignore 대상)

# 3. TLS 인증서를 발급(Let's Encrypt / certbot)
scripts/setup-tls.sh aruaru.example.com admin@example.com /var/www/aruaru.example.com

# 4. 자동 갱신(하루 2회) + 자동 감시(하루 1회, 만료 임박 감지)를 활성화
sudo deploy/systemd/install-systemd-units.sh
```

등록된 모든 도메인의 인증서 만료일은 `scripts/check-all-tls.sh`로 언제든지
수동으로 확인할 수 있다. aruaru-web의 GUI 쪽 "사이트 관리" 탭에도 동일한
접속 대상을 등록해 두면, 브라우저에서의 접속 대상 전환과 일치시킬 수 있다.

## 동작 확인(이번 패스에서 실시)

- `cargo check --target wasm32-unknown-unknown` / `cargo build --target wasm32-unknown-unknown`
  모두 성공(경고 0건).
- `wasm-bindgen --target web`로 `pkg/aruaru_web.js` / `pkg/aruaru_web_bg.wasm`을 생성하고,
  실제 브라우저(Chromium, Playwright 경유)에서 `index.html`을 불러와 다음을 실제로
  조작하여 확인 완료: 탭 전환, SQL 실행→오프라인 폴백 렌더링, 쿼리
  이력에 기록・다시 불러오기・마우스 오버 시 툴팁, Ctrl+Enter 단축키,
  CSV 내보내기(실제 다운로드 발생), 레지스트리 집계・등록 DB 목록 조회,
  버전 관리 탭에서의 브랜치 목록・커밋 로그・Diff 조회(모두 오프라인
  폴백 포함), 사이트 관리 탭에서의 등록된 사이트 표시・신규 추가・잘못된
  포트 입력 거부・연결 테스트 버튼・JSON 내보내기/가져오기(라운드트립
  확인 완료)・삭제 확인 대화상자(취소/실행 양쪽 모두). 콘솔상의 JS 오류는
  없음(의도된 접속 실패 로그만 발생).
- Nginx 1.24(Ubuntu 24.04 표준)・Apache 2.4・certbot을 실제로 설치하여,
  `scripts/gen-vhost.sh`의 생성물을 자체 서명 인증서로 실제 구동・`curl` 검증
  (HTTP→HTTPS 리다이렉트, ACME challenge 경로, `/graphql` 리버스 프록시),
  `scripts/check-tls.sh`를 실제 HTTPS 서버에 대해 실행하여 WARN/healthy/
  ERROR 3가지 상태를 확인, `deploy/systemd/*`를 `systemd-analyze verify`로
  검증(오류 0건). 이 과정에서 Nginx vhost 템플릿의 실제 버그(`http2 on;`가
  Nginx 1.24에서 구문 오류가 되는 문제)를 발견・수정 완료. 실제 certbot에
  의한 Let's Encrypt 발급(ACME 인증)은 퍼블릭 도메인이 없다는 점과 검증
  환경의 Python ABI 불일치로 인해 미검증(자세한 내용은 CLAUDE.md 참고).

## 구성

```text
aruaru-web/
├── Cargo.toml            # crate-type = ["cdylib", "rlib"], wasm-bindgen/web-sys 의존
├── src/
│   ├── lib.rs             # 진입점・탭 전환・이벤트 배선
│   ├── dom.rs             # DOM 조작 공통 헬퍼(파일 다운로드 등)
│   ├── graphql.rs         # /graphql 로의 fetch 호출
│   ├── render.rs          # SQL 결과・레지스트리 집계 렌더링, CSV 출력
│   ├── profiles.rs        # 사이트 관리(접속 프로필, localStorage 저장, JSON 입출력)
│   ├── history.rs         # SQL 쿼리 이력(최근 10건, localStorage 저장)
│   └── shell.rs           # HTML 셸(탭・폼)
├── index.html             # pkg/ 를 불러오는 로더 + CSS
├── pkg/                   # wasm-bindgen 생성물(.gitignore 대상, 빌드 시 재생성)
├── scripts/
│   ├── serve.sh            # 임의의 IP 주소로 배포하는 개발 서버 실행
│   ├── gen-vhost.sh         # 도메인/IP로부터 Nginx・Apache vhost 생성
│   ├── setup-tls.sh         # Let's Encrypt 인증서 발급
│   ├── check-tls.sh         # 도메인 1개의 인증서 만료일 확인
│   └── check-all-tls.sh     # 등록된 모든 도메인의 만료일을 일괄 확인
├── deploy/
│   ├── nginx/vhost.conf.template
│   ├── apache/vhost.conf.template
│   ├── systemd/             # 자동 갱신(renew)・자동 감시(monitor) 타이머 일체
│   └── generated/           # gen-vhost.sh의 출력(.gitignore 대상)
└── CLAUDE.md
```

## 관련 프로젝트

- **aruaru-db**(이 UI가 접속하는 대상): https://github.com/aon-co-jp/aruaru-db
- **open-runo**: https://github.com/aon-co-jp/open-runo
- **open-web-server**: https://github.com/aon-co-jp/open-web-server
- **poem-cosmo-tauri**: https://github.com/aon-co-jp/poem-cosmo-tauri
- **open-raid-z**(개발 규칙의 정본): https://github.com/aon-co-jp/open-raid-z
- **rs-to-readme**: https://github.com/aon-co-jp/rs-to-readme

## License

Apache-2.0
