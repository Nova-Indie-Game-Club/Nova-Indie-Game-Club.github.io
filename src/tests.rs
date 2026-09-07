use super::*;
use scraper::{Html, Selector};
use website_model::work::*;

fn work(id: &str) -> Work {
    Work {
        id: id.into(),
        name: "测试游戏".into(),
        sub_name: None,
        introduce: "介绍第一段。\n第二段。".into(),
        tags: vec![],
        gamejams: vec![],
        nova_gamejams: vec![],
        platforms: vec![Platform {
            platform_type: PlatformType::Itch,
            url: "https://example.itch.io/game".into(),
        }],
        authors: vec![Author {
            name: "作者".into(),
            url: None,
        }],
        cover: Some(format!("assets/local-works/{id}/cover.png")),
        screenshots: vec![format!("assets/local-works/{id}/screenshot.png")],
        submission_date: DateTimeUtc {
            date_rfc3339: "2024-05-01T00:00:00+00:00".into(),
        },
        class: Class::Spotlight,
    }
}

fn count(document: &Html, selector: &str) -> usize {
    document.select(&Selector::parse(selector).unwrap()).count()
}

#[test]
fn renders_static_content_and_interaction_hooks() {
    let source = templates::index::render(&[work("example")]).into_string();
    let page = Html::parse_document(&source);
    assert!(source.starts_with("<!DOCTYPE html>"));
    assert_eq!(count(&page, "html[lang='zh-CN']"), 1);
    assert_eq!(count(&page, "meta[name='viewport']"), 1);
    assert_eq!(count(&page, "[data-about-tab]"), 3);
    assert_eq!(count(&page, "[data-about-panel]:not([hidden])"), 1);
    assert_eq!(count(&page, "[data-recommendation-panel]:not([hidden])"), 1);
    assert_eq!(count(&page, "[data-open-work]"), 1);
    assert_eq!(count(&page, "dialog"), 1);
    assert_eq!(count(&page, "template#work-example"), 1);
    assert_eq!(count(&page, "script[src='static/js/site.js'][defer]"), 1);
    assert!(!source.contains(".perseus"));
    assert!(!source.contains(".wasm"));
    assert!(source.contains("测试游戏"));
    assert!(source.contains("介绍第一段。\n第二段。"));
}

#[test]
fn caps_recent_works_without_limiting_recommendations() {
    let works: Vec<_> = (0..12).map(|i| work(&format!("game-{i}"))).collect();
    let source = templates::index::render(&works).into_string();
    let page = Html::parse_document(&source);
    assert_eq!(count(&page, "[data-open-work]"), 9);
    assert_eq!(count(&page, "template"), 9);
    assert_eq!(count(&page, "[data-recommend-work]"), 12);
    assert_eq!(count(&page, "[data-recommendation-panel]:not([hidden])"), 1);
}

#[test]
fn handles_no_recommendations_and_missing_optional_images() {
    let mut example = work("example");
    example.class = Class::Normal;
    example.cover = None;
    example.screenshots.clear();
    let source = templates::index::render(&[example]).into_string();
    let page = Html::parse_document(&source);
    assert_eq!(count(&page, "[data-recommend-work]"), 0);
    assert_eq!(count(&page, "img[src='']"), 0);
    assert!(source.contains("推荐作品即将更新。"));
    // Rendering an empty list is also safe, even though the data loader rejects an empty catalog.
    assert!(!templates::index::render(&[]).into_string().is_empty());
}

#[test]
fn escapes_content_in_text_and_attributes() {
    let mut example = work("example");
    example.name = "<script>alert('name')</script>\" onload=\"bad".into();
    example.introduce = "<img src=x onerror=alert(1)> & text".into();
    example.authors[0].name = "<b>not HTML</b>".into();
    let source = templates::index::render(&[example]).into_string();
    let page = Html::parse_document(&source);
    assert_eq!(count(&page, "script"), 1); // The sole script is our external site.js.
    assert_eq!(count(&page, "[onload], [onerror]"), 0);
    assert_eq!(count(&page, "b"), 0);
    assert!(source.contains("&lt;script&gt;"));
    assert!(source.contains("&lt;img"));
    assert!(source.contains("&amp; text"));
}

