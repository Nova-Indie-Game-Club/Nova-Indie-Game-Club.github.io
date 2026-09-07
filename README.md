
# Nova Website
Here is the source of Nova Indie Game Club's website. This website is under construction now.

# Nova 独游社网站
这里是 Nova 独游社网站的仓库，网站正在建设中……

## 开发与构建

网站使用 [Rust](https://www.rust-lang.org/) 和 [Maud](https://maud.lambda.xyz/) 生成纯静态 HTML，
页面交互由少量原生 JavaScript 实现。构建只需安装 Rust，不需要额外框架 CLI、WASM 工具链或 Node.js。

### 构建网站

使用如下命令构建网站：

```shell
cargo dev
```

`cargo dev` 是 `.cargo/config.toml` 中定义的 alias，运行本地静态网站生成器，完成后退出，不启动服务器或文件监听。
它读取 TOML、校验数据、渲染 Maud 模板并复制 CSS、JavaScript 和图片，输出到 `dist/exported/`：

```text
dist/exported/
  index.html
  about/index.html
  static/
    css/
    js/site.js
    assets/
```

构建先在临时目录完成；数据或资源处理失败时保留上次成功的导出，成功后替换输出目录并清理旧文件。
推荐作品切换、介绍选项卡、作品详情弹窗和截图切换均使用本地 `site.js`，无需客户端框架或远程 API。
原有 `docs/` 旧版生成文件已移除；该目录现在存放维护说明，发布目录统一为 `dist/exported/`。

本地预览：

```shell
miniserve dist/exported --index index.html -i 127.0.0.1 -p 8080
```

打开 http://127.0.0.1:8080/ 即可，不需要 `--hidden`。修改数据或源码后重新运行 `cargo dev`，再刷新浏览器。
资源使用相对路径，也支持部署到子目录。

构建直接读取本地作品数据，不需要 Notion Token，也不会抓取 itch.io 等平台的内容。
首次安装 Rust 依赖需要网络；依赖缓存完成后可用 `cargo dev --offline` 离线生成。

### 作品数据维护

完整操作指南见 [添加作品、重建与发布](docs/works_data.md)。

`data/works/<作品 ID>/work.toml` 是作品数据的唯一维护入口，图片与 TOML 放在同一目录：

```text
data/works/quack-on-ice/
  work.toml
  cover.png
  screenshot-1.png
```

新增作品时可以复制已有作品目录，修改 `id`、内容和图片引用。示例：

```toml
id = "my-game"
name = "我的游戏"
sub_name = "My Game"
submission_date = "2024-05-01"
class = "Normal" # Spotlight 表示推荐作品；省略时为 Normal

tags = ["解谜", "鸭子"]
gamejams = ["Bevy Jam"]
nova_gamejams = []
cover = "cover.png"
screenshots = ["screenshot-1.png"]

introduce = """
游戏介绍第一段。

可以直接换行继续编写。
"""

authors = [
  { name = "作者", url = "https://example.com/" },
  { name = "另一位作者" },
]

[platforms]
itch = "https://example.itch.io/my-game"
# steam = "https://store.steampowered.com/app/123456/"
# gamecore = "https://www.gcores.com/games/123456"
# homepage = "https://example.com/"
```

维护规则：

- 必填：`id`、`name`、`introduce`、`submission_date`、`authors`，作者列表至少包含一人。
- `id` 全库唯一，仅使用小写英文字母、数字和中间的连字符；发布后保持稳定，建议目录名与 ID 一致。
- 日期使用带引号的 `YYYY-MM-DD`；近期作品按投稿日期倒序，同日按 ID 排序。
- `sub_name`、`cover`、平台链接和作者链接不需要时直接省略；列表字段省略时默认为空。
- 图片路径相对于作品目录，使用 `/` 分隔；不允许绝对路径、`..` 或远程图片 URL。
- TOML 中 `[platforms]` 之后的字段属于该表，因此作品的其他字段应放在它之前。
- 未知字段、重复 ID、无效日期/链接、缺失图片都会中止构建并提示文件和字段，不会静默显示空列表。
- 不再维护 `AutoCollection`、Notion 最后编辑时间等采集字段；CSV 中 `NovaGameJam` 的 `None` 已转为空列表。

仅校验数据并准备图片（不生成网页）：

```shell
cargo run -p website_data
```

运行全部测试（数据校验、模板转义、资源引用、重复构建和失败保护）：

```shell
cargo test --workspace --locked
```

`cargo dev` 将作品图片直接生成到 `dist/exported/static/assets/local-works/`，不会修改源文件。
独立的 `website_data` 命令仍可生成 `static/assets/local-works/` 缓存；该缓存不参与网页构建，**无需手动维护**。
原始 CSV 保存在 `data/archive/csv-export/`，只作迁移备份，不参与构建。

### GitHub CI / Pages

`.github/workflows/deploy.yml` 会检查 Rust 格式、运行整个 workspace 的测试、检查 JavaScript 语法，
然后执行 `cargo dev --release --locked` 生成网站。
推送到 `master` 或手动触发工作流时，将 `dist/exported/` 作为 Pages artifact 部署；PR 只检查和构建，不部署。
仓库 Settings → Pages 的 Source 应选择 **GitHub Actions**。

### 静态资源

通用静态资源（图片、样式表和 JavaScript）存储在 `static/` 下，作品专属图片存储在 `data/works/` 下。
如果需要修改内容后自动重新生成，可以安装 `watchexec-cli` 并直接运行：

```shell
cargo install watchexec-cli
watchexec --debounce 500ms -w src -w crates -w data -w static -w Cargo.toml -- cargo dev
```

### 样式表开发

样式表位于 `static/css/`。如需监听并编译其中的 SCSS 文件，安装 Sass 后直接运行：

```shell
sass --watch static/css
```

`cargo dev` 使用已提交的 CSS，不依赖 Sass。修改 SCSS 时先用上述命令编译；`interactions.css` 是直接维护的普通 CSS。
接着在另一个终端运行上一小节的监听命令，即可自动更新生成结果。
若此时本地服务器打开，强制刷新网页（通常是 ctrl-shift-r）即可查看修改了的样式表。