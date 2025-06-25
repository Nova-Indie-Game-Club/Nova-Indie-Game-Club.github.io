
# Nova Website
Here is the source of Nova Indie Game Club's website. This website is under construction now.

# Nova 独游社网站
这里是 Nova 独游社网站的仓库，网站正在建设中……

## 开发与构建

### 构建网站

网站基于 Rust 和 perseus 框架，因此首先需要确保计算机：

1. 安装 Rust：https://www.rust-lang.org/tools/install
2. 安装 `perseus-cli`：`cargo install perseus-cli`

接着使用：

```shell
perseus export -s
```

会输出静态网页到项目的 `dist/exported/`，`-s` 代表同时运行一个本地服务器。
使用本地服务器地址即可查看网页。

### 静态资源

项目的静态资源（包括图片、样式表等）存储在 `static` 文件夹下。如果需要实时监控和同步这个文件夹到 `dist/exported/`，
可以安装 `watchexec-cli` 并使用 `sync-static-dir.sh` 脚本。

```shell
cargo install watchexec-cli
sh sync-static-dir.sh
```

### 样式表开发

项目的样式表使用全部在 `static/css/` 文件夹下，使用 Scss。如果想要监控和编译该文件夹下的 .scss 文件。
可以使用 `compile-scss.sh` 脚本。

```shell
sh compile-scss.sh
```