# 开发与构建

本文介绍网站代码、静态资源和样式表的开发流程。作品数据维护及发布操作见 [添加作品、重建与发布](works_data.md)。
所有命令均在仓库根目录执行。

## 技术与环境

网站使用 [Rust](https://www.rust-lang.org/) 和 [Maud](https://maud.lambda.xyz/) 生成纯静态 HTML。
介绍选项卡、推荐作品切换、作品详情弹窗和截图切换由 `static/js/site.js` 中的原生 JavaScript 实现。

- 构建只需安装 [Rust](https://www.rust-lang.org/tools/install)，无需框架 CLI、WASM 工具链或 Node.js。
- 构建读取本地 TOML 和图片，不需要 Notion Token，也不会抓取 itch.io 等平台内容。
- 首次下载 Rust 依赖需要网络；缓存完成后可以离线生成。
- miniserve、watchexec 和 Sass 均为可选开发工具，分别用于预览、监听变更和编译 SCSS。

## 目录约定

| 路径 | 用途 |
| --- | --- |
| `src/templates/` | Maud 页面模板及介绍文案 |
| `src/components/` | 作品展示、页脚等复用组件 |
| `crates/website_data/` | 本地作品数据读取、校验和图片准备 |
| `crates/website_model/` | 页面使用的作品模型 |
| `data/works/` | 人工维护的作品 TOML 和图片 |
| `static/` | 通用图片、CSS、SCSS 和 JavaScript |
| `docs/` | 维护说明，不是网站发布目录 |
| `dist/exported/` | 生成的静态网站，不应手动编辑或提交 |

旧版 `docs/` 中的生成网页已移除，发布目录统一为 `dist/exported/`。

## 构建与预览

```shell
cargo dev
```

`cargo dev` 是 [`.cargo/config.toml`](../.cargo/config.toml) 中定义的 alias，运行本地静态网站生成器。
它读取 TOML、校验数据、渲染模板，并将 CSS、JavaScript 和图片复制到输出目录：

```text
dist/exported/
  index.html
  about/index.html
  static/
    css/
    js/site.js
    assets/
```

生成器先在临时目录完成处理；数据或资源处理失败时保留上次成功的导出。
成功后替换输出目录并清理旧的生成文件，不修改作品源数据。
命令完成后退出，**不会启动服务器或监听文件变化**。

其他构建方式：

```shell
# 与 CI 相同的发布构建
cargo dev --release --locked

# 依赖已缓存时离线生成
cargo dev --offline --locked
```

安装 [miniserve](https://github.com/svenstaro/miniserve) 后预览：

```shell
cargo install miniserve --locked
miniserve dist/exported --index index.html -i 127.0.0.1 -p 8080
```

打开 http://127.0.0.1:8080/，不需要 `--hidden`。
资源使用相对路径，也支持将导出目录部署到网站子目录。
修改源码或数据后重新生成，再刷新浏览器；必要时使用 `Ctrl+Shift+R` 强制刷新。

## 测试与检查

```shell
cargo fmt --all --check
cargo test --workspace --locked
```

测试覆盖作品数据校验、模板转义、资源引用、重复构建和失败保护。
修改 Rust 代码后可用 `cargo fmt --all` 自动格式化。

修改浏览器脚本时，若本地已安装 Node.js，可额外检查语法：

```shell
node --check static/js/site.js
```

这不是网站生成的前置要求。交互变更还应在浏览器中检查作品切换、图片切换、弹窗操作及手机布局。

## 静态资源与自动重建

通用图片、样式表和 JavaScript 存储在 `static/` 下，作品专属图片存储在 `data/works/` 下。
`cargo dev` 会将作品图片直接生成到 `dist/exported/static/assets/local-works/`。
旧的 `static/assets/works/` 和独立数据命令产生的 `static/assets/local-works/` 缓存都不参与网页构建。

如果需要文件变化后自动重新生成，可以安装 `watchexec-cli` 并运行：

```shell
cargo install watchexec-cli
watchexec --debounce 500ms -w src -w crates -w data -w static -w Cargo.toml -- cargo dev
```

在另一个终端保持 miniserve 运行。监听命令只负责重新生成，不会自动刷新浏览器。

## 样式表开发

样式表位于 `static/css/`。`cargo dev` 使用已提交的 CSS，**不会自动编译 SCSS**。
修改 SCSS 时，安装 [Sass](https://sass-lang.com/install/) 后运行：

```shell
sass --watch static/css
```

- 保持 Sass 监听运行，将 SCSS 修改编译为 CSS。
- `static/css/interactions.css` 是直接维护的普通 CSS，没有对应的 SCSS 源文件。
- 在另一个终端运行上一节的 watchexec 命令，或手动执行 `cargo dev`，将最新 CSS 放入导出目录。
- 刷新浏览器检查效果；提交时同时提交修改后的 SCSS 和编译生成的 CSS，存在对应 source map 时也同步更新。

## GitHub CI / Pages

[部署工作流](../.github/workflows/deploy.yml) 会检查 Rust 格式、运行整个 workspace 的测试、检查 JavaScript 语法，
然后执行 `cargo dev --release --locked`，将 `dist/exported/` 上传为 Pages artifact。

- 推送到 `master` 或手动运行工作流时部署到 GitHub Pages。
- 向 `master` 提交的 PR 只检查和构建，不部署。
- 仓库 **Settings → Pages → Source** 应选择 **GitHub Actions**，而不是从 `docs/` 发布。

提交步骤、手动发布和排查方法见 [作品维护与发布指南](works_data.md)。