fn fixture() -> tempfile::TempDir {
    let root = tempfile::tempdir().unwrap();
    let source = Path::new(env!("CARGO_MANIFEST_DIR"));
    // Only web resources actually referenced by the templates; omit design/source archives.
    for path in [
        "css/root.css",
        "css/page/home.css",
        "css/recommended_works.css",
        "css/recent_work_focused.css",
        "css/interactions.css",
        "js/site.js",
        "assets/images/logo_small.png",
        "assets/images/logo_normal.png",
        "assets/images/cover_title.png",
        "assets/images/cover_background.png",
        "assets/images/bg_noise.jpg",
        "assets/images/navi_option_selected.png",
        "assets/images/title_about.png",
        "assets/images/title_recommended_works.png",
        "assets/images/title_recent_works.png",
        "assets/images/title_media.png",
        "assets/images/qr_code/wechat.jpeg",
        "assets/images/qr_code/bilibili.png",
    ] {
        let target = root.path().join("static").join(path);
        fs::create_dir_all(target.parent().unwrap()).unwrap();
        fs::copy(source.join("static").join(path), target).unwrap();
    }
    let folder = root.path().join("data/works/example");
    fs::create_dir_all(&folder).unwrap();
    fs::write(
        folder.join("work.toml"),
        r#"
id = "example"
name = "测试游戏"
introduce = "介绍"
submission_date = "2024-05-01"
authors = [{ name = "作者" }]
class = "Spotlight"
cover = "cover.png"
screenshots = ["screenshot.png"]
"#,
    )
    .unwrap();
    fs::write(folder.join("cover.png"), b"cover bytes").unwrap();
    fs::write(folder.join("screenshot.png"), b"screenshot bytes").unwrap();
    root
}

fn assert_local_references_exist(file: &Path) {
    let source = fs::read_to_string(file).unwrap();
    let page = Html::parse_document(&source);
    for node in page.select(&Selector::parse("[src], link[href]").unwrap()) {
        let value = node
            .value()
            .attr("src")
            .or(node.value().attr("href"))
            .unwrap();
        if !value.starts_with("https://") {
            assert!(
                file.parent().unwrap().join(value).is_file(),
                "{}: missing {value}",
                file.display()
            );
        }
    }
}

fn snapshot(folder: &Path) -> std::collections::BTreeMap<PathBuf, Vec<u8>> {
    fn walk(
        root: &Path,
        current: &Path,
        output: &mut std::collections::BTreeMap<PathBuf, Vec<u8>>,
    ) {
        for entry in fs::read_dir(current).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(root, &path, output);
            } else {
                output.insert(
                    path.strip_prefix(root).unwrap().into(),
                    fs::read(path).unwrap(),
                );
            }
        }
    }
    let mut result = std::collections::BTreeMap::new();
    walk(folder, folder, &mut result);
    result
}

#[test]
fn generates_complete_repeatable_export_and_removes_obsolete_files() {
    let root = fixture();
    for path in [
        "dist/exported/.perseus",
        "static/assets/local-works",
        "static/assets/works",
    ] {
        fs::create_dir_all(root.path().join(path)).unwrap();
        fs::write(root.path().join(path).join("stale.txt"), b"stale").unwrap();
    }
    let summary = generate(root.path()).unwrap();
    assert_eq!(summary.work_count, 1);
    assert_local_references_exist(&summary.output.join("index.html"));
    assert_local_references_exist(&summary.output.join("about/index.html"));
    for stylesheet in ["page/home.css", "recent_work_focused.css"] {
        let file = summary.output.join("static/css").join(stylesheet);
        let css = fs::read_to_string(&file).unwrap();
        for url in css.split("url(").skip(1) {
            let url = url
                .split(')')
                .next()
                .unwrap()
                .trim()
                .trim_matches(['\"', '\'']);
            assert!(
                file.parent().unwrap().join(url).is_file(),
                "{}: missing {url}",
                file.display()
            );
        }
    }
    assert!(!summary.output.join(".perseus").exists());
    assert!(!summary.output.join("static/assets/works").exists());
    assert!(!summary
        .output
        .join("static/assets/local-works/stale.txt")
        .exists());
    assert_eq!(
        fs::read(
            summary
                .output
                .join("static/assets/local-works/example/cover.png")
        )
        .unwrap(),
        b"cover bytes"
    );
    assert_eq!(
        fs::read(
            summary
                .output
                .join("static/assets/local-works/example/screenshot.png")
        )
        .unwrap(),
        b"screenshot bytes"
    );
    // A build must not overwrite the source tree or mutate a legacy cache.
    assert_eq!(
        fs::read(root.path().join("static/assets/local-works/stale.txt")).unwrap(),
        b"stale"
    );
    let first = snapshot(&summary.output);
    generate(root.path()).unwrap();
    assert_eq!(snapshot(&summary.output), first);
}

#[test]
fn invalid_data_preserves_the_previous_export() {
    let root = fixture();
    let first = generate(root.path()).unwrap();
    let previous = snapshot(&first.output);
    fs::write(
        root.path().join("data/works/example/work.toml"),
        "invalid TOML",
    )
    .unwrap();
    assert!(generate(root.path()).is_err());
    assert_eq!(snapshot(&first.output), previous);
    assert_eq!(fs::read_dir(root.path().join("dist")).unwrap().count(), 1);
}
