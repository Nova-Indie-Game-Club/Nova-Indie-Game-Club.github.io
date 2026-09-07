use anyhow::{ensure, Context, Result};
use chrono::NaiveDate;
use serde::Deserialize;
use std::{
    collections::HashSet,
    fs,
    path::{Component, Path, PathBuf},
};
use website_model::work::*;

/// Generated assets live separately from manually maintained static files.
const ASSET_PATH: &str = "assets/local-works";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LocalWork {
    id: String,
    name: String,
    sub_name: Option<String>,
    introduce: String,
    submission_date: String,
    #[serde(default = "normal_class")]
    class: Class,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    gamejams: Vec<String>,
    #[serde(default)]
    nova_gamejams: Vec<String>,
    authors: Vec<LocalAuthor>,
    cover: Option<String>,
    #[serde(default)]
    screenshots: Vec<String>,
    #[serde(default)]
    platforms: LocalPlatforms,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LocalAuthor {
    name: String,
    url: Option<String>,
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct LocalPlatforms {
    itch: Option<String>,
    steam: Option<String>,
    gamecore: Option<String>,
    homepage: Option<String>,
}

struct LoadedWork {
    work: Work,
    // Source file and destination relative to ASSET_PATH.
    assets: Vec<(PathBuf, PathBuf)>,
}

fn normal_class() -> Class {
    Class::Normal
}

fn required_text(value: &str, field: &str) -> Result<()> {
    ensure!(!value.trim().is_empty(), "{field}: must not be empty");
    Ok(())
}

fn validate_url(value: &str, field: &str) -> Result<()> {
    let url = url::Url::parse(value).with_context(|| format!("{field}: invalid URL"))?;
    ensure!(
        matches!(url.scheme(), "http" | "https") && url.host_str().is_some(),
        "{field}: expected an absolute HTTP(S) URL"
    );
    Ok(())
}

fn selected_values(values: Vec<String>, field: &str) -> Result<Vec<SelectedValue>> {
    values
        .into_iter()
        .map(|name| {
            required_text(&name, field)?;
            Ok(SelectedValue { name })
        })
        .collect()
}

fn local_asset(
    folder: &Path,
    id: &str,
    value: &str,
    field: &str,
    assets: &mut Vec<(PathBuf, PathBuf)>,
) -> Result<String> {
    let relative = Path::new(value);
    ensure!(
        !value.is_empty()
            && !value.contains(['\\', ':', '?', '#'])
            && relative
                .components()
                .all(|part| matches!(part, Component::Normal(_))),
        "{field}: expected a relative file path inside the work directory: {value}"
    );
    let source = folder.join(relative);
    let resolved = source
        .canonicalize()
        .with_context(|| format!("{field}: cannot access {}", source.display()))?;
    ensure!(
        resolved.starts_with(folder.canonicalize()?) && resolved.is_file(),
        "{field}: must reference a file inside the work directory: {value}"
    );
    assets.push((resolved, Path::new(id).join(relative)));
    Ok(format!("{ASSET_PATH}/{id}/{value}"))
}

fn load_work(file: &Path) -> Result<LoadedWork> {
    let text = fs::read_to_string(file).context("cannot read TOML")?;
    let local: LocalWork = toml::from_str(&text).context("invalid work TOML")?;
    ensure!(
        !local.id.is_empty()
            && local
                .id
                .bytes()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
            && !local.id.starts_with('-')
            && !local.id.ends_with('-'),
        "id: use lowercase ASCII letters, digits and interior hyphens"
    );
    required_text(&local.name, "name")?;
    required_text(&local.introduce, "introduce")?;
    let date = NaiveDate::parse_from_str(&local.submission_date, "%Y-%m-%d")
        .context("submission_date: expected a valid YYYY-MM-DD date")?;
    ensure!(
        date.format("%Y-%m-%d").to_string() == local.submission_date,
        "submission_date: expected YYYY-MM-DD"
    );
    ensure!(
        !local.authors.is_empty(),
        "authors: at least one author is required"
    );
    let authors = local
        .authors
        .into_iter()
        .enumerate()
        .map(|(index, author)| {
            required_text(&author.name, &format!("authors[{index}].name"))?;
            if let Some(url) = &author.url {
                validate_url(url, &format!("authors[{index}].url"))?;
            }
            Ok(Author {
                name: author.name,
                url: author.url,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let mut platforms = Vec::new();
    for (value, platform_type, field) in [
        (local.platforms.itch, PlatformType::Itch, "platforms.itch"),
        (
            local.platforms.steam,
            PlatformType::Steam,
            "platforms.steam",
        ),
        (
            local.platforms.gamecore,
            PlatformType::GameCore,
            "platforms.gamecore",
        ),
        (
            local.platforms.homepage,
            PlatformType::HomePage,
            "platforms.homepage",
        ),
    ] {
        if let Some(url) = value {
            validate_url(&url, field)?;
            platforms.push(Platform { platform_type, url });
        }
    }
    let folder = file.parent().context("missing work directory")?;
    let mut assets = Vec::new();
    let cover = local
        .cover
        .map(|value| local_asset(folder, &local.id, &value, "cover", &mut assets))
        .transpose()?;
    let screenshots = local
        .screenshots
        .into_iter()
        .enumerate()
        .map(|(index, value)| {
            local_asset(
                folder,
                &local.id,
                &value,
                &format!("screenshots[{index}]"),
                &mut assets,
            )
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(LoadedWork {
        work: Work {
            id: local.id,
            name: local.name,
            sub_name: local.sub_name,
            introduce: local.introduce,
            submission_date: DateTimeUtc {
                date_rfc3339: format!("{}T00:00:00+00:00", local.submission_date),
            },
            class: local.class,
            tags: selected_values(local.tags, "tags")?,
            gamejams: selected_values(local.gamejams, "gamejams")?,
            nova_gamejams: selected_values(local.nova_gamejams, "nova_gamejams")?,
            authors,
            platforms,
            cover,
            screenshots,
        },
        assets,
    })
}

fn load_works(folder: &Path) -> Result<Vec<LoadedWork>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(folder)
        .with_context(|| format!("cannot read works directory {}", folder.display()))?
    {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let file = entry.path().join("work.toml");
            ensure!(
                file.is_file(),
                "{}: missing work.toml",
                entry.path().display()
            );
            files.push(file);
        }
    }
    files.sort();
    ensure!(
        !files.is_empty(),
        "{}: no work.toml files found",
        folder.display()
    );
    let mut ids = HashSet::new();
    let mut works = Vec::new();
    for file in files {
        let loaded = load_work(&file).with_context(|| file.display().to_string())?;
        ensure!(
            ids.insert(loaded.work.id.clone()),
            "{}: duplicate id {}",
            file.display(),
            loaded.work.id
        );
        works.push(loaded);
    }
    // Newest submissions first; stable ordering for works submitted on the same day.
    works.sort_by(|a, b| {
        b.work
            .submission_date
            .date_rfc3339
            .cmp(&a.work.submission_date.date_rfc3339)
            .then_with(|| a.work.id.cmp(&b.work.id))
    });
    Ok(works)
}

/// Read and validate the local catalog without modifying static assets.
pub fn read_works(folder: impl AsRef<Path>) -> Result<Vec<Work>> {
    Ok(load_works(folder.as_ref())?
        .into_iter()
        .map(|loaded| loaded.work)
        .collect())
}

/// Validate the entire catalog, then refresh its generated static assets.
/// `static_root/assets/local-works` is exclusively owned by this function.
pub fn prepare_works(folder: impl AsRef<Path>, static_root: impl AsRef<Path>) -> Result<Vec<Work>> {
    let loaded = load_works(folder.as_ref())?;
    let destination = static_root.as_ref().join(ASSET_PATH);
    if destination.exists() {
        fs::remove_dir_all(&destination)
            .with_context(|| format!("cannot clear {}", destination.display()))?;
    }
    fs::create_dir_all(&destination)?;
    for item in &loaded {
        for (source, relative) in &item.assets {
            let target = destination.join(relative);
            fs::create_dir_all(target.parent().context("missing asset directory")?)?;
            fs::copy(source, &target).with_context(|| {
                format!("cannot copy {} to {}", source.display(), target.display())
            })?;
        }
    }
    Ok(loaded.into_iter().map(|item| item.work).collect())
}

#[cfg(test)]
mod tests;
