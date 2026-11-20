# aruaru-web

## aruaru-db 的最小化 Web UI(Rust → WebAssembly，不使用框架)

这是首个引导版本：在浏览器中真实调用 `aruaru-db`(分布式 Git-on-SQL 数据库)
通过 GraphQL(`/graphql`)公开的 `sql` 查询与 `registrySummary`(支持数据库
注册表汇总)查询，并渲染结果。**目前还不是完整的管理界面** —— 仅有 SQL
输入框、注册表汇总按钮和结果表格，范围刻意保持精简。

📖 其他语言: [日本語](README-Japan.md) / [English](README-English.md) /
[中文](README-Chinese.md) / [한국어](README-Korea.md) / [Español](README-Spain.md) /
[Français](README-France.md) / [Deutsch](README-Germany.md) / [Italiano](README-Italy.md) /
[Русский](README-Russia.md) / [العربية](README-Arabic.md)

## 现状

- 通过真实的 `fetch()` 向 `aruaru-graphql` 的 `/graphql` 端点发送 GraphQL 请求
  (`sql(query: String!)`、`registrySummary`)。
- 若 `aruaru-server` 未运行或无法连接，会显示与真实 schema 完全一致的**离线
  示例数据**，并明确标注为离线示例(已在真实浏览器中验证)。
- GraphQL Mutation、认证、分页等尚未实现。

## 快速开始

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.126
cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir pkg \
  target/wasm32-unknown-unknown/debug/aruaru_web.wasm
python -m http.server 8080   # 然后打开 http://localhost:8080/index.html
```

## 项目结构

`Cargo.toml`(`cdylib`/`rlib`，依赖 wasm-bindgen/web-sys)、
`src/lib.rs`(唯一源文件：DOM 构建・fetch・GraphQL 响应渲染)、
`index.html`(加载 `pkg/` 的薄封装)。

## License

Apache-2.0
