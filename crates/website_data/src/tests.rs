use super::*;
use tempfile::TempDir;

fn minimal(id: &str, date: &str) -> String {
    format!(
        r#"id = "{id}"
name = "测试游戏"
introduce = """
第一段。
第二段。"""
submission_date = "{date}"
authors = [{{ name = "作者" }}]
"#
    )
}

fn write_work(root: &Path, directory: &str, text: &str) -> PathBuf {
    let folder = root.join(directory);
    fs::create_dir_all(&folder).unwrap();
    let file = folder.join("work.toml");
    fs::write(&file, text).unwrap();
    file
}

fn failure(root: &Path) -> String {
    match read_works(root) {
        Ok(_) => panic!("expected invalid catalog to fail"),
        Err(error) => format!("{error:#}"),
    }
}

#[test]
fn reads_human_friendly_fields_and_defaults() {
    let root = TempDir::new().unwrap();
    let text = minimal("example", "2024-05-01").replace(
        "authors = [{ name = \"作者\" }]",
        "authors = [{ name = \"作者\", url = \"https://example.com/\" }, { name = \"搭档\" }]",
    );
    write_work(root.path(), "example", &format!("{text}\ntags = [\"解谜\", \"鸭子\"]\ngamejams = [\"Bevy Jam\"]\n[platforms]\nitch = \"https://example.itch.io/game\"\nhomepage = \"https://example.com/\"\n"));
    let works = read_works(root.path()).unwrap();
    let work = &works[0];
    assert_eq!(works.len(), 1);
    assert_eq!(work.name, "测试游戏");
    assert_eq!(work.introduce, "第一段。\n第二段。");
    assert_eq!(work.plain_submission_date_day(), "2024-05-01");
    assert_eq!(work.plain_author_string(), "作者, 搭档");
    assert_eq!(work.authors[0].url.as_deref(), Some("https://example.com/"));
    assert!(work.authors[1].url.is_none());
    assert_eq!(work.tags[1].name, "鸭子");
    assert_eq!(work.gamejams[0].name, "Bevy Jam");
    assert_eq!(work.platforms.len(), 2);
    assert!(work.platforms[1].platform_type == PlatformType::HomePage);
    assert!(work.class == Class::Normal);
    assert!(work.sub_name.is_none());
    assert!(work.cover.is_none());
    assert!(work.screenshots.is_empty());
    assert!(work.nova_gamejams.is_empty());
}

#[test]
fn sorts_newest_first_with_id_tie_breaker() {
    let root = TempDir::new().unwrap();
    for (id, date) in [
        ("old", "2024-02-29"),
        ("z-new", "2024-05-01"),
        ("a-new", "2024-05-01"),
    ] {
        write_work(root.path(), id, &minimal(id, date));
    }
    let works = read_works(root.path()).unwrap();
    assert_eq!(
        works
            .iter()
            .map(|work| work.id.as_str())
            .collect::<Vec<_>>(),
        ["a-new", "z-new", "old"]
    );
}

#[test]
fn rejects_duplicate_ids_with_file_context() {
    let root = TempDir::new().unwrap();
    write_work(root.path(), "a", &minimal("duplicate", "2024-05-01"));
    let file = write_work(root.path(), "b", &minimal("duplicate", "2024-05-01"));
    let error = failure(root.path());
    assert!(error.contains("duplicate id duplicate"), "{error}");
    assert!(error.contains(&file.display().to_string()), "{error}");
}

#[test]
fn rejects_invalid_fields_with_file_context() {
    let base = minimal("example", "2024-05-01");
    for (text, expected) in [
        (base.replace("2024-05-01", "2024-02-30"), "submission_date"),
        (base.replace("2024-05-01", "2024-5-1"), "submission_date"),
        (
            base.replace("id = \"example\"", "id = \"../escape\""),
            "id:",
        ),
        (base.replace("测试游戏", "  "), "name:"),
        (
            base.replace("authors = [{ name = \"作者\" }]", "authors = []"),
            "authors:",
        ),
        (format!("{base}\nclass = \"Typo\""), "Typo"),
        (format!("{base}\ntga = []"), "unknown field"),
        (
            format!("{base}\n[platforms]\nitch = \"javascript:alert(1)\""),
            "platforms.itch",
        ),
        (
            base.replace(
                "{ name = \"作者\" }",
                "{ name = \"作者\", url = \"bad-link\" }",
            ),
            "authors[0].url",
        ),
        (format!("{base}\ntags = [\"\"]"), "tags:"),
        (format!("{base}\ncover = \"missing.png\""), "cover:"),
        (
            format!("{base}\nscreenshots = [\"missing.png\"]"),
            "screenshots[0]:",
        ),
        ("not valid TOML".to_string(), "invalid work TOML"),
    ] {
        let root = TempDir::new().unwrap();
        let file = write_work(root.path(), "example", &text);
        let error = failure(root.path());
        assert!(error.contains(expected), "expected {expected}: {error}");
        assert!(error.contains(&file.display().to_string()), "{error}");
    }
}

