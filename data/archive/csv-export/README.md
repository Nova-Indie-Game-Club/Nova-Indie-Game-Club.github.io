# 原始 CSV 归档

`works.csv` 与 `works_all.csv` 均包含同样的 7 个作品，只有字段顺序和作品顺序不同。
迁移以 `works.csv` 为准，两份原文件保留在这里用于核对，**不再参与网站构建，也无需同步修改**。

请维护 `data/works/<id>/work.toml` 及同目录图片。迁移规则：

- 从 itch.io 作品地址提取稳定的英文 ID。
- 作者、标签、GameJam、截图的逗号分隔值转换为列表；去掉列表项两侧空白。
- 作者链接与作者直接配对。
- 投稿日期转换为 `YYYY-MM-DD`。
- `NovaGameJam` 的 `None` 转为空列表。
- 封面统一命名为 `cover.png`，截图为 `screenshot-1.png`、`screenshot-2.png`，保留原始图片内容。
- `AutoCollection` 不迁移：网站只使用本地图片，不再自动抓取平台资源。
