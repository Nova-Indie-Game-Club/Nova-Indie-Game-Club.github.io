# 添加作品、重建与发布

本项目使用本地 TOML 和图片维护作品，由 Rust + Maud 生成静态网站，不再从 Notion 或游戏平台抓取数据。
**以下命令均在仓库根目录运行。** 模板、静态资源和样式表开发见 [开发与构建](development.md)。

## 1. 准备环境

- 安装 [Rust stable](https://www.rust-lang.org/tools/install)，确保可以运行 `cargo`。
- 提交和发布需要 Git，以及仓库写入权限或通过 Pull Request 贡献的权限。
- 本地预览可使用 [miniserve](https://github.com/svenstaro/miniserve)：

  ```shell
  cargo install miniserve --locked
  ```

日常添加作品不需要 Node.js、Sass、WASM 工具链或框架 CLI。首次下载 Rust 依赖需要网络。

## 2. 创建作品目录

在 `data/works/` 下新建一个目录，或者复制已有作品目录作为起点：

```text
data/works/my-game/
  work.toml
  cover.png
  screenshot-1.png
  screenshot-2.png
```

- 每个作品目录必须有一个名为 `work.toml` 的 UTF-8 文件。
- 建议目录名与 TOML 中的 `id` 一致，例如 `my-game`。
- 复制已有作品后，务必修改 `id`、作者、日期、平台链接和图片，不要保留原作品的信息。
- 不要在 `data/works/` 下放备份目录：读取器会将每个直接子目录视为作品目录。
- 原始 CSV 在 `data/archive/csv-export/` 中仅作归档，**无需修改，也不参与构建**。

### 可复制的 `work.toml` 模板

将以下内容保存为 `data/works/my-game/work.toml`，替换示例信息，并放入引用的图片：

```toml
id = "my-game"
name = "我的游戏"
sub_name = "My Game"
submission_date = "2026-09-01"
class = "Normal"

tags = ["解谜", "冒险"]
gamejams = ["Bevy Jam"]
nova_gamejams = []
cover = "cover.png"
screenshots = ["screenshot-1.png", "screenshot-2.png"]

introduce = """
用一两句话介绍游戏的玩法和特点。

可以直接换行，继续说明操作方式、开发背景等。
"""

authors = [
  { name = "作者甲", url = "https://example.com/" },
  { name = "作者乙" },
]

[platforms]
itch = "https://example.itch.io/my-game"
# steam = "https://store.steampowered.com/app/123456/"
# gamecore = "https://www.gcores.com/games/123456"
# homepage = "https://example.com/"
```

没有截图时改为 `screenshots = []`；没有某个平台或作者主页时，删除相应链接字段，不要留下示例地址或空字符串。

## 3. 字段规则

字段名和枚举值区分大小写。未知字段会导致构建失败。

| 字段 | 是否必填 | 说明 |
| --- | --- | --- |
| `id` | 是 | 全库唯一。仅含小写英文字母、数字和连字符，不能以连字符开头或结尾。发布后保持稳定。 |
| `name` | 是 | 非空作品名称。 |
| `introduce` | 是 | 非空介绍；支持多行纯文本，不解析 Markdown 或 HTML。 |
| `submission_date` | 是 | 投稿日期，写成带引号的 `"YYYY-MM-DD"`，例如 `"2026-09-01"`。 |
| `authors` | 是 | 至少一位作者。每项必须有非空 `name`，可选 `url`。 |
| `sub_name` | 否 | 副标题或英文名；不需要时省略。 |
| `class` | 否 | `"Normal"` 或 `"Spotlight"`；默认 `"Normal"`。 |
| `tags` | 否 | 标签字符串列表，默认 `[]`，列表项不能是空白字符串。 |
| `gamejams` | 否 | 参加的 GameJam 名称列表，默认 `[]`。 |
| `nova_gamejams` | 否 | 社团 GameJam 名称列表，默认 `[]`。 |
| `cover` | 否 | 封面图片的本地相对路径；建议提供。 |
| `screenshots` | 否 | 截图相对路径列表，按列表顺序展示，默认 `[]`。 |
| `[platforms]` | 否 | 可包含 `itch`、`steam`、`gamecore`、`homepage` 四种链接。 |

注意：

- 平台链接和作者链接必须是包含主机名的完整 `http://` 或 `https://` URL。
- TOML 中 `[platforms]` 之后的字段属于这个表。请把 `authors`、`introduce` 等作品字段放在它**之前**。
- 多行介绍可以使用 `"""` 包围；其中双引号和反斜杠仍遵循 TOML 的转义规则。
- 不使用 CSV 的 `None`、`AutoCollection`、Notion 最后编辑时间等旧字段。空列表写 `[]`，可选字符串不需要时省略。
- `sub_name`、标签和 GameJam 等元数据会被读取，但当前页面不一定展示所有字段。

### 首页展示规则

- **近期作品**：按投稿日期倒序，最多展示 9 个；同一天按 `id` 升序排列。修改介绍不会自动改变排序。
- **推荐作品**：展示所有 `class = "Spotlight"` 的作品，顺序同上。
- 作品详情默认展示第一张截图；没有截图时使用封面。

因此，较早的 `Normal` 作品可能不在首页近期列表中，这不一定是读取失败。

## 4. 添加图片

- 将作品图片放在对应作品目录内。推荐使用浏览器常见的 PNG、JPEG、WebP 或 GIF 格式，并适当压缩体积。
- 路径相对于 `work.toml` 所在目录，而不是仓库根目录。
- 可以使用子目录，例如 `screenshots = ["screenshots/level-1.png"]`。
- 路径分隔符使用 `/`，即使是在 Windows 上维护。
- 不允许绝对路径、`..`、远程图片 URL，以及带 `\`、`:`、`?`、`#` 的路径。
- 图片必须真实存在于作品目录内；文件名大小写要完全一致，避免本地 Windows 正常但 Linux CI 找不到文件。
- 删除或重命名图片后，同时更新 TOML 引用。未被引用的作品图片不会自动出现在网站上。

构建会将被引用的图片复制到：

```text
dist/exported/static/assets/local-works/<id>/
```

不要手动维护该输出目录，也不要将作品图片放到旧的 `static/assets/works/` 中。

## 5. 校验、重建与预览

### 仅校验数据并准备图片

```shell
cargo run -p website_data
```

该命令校验作品库，并将引用的图片复制到 `static/assets/local-works/` 缓存，不生成网页。
缓存已被 Git 忽略，不参与 `cargo dev` 的网页构建，无需手动维护或提交。
日常更新也可以直接运行下文的 `cargo dev`，不必先执行这个独立命令。

### 运行测试

```shell
cargo test --workspace --locked
```

测试包含实际作品库校验、模板转义、资源引用和生成器行为。重复 ID、无效日期、缺失图片等问题需要先修复再发布。

### 生成静态网站

```shell
cargo dev
```

`cargo dev` 是 Cargo alias，执行一次生成后退出，**不会启动服务器或监听文件变化**。
生成结果为：

```text
dist/exported/
  index.html
  about/index.html
  static/
    css/
    js/site.js
    assets/
```

生成器先在临时目录处理数据和资源；处理失败时保留上次成功导出。成功后替换输出目录，清除旧的生成文件。
**如果命令失败，预览到的可能仍是旧网页，请先查看并修复终端错误。**

与 CI 相同的发布构建命令：

```shell
cargo dev --release --locked
```

`--release` 优化 Rust 生成器，不是额外的前端打包步骤。依赖已缓存时，也可用 `cargo dev --offline --locked` 离线生成。

### 本地预览

```shell
miniserve dist/exported --index index.html -i 127.0.0.1 -p 8080
```

打开 **http://127.0.0.1:8080/**。不需要 `--hidden`，也不要直接双击 HTML 文件代替 HTTP 预览。
端口占用时可将 `8080` 改为其他端口。

建议检查：

- 新作品的标题、作者、投稿日期和介绍是否正确。
- 封面、详情图片和截图切换是否正常。
- 游戏平台链接是否指向正确作品。
- 推荐作品切换、作品弹窗和手机宽度下的布局是否正常。

修改 TOML 或图片后，重新运行 `cargo dev`，然后刷新浏览器。必要时使用 `Ctrl+Shift+R` 清除页面缓存。

## 6. 提交与发布到 GitHub Pages

### 提交源数据

只提交需要维护的源文件，例如新增作品的 TOML 和图片。`dist/` 和 `target/` 是被忽略的生成目录，**不要强制加入 Git**。
独立数据命令产生的 `static/assets/local-works/` 缓存也不需要提交。

推荐使用分支和 Pull Request。例如新增 `my-game` 时：

```shell
git switch -c works/my-game
git add data/works/my-game
git diff --cached --stat
git commit -m "feat: add my-game"
git push -u origin works/my-game
```

以上是供维护者执行的示例命令。提交前确认暂存区没有混入其他无关修改；没有仓库写权限时，向自己的 Fork 推送并创建 PR。
在 GitHub 上创建以 **`master`** 为目标的 PR，检查通过后合并。

### 自动构建与部署

工作流位于 [`../.github/workflows/deploy.yml`](../.github/workflows/deploy.yml)：

1. 检查 Rust 格式：`cargo fmt --all --check`。
2. 运行测试：`cargo test --workspace --locked`。
3. 检查浏览器脚本语法：`node --check static/js/site.js`（CI 自带 Node.js）。
4. 生成静态网站：`cargo dev --release --locked`。
5. 将 `dist/exported/` 上传为 Pages artifact，并部署到 GitHub Pages。

触发规则：

| 操作 | 检查与构建 | 发布 |
| --- | --- | --- |
| 向 `master` 提交 PR | 是 | 否 |
| PR 合并到 `master`，或直接推送到 `master` | 是 | 是 |
| Actions 页面手动运行工作流 | 是 | 是 |

首次配置时，在仓库 **Settings → Pages → Build and deployment → Source** 中选择 **GitHub Actions**，
不要选择从 `docs/` 目录发布；`docs/` 现在用于维护说明，不是静态网页输出。

发布后，在 **Actions → Build and Deploy Website** 查看 `build` 和 `deploy` 两个任务。
两者成功后，通过 Pages 设置页或 `github-pages` deployment 提供的网址检查线上结果。
仅 `build` 成功或本地生成成功，并不表示线上发布已完成。

如需重建已提交的内容，在 Actions 页面选择 **Build and Deploy Website → Run workflow**，通常选择 `master` 分支。
若部署失败，查看失败步骤日志及仓库的 Pages 权限、环境审批设置；本地生成命令本身不会上传或发布网站。

## 7. 修改或删除已有作品

- **修改内容**：编辑现有 `work.toml` 或图片，保留稳定的 `id`，重新校验、构建并提交。
- **取消推荐**：将 `class` 改为 `"Normal"`，或省略该字段。
- **删除作品**：删除该作品的整个 `data/works/<id>/` 目录，而不只是删除 `work.toml`；重新生成后对应输出图片会被清理。
- 当前读取器不接受完全空的作品库，至少需要保留一个有效作品。

## 常见问题

| 现象 | 检查方法 |
| --- | --- |
| `duplicate id` | 是否复制目录后忘记修改 `id`？ |
| `unknown field` 或枚举错误 | 字段是否拼错？`class` 是否严格为 `Normal` 或 `Spotlight`？字段是否误放在 `[platforms]` 后？ |
| 日期解析失败 | 是否使用带引号、补齐月份和日期位数的有效 `YYYY-MM-DD`？ |
| `cover` / `screenshots` 无法访问 | 图片是否存在？路径是否相对于作品目录？大小写是否一致？ |
| 新作品没有出现在近期列表 | 是否成功重建？投稿日期是否排在前 9 个以内？ |
| 本地网页仍显示旧内容 | 构建是否报错并保留旧输出？浏览器是否需要强制刷新？ |
| PR 已构建但线上没变化 | PR 不部署，需要合并到 `master`，并等待 `deploy` 成功。 |