#[test]
fn rejects_escaping_asset_paths() {
    for value in [
        "../outside.png",
        "/outside.png",
        "C:/outside.png",
        "image.png?x=1",
        "image.png#fragment",
    ] {
        let root = TempDir::new().unwrap();
        write_work(
            root.path(),
            "example",
            &format!("{}\ncover = \"{value}\"", minimal("example", "2024-05-01")),
        );
        let error = failure(root.path());
        assert!(
            error.contains("cover: expected a relative file path"),
            "{error}"
        );
    }
}

#[test]
fn prepares_assets_and_removes_stale_generated_files() {
    let root = TempDir::new().unwrap();
    let works_dir = root.path().join("works");
    let static_dir = root.path().join("static");
    write_work(
        &works_dir,
        "example",
        &format!(
            "{}\ncover = \"cover.png\"\nscreenshots = [\"shots/one.gif\"]",
            minimal("example", "2024-05-01")
        ),
    );
    fs::create_dir_all(works_dir.join("example/shots")).unwrap();
    fs::write(works_dir.join("example/cover.png"), b"cover bytes").unwrap();
    fs::write(works_dir.join("example/shots/one.gif"), b"screenshot bytes").unwrap();
    fs::create_dir_all(static_dir.join(ASSET_PATH)).unwrap();
    fs::write(static_dir.join(ASSET_PATH).join("stale.png"), b"old").unwrap();
    fs::write(static_dir.join("manual.txt"), b"keep").unwrap();

    let works = prepare_works(&works_dir, &static_dir).unwrap();
    let cover = works[0].cover.as_ref().unwrap();
    assert_eq!(cover, "assets/local-works/example/cover.png");
    assert_eq!(fs::read(static_dir.join(cover)).unwrap(), b"cover bytes");
    assert_eq!(
        fs::read(static_dir.join(&works[0].screenshots[0])).unwrap(),
        b"screenshot bytes"
    );
    assert!(!static_dir.join(ASSET_PATH).join("stale.png").exists());
    assert_eq!(fs::read(static_dir.join("manual.txt")).unwrap(), b"keep");
    // A second build is safe and repeatable.
    prepare_works(&works_dir, &static_dir).unwrap();
    assert_eq!(fs::read(static_dir.join(cover)).unwrap(), b"cover bytes");
}

#[test]
fn validates_before_touching_existing_assets() {
    let root = TempDir::new().unwrap();
    let works_dir = root.path().join("works");
    let static_dir = root.path().join("static");
    write_work(
        &works_dir,
        "example",
        &format!(
            "{}\ncover = \"missing.png\"",
            minimal("example", "2024-05-01")
        ),
    );
    fs::create_dir_all(static_dir.join(ASSET_PATH)).unwrap();
    let existing = static_dir.join(ASSET_PATH).join("existing.png");
    fs::write(&existing, b"keep").unwrap();
    assert!(prepare_works(&works_dir, &static_dir).is_err());
    assert_eq!(fs::read(existing).unwrap(), b"keep");
}

#[test]
fn missing_empty_or_incomplete_catalog_is_an_error() {
    let root = TempDir::new().unwrap();
    assert!(failure(&root.path().join("missing")).contains("cannot read works directory"));
    assert!(failure(root.path()).contains("no work.toml files found"));
    fs::create_dir(root.path().join("unfinished")).unwrap();
    assert!(failure(root.path()).contains("missing work.toml"));
}

#[test]
fn repository_catalog_is_valid() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/works");
    let works = read_works(root).unwrap();
    assert!(!works.is_empty());
}
