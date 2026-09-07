# Nova 独游社网站

Nova Indie Game Club 网站源码。使用 [Rust](https://www.rust-lang.org/) 和 [Maud](https://maud.lambda.xyz/) 生成静态网页，作品数据通过本地 TOML 和图片维护。

## 快速构建

安装 [Rust](https://www.rust-lang.org/tools/install) 后，在仓库根目录运行：

```shell
cargo dev
```

生成结果位于 `dist/exported/`。如需本地预览，安装 [miniserve](https://github.com/svenstaro/miniserve) 后运行：

```shell
miniserve dist/exported --index index.html -i 127.0.0.1 -p 8080
```

打开 http://127.0.0.1:8080/。修改内容后重新运行 `cargo dev`，再刷新浏览器。

## 文档

- [开发与构建](docs/development.md)
- [添加作品、重建与发布](docs/works_data.md)
