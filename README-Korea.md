# aruaru-web

## aruaru-db용 최소한의 웹 UI (Rust → WebAssembly, 프레임워크 미사용)

`aruaru-db`(분산 Git-on-SQL 데이터베이스)가 GraphQL(`/graphql`)로 공개하는
`sql` 쿼리와 `registrySummary`(지원 DB 레지스트리 집계) 쿼리를 브라우저에서
실제로 호출하여 결과를 표시하는 첫 부트스트랩 버전입니다. **아직 본격적인
관리 화면은 아닙니다** — SQL 입력창, 레지스트리 집계 버튼, 결과 테이블
세 가지로 범위를 의도적으로 작게 유지했습니다.

📖 다른 언어: [日本語](README-Japan.md) / [English](README-English.md) /
[中文](README-Chinese.md) / [한국어](README-Korea.md) / [Español](README-Spain.md) /
[Français](README-France.md) / [Deutsch](README-Germany.md) / [Italiano](README-Italy.md) /
[Русский](README-Russia.md) / [العربية](README-Arabic.md)

## 현재 상태

- 실제 `fetch()`로 `aruaru-graphql`의 `/graphql` 엔드포인트에 GraphQL 요청을
  전송(`sql(query: String!)`, `registrySummary`).
- `aruaru-server`가 실행되지 않았거나 연결할 수 없으면, 실제 스키마와
  동일한 형태의 **오프라인 샘플 데이터**를 표시하고 이를 명확히 안내한다
  (실제 브라우저에서 검증 완료).
- GraphQL Mutation, 인증, 페이지네이션은 아직 구현되지 않았다.

## 빠른 시작

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.126
cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg \
  target/wasm32-unknown-unknown/debug/aruaru_web.wasm
python -m http.server 8080   # 이후 http://localhost:8080/index.html 접속
```

## 구성

`Cargo.toml`(`cdylib`/`rlib`, wasm-bindgen/web-sys 의존), `src/lib.rs`(유일한
소스 파일: DOM 구성・fetch・GraphQL 응답 렌더링), `index.html`(`pkg/`를
불러오는 얇은 로더).

## License

Apache-2.0
